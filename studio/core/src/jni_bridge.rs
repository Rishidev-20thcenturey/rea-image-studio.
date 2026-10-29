//! The JNI surface the Kotlin UI calls. Compiled only for Android.
//!
//! Documents live in a process-wide map keyed by handle, because a Kotlin `Long` cannot hold a
//! pointer safely across a garbage-collected heap. Every entry point locks that map, so the rules
//! are simple: handles are opaque, `0` means "failed", and the last failure's message is available
//! from `lastError` for the UI to show.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

use jni::objects::{JClass, JFloatArray, JIntArray, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jstring};
use jni::JNIEnv;

use crate::blend::BlendMode;
use crate::doc::Document;

static DOCS: OnceLock<Mutex<HashMap<u64, Document>>> = OnceLock::new();
static NEXT_HANDLE: OnceLock<Mutex<u64>> = OnceLock::new();
static LAST_ERROR: OnceLock<Mutex<String>> = OnceLock::new();

fn docs() -> MutexGuard<'static, HashMap<u64, Document>> {
    DOCS.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn next_handle() -> u64 {
    let m = NEXT_HANDLE.get_or_init(|| Mutex::new(1));
    let mut g = m.lock().unwrap_or_else(|e| e.into_inner());
    let h = *g;
    *g += 1;
    h
}

fn set_error(msg: impl Into<String>) {
    let m = LAST_ERROR.get_or_init(|| Mutex::new(String::new()));
    if let Ok(mut g) = m.lock() {
        *g = msg.into();
    }
}

#[inline]
fn argb(v: jint) -> [u8; 4] {
    [
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8) & 0xFF) as u8,
        (v & 0xFF) as u8,
        ((v >> 24) & 0xFF) as u8,
    ]
}

fn new_string(env: &mut JNIEnv, s: String) -> jstring {
    match env.new_string(s) {
        Ok(js) => js.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

fn read_string(env: &mut JNIEnv, s: &JString) -> String {
    env.get_string(s)
        .map(|v| v.into())
        .unwrap_or_else(|_| String::new())
}

fn write_ints(env: &mut JNIEnv, arr: &JIntArray, data: &[jint]) {
    if !data.is_empty() {
        let _ = env.set_int_array_region(arr, 0, data);
    }
}

/// ARGB pixels are u32 in the core and jint (i32) on the Java side; convert on the way out.
fn write_pixels(env: &mut JNIEnv, arr: &JIntArray, data: &[u32]) {
    if data.is_empty() {
        return;
    }
    let converted: Vec<jint> = data.iter().map(|v| *v as jint).collect();
    let _ = env.set_int_array_region(arr, 0, &converted);
}

fn read_floats(env: &mut JNIEnv, arr: &JFloatArray) -> Vec<f32> {
    let len = env.get_array_length(arr).unwrap_or(0).max(0) as usize;
    let mut buf = vec![0f32; len];
    if len > 0 {
        let _ = env.get_float_array_region(arr, 0, &mut buf);
    }
    buf
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_nativeVersion(mut env: JNIEnv, _c: JClass) -> jstring {
    new_string(&mut env, crate::about())
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_lastError(mut env: JNIEnv, _c: JClass) -> jstring {
    let m = LAST_ERROR.get_or_init(|| Mutex::new(String::new()));
    let msg = m.lock().map(|g| g.clone()).unwrap_or_default();
    new_string(&mut env, msg)
}

// --- documents -------------------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_createDocument(_env: JNIEnv, _c: JClass, w: jint, h: jint) -> jlong {
    let doc = Document::new(w, h);
    let handle = next_handle();
    docs().insert(handle, doc);
    handle as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_openDocument(mut env: JNIEnv, _c: JClass, path: JString) -> jlong {
    let p = read_string(&mut env, &path);
    match Document::load(&p) {
        Ok(doc) => {
            let handle = next_handle();
            docs().insert(handle, doc);
            handle as jlong
        }
        Err(e) => {
            set_error(format!("could not open {}: {}", p, e));
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_saveDocument(mut env: JNIEnv, _c: JClass, handle: jlong, path: JString) -> jboolean {
    let p = read_string(&mut env, &path);
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => match doc.save(&p) {
            Ok(()) => 1,
            Err(e) => {
                set_error(format!("could not save {}: {}", p, e));
                0
            }
        },
        None => {
            set_error("invalid document handle");
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_closeDocument(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let removed = docs().remove(&(handle as u64)).is_some();
    if removed {
        1
    } else {
        0
    }
}

/// "width,height"
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_documentSize(mut env: JNIEnv, _c: JClass, handle: jlong) -> jstring {
    let m = docs();
    let s = m
        .get(&(handle as u64))
        .map(|d| format!("{},{}", d.w, d.h))
        .unwrap_or_else(|| "0,0".to_string());
    drop(m);
    new_string(&mut env, s)
}

/// "x,y,w,h" of the selected area; the whole canvas when nothing is selected.
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectionBounds(mut env: JNIEnv, _c: JClass, handle: jlong) -> jstring {
    let m = docs();
    let s = m
        .get(&(handle as u64))
        .map(|d| {
            let r = d.selection_bounds();
            format!("{},{},{},{}", r.x0, r.y0, r.width(), r.height())
        })
        .unwrap_or_else(|| "0,0,0,0".to_string());
    drop(m);
    new_string(&mut env, s)
}

// --- rendering -------------------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_render(
    mut env: JNIEnv,
    _c: JClass,
    handle: jlong,
    out: JIntArray,
    outw: jint,
    outh: jint,
    vx: jfloat,
    vy: jfloat,
    scale: jfloat,
) -> jboolean {
    render_impl(&mut env, handle, &out, outw, outh, vx, vy, scale, false)
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_renderSelection(
    mut env: JNIEnv,
    _c: JClass,
    handle: jlong,
    out: JIntArray,
    outw: jint,
    outh: jint,
    vx: jfloat,
    vy: jfloat,
    scale: jfloat,
) -> jboolean {
    render_impl(&mut env, handle, &out, outw, outh, vx, vy, scale, true)
}


#[allow(clippy::too_many_arguments)]
fn render_impl(
    env: &mut JNIEnv,
    handle: jlong,
    out: &JIntArray,
    outw: jint,
    outh: jint,
    vx: f32,
    vy: f32,
    scale: f32,
    selection: bool,
) -> jboolean {
    if outw <= 0 || outh <= 0 {
        return 0;
    }
    let mut buf = vec![0u32; (outw as usize) * (outh as usize)];
    {
        let mut m = docs();
        match m.get_mut(&(handle as u64)) {
            Some(doc) => {
                if selection {
                    doc.render_selection(&mut buf, outw, outh, vx, vy, scale);
                } else {
                    doc.render_viewport(&mut buf, outw, outh, vx, vy, scale, 0xFF32_3232, 0xFF28_2828);
                }
            }
            None => {
                set_error("invalid document handle");
                return 0;
            }
        }
    }
    write_pixels(env, out, &buf);
    1
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_layerThumb(
    mut env: JNIEnv,
    _c: JClass,
    handle: jlong,
    index: jint,
    out: JIntArray,
    outw: jint,
    outh: jint,
) -> jboolean {
    if outw <= 0 || outh <= 0 {
        return 0;
    }
    let mut buf = vec![0u32; (outw as usize) * (outh as usize)];
    {
        let m = docs();
        match m.get(&(handle as u64)) {
            Some(doc) => doc.layer_thumb(index.max(0) as usize, &mut buf, outw, outh),
            None => return 0,
        }
    }
    write_pixels(&mut env, &out, &buf);
    1
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_exportPixels(
    mut env: JNIEnv,
    _c: JClass,
    handle: jlong,
    out: JIntArray,
) -> jboolean {
    let flat = {
        let mut m = docs();
        match m.get_mut(&(handle as u64)) {
            Some(doc) => doc.flatten_argb(),
            None => {
                set_error("invalid document handle");
                return 0;
            }
        }
    };
    let data: Vec<jint> = flat.iter().map(|v| *v as jint).collect();
    write_ints(&mut env, &out, &data);
    1
}

// --- layers ----------------------------------------------------------------------------------

/// One line per layer, top of the stack first: `index|id|name|visible|opacity|blend|hasMask`.
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_layers(mut env: JNIEnv, _c: JClass, handle: jlong) -> jstring {
    let m = docs();
    let s = match m.get(&(handle as u64)) {
        Some(doc) => {
            let mut out = String::new();
            for (i, l) in doc.layers.iter().enumerate() {
                // Newline-free field, so the line format survives a name with a pipe in it.
                let name = l.name.replace('|', "/").replace('\n', " ");
                out.push_str(&format!(
                    "{}|{}|{}|{}|{}|{}|{}\n",
                    i,
                    l.id,
                    name,
                    if l.visible { 1 } else { 0 },
                    l.opacity,
                    l.blend as u8,
                    if l.mask.is_some() { 1 } else { 0 }
                ));
            }
            out
        }
        None => String::new(),
    };
    drop(m);
    new_string(&mut env, s)
}

/// The active layer index, which the panel highlights.
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_activeLayer(_env: JNIEnv, _c: JClass, handle: jlong) -> jint {
    docs().get(&(handle as u64)).map(|d| d.active as jint).unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectLayer(_env: JNIEnv, _c: JClass, handle: jlong, index: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            let i = index.max(0) as usize;
            if i < doc.layers.len() {
                doc.active = i;
                1
            } else {
                0
            }
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_addLayer(mut env: JNIEnv, _c: JClass, handle: jlong, name: JString) -> jboolean {
    let n = read_string(&mut env, &name);
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.add_layer(&n);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_addLayerFromPixels(
    mut env: JNIEnv,
    _c: JClass,
    handle: jlong,
    name: JString,
    w: jint,
    h: jint,
    pixels: JIntArray,
) -> jboolean {
    if w <= 0 || h <= 0 {
        return 0;
    }
    let n = read_string(&mut env, &name);
    let count = (w as usize) * (h as usize);
    let mut buf = vec![0i32; count];
    if env.get_int_array_region(&pixels, 0, &mut buf).is_err() {
        set_error("could not read the pixel array");
        return 0;
    }
    // ARGB ints -> straight RGBA bytes.
    let mut rgba = Vec::with_capacity(count * 4);
    for v in buf {
        let c = argb(v);
        rgba.extend_from_slice(&c);
    }
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.add_layer_from_pixels(&n, w, h, rgba);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_deleteLayer(_env: JNIEnv, _c: JClass, handle: jlong, index: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.delete_layer(index.max(0) as usize) as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_duplicateLayer(_env: JNIEnv, _c: JClass, handle: jlong, index: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.duplicate_layer(index.max(0) as usize) as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_mergeDown(_env: JNIEnv, _c: JClass, handle: jlong, index: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.merge_down(index.max(0) as usize) as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_moveLayer(_env: JNIEnv, _c: JClass, handle: jlong, from: jint, to: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.move_layer(from.max(0) as usize, to.max(0) as usize) as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_setLayerVisible(_env: JNIEnv, _c: JClass, handle: jlong, index: jint, visible: jboolean) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.set_visible(index.max(0) as usize, visible != 0);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_setLayerOpacity(_env: JNIEnv, _c: JClass, handle: jlong, index: jint, opacity: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.set_opacity(index.max(0) as usize, opacity.clamp(0, 255) as u8);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_setLayerBlend(_env: JNIEnv, _c: JClass, handle: jlong, index: jint, mode: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.set_blend(index.max(0) as usize, BlendMode::from_u8(mode.clamp(0, 15) as u8));
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_setLayerName(mut env: JNIEnv, _c: JClass, handle: jlong, index: jint, name: JString) -> jboolean {
    let n = read_string(&mut env, &name);
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.set_layer_name(index.max(0) as usize, &n);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_addLayerMask(_env: JNIEnv, _c: JClass, handle: jlong, index: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.add_mask_to_layer(index.max(0) as usize) as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectionToMask(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.selection_to_mask();
            1
        }
        None => 0,
    }
}

/// The 16 blend-mode labels, in the order the numeric encoding uses.
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_blendModes(mut env: JNIEnv, _c: JClass) -> jstring {
    let labels: Vec<&str> = BlendMode::all().iter().map(|m| m.label()).collect();
    new_string(&mut env, labels.join("|"))
}

// --- painting --------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_strokeBegin(
    _env: JNIEnv,
    _c: JClass,
    handle: jlong,
    x: jfloat,
    y: jfloat,
    pressure: jfloat,
    tool: jint,
    color: jint,
    size: jfloat,
    hardness: jfloat,
    flow: jfloat,
    opacity: jfloat,
) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.stroke_begin(x, y, pressure, tool.clamp(0, 3) as u8, argb(color), size, hardness, flow, opacity);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_strokeTo(
    _env: JNIEnv,
    _c: JClass,
    handle: jlong,
    x: jfloat,
    y: jfloat,
    pressure: jfloat,
) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.stroke_to(x, y, pressure);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_strokeEnd(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.stroke_end();
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_bucketFill(_env: JNIEnv, _c: JClass, handle: jlong, x: jint, y: jint, color: jint, tolerance: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.bucket_fill(x, y, argb(color), tolerance.clamp(0, 255) as u8);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_fillSelection(_env: JNIEnv, _c: JClass, handle: jlong, color: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.fill_selection(argb(color));
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_clearSelection(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.clear_selection();
            1
        }
        None => 0,
    }
}

// --- selection -------------------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectRect(_env: JNIEnv, _c: JClass, handle: jlong, x0: jfloat, y0: jfloat, x1: jfloat, y1: jfloat, feather: jfloat) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.select_rect(x0, y0, x1, y1, feather);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectEllipse(_env: JNIEnv, _c: JClass, handle: jlong, x0: jfloat, y0: jfloat, x1: jfloat, y1: jfloat, feather: jfloat) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.select_ellipse(x0, y0, x1, y1, feather);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectPolygon(mut env: JNIEnv, _c: JClass, handle: jlong, points: JFloatArray, feather: jfloat) -> jboolean {
    let pts = read_floats(&mut env, &points);
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.select_polygon(&pts, feather);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectAll(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.select_all();
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_selectNone(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.select_none();
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_invertSelection(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.invert_selection();
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_featherSelection(_env: JNIEnv, _c: JClass, handle: jlong, radius: jfloat) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.feather_selection(0.0, radius);
            1
        }
        None => 0,
    }
}

// --- adjustments and filters -----------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_adjust(mut env: JNIEnv, _c: JClass, handle: jlong, kind: jint, params: JFloatArray) -> jboolean {
    let p = read_floats(&mut env, &params);
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.adjust(kind.clamp(0, 8) as u8, &p);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_filter(mut env: JNIEnv, _c: JClass, handle: jlong, kind: jint, params: JFloatArray) -> jboolean {
    let p = read_floats(&mut env, &params);
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.filter(kind.clamp(0, 3) as u8, &p);
            1
        }
        None => 0,
    }
}

// --- transforms and canvas -------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_flip(_env: JNIEnv, _c: JClass, handle: jlong, horizontal: jboolean) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.flip(horizontal != 0);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_rotate90(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.rotate90();
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_nudgeLayer(_env: JNIEnv, _c: JClass, handle: jlong, dx: jint, dy: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.nudge_layer(dx, dy);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_cropToSelection(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            let r = doc.selection_bounds();
            if r.is_empty() {
                return 0;
            }
            doc.crop(r);
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_resizeCanvas(_env: JNIEnv, _c: JClass, handle: jlong, w: jint, h: jint) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => {
            doc.resize_canvas(w, h);
            1
        }
        None => 0,
    }
}

// --- history ---------------------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_undo(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.undo() as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_redo(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    let mut m = docs();
    match m.get_mut(&(handle as u64)) {
        Some(doc) => doc.redo() as jboolean,
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_canUndo(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    docs().get(&(handle as u64)).map(|d| d.can_undo() as jboolean).unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_canRedo(_env: JNIEnv, _c: JClass, handle: jlong) -> jboolean {
    docs().get(&(handle as u64)).map(|d| d.can_redo() as jboolean).unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_undoName(mut env: JNIEnv, _c: JClass, handle: jlong) -> jstring {
    let m = docs();
    let name = m.get(&(handle as u64)).map(|d| d.undo_name().to_string()).unwrap_or_default();
    drop(m);
    new_string(&mut env, name)
}

#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_redoName(mut env: JNIEnv, _c: JClass, handle: jlong) -> jstring {
    let m = docs();
    let name = m.get(&(handle as u64)).map(|d| d.redo_name().to_string()).unwrap_or_default();
    drop(m);
    new_string(&mut env, name)
}

/// The undo stack, newest last, one name per line — the History panel.
#[no_mangle]
pub extern "system" fn Java_com_rea_studio_NativeCore_history(mut env: JNIEnv, _c: JClass, handle: jlong) -> jstring {
    let m = docs();
    let s = match m.get(&(handle as u64)) {
        Some(doc) => doc.undo.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join("\n"),
        None => String::new(),
    };
    drop(m);
    new_string(&mut env, s)
}
