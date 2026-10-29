//! The document model: layers, an incremental composite cache, history, and the `.rea` file format.
//!
//! Pixels are straight RGBA8, row-major, in a layer-local buffer. A layer sits on the canvas at
//! `offset`. Layer buffers are behind `Arc` so a history snapshot is a cheap pointer copy; the
//! first edit after a snapshot copies only the layer it touches (`Arc::make_mut`), which keeps undo
//! memory proportional to what actually changed instead of to the document size.

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;

use crate::blend::{composite_px, BlendMode};

/// How many undo steps are kept. Each one is cheap until the layer is touched again.
pub const HISTORY_LIMIT: usize = 40;

/// The file format's magic line. Bumping it invalidates older files on purpose.
const MAGIC: &[u8] = b"REASTUDIO1\n";

/// An integer rectangle in document space; `x1` and `y1` are exclusive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Rect {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Rect {
        Rect { x0, y0, x1, y1 }
    }

    pub fn empty() -> Rect {
        Rect { x0: 0, y0: 0, x1: 0, y1: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.x1 <= self.x0 || self.y1 <= self.y0
    }

    pub fn width(&self) -> i32 {
        (self.x1 - self.x0).max(0)
    }

    pub fn height(&self) -> i32 {
        (self.y1 - self.y0).max(0)
    }

    pub fn intersect(&self, o: Rect) -> Rect {
        Rect {
            x0: self.x0.max(o.x0),
            y0: self.y0.max(o.y0),
            x1: self.x1.min(o.x1),
            y1: self.y1.min(o.y1),
        }
    }

    pub fn union(&self, o: Rect) -> Rect {
        if self.is_empty() {
            return o;
        }
        if o.is_empty() {
            return *self;
        }
        Rect {
            x0: self.x0.min(o.x0),
            y0: self.y0.min(o.y0),
            x1: self.x1.max(o.x1),
            y1: self.y1.max(o.y1),
        }
    }

    /// Grow by `n` on every side, then clamp to `0..w, 0..h`.
    pub fn expand_clamped(&self, n: i32, w: i32, h: i32) -> Rect {
        Rect {
            x0: (self.x0 - n).max(0),
            y0: (self.y0 - n).max(0),
            x1: (self.x1 + n).min(w),
            y1: (self.y1 + n).min(h),
        }
    }

    pub fn clamp(&self, w: i32, h: i32) -> Rect {
        Rect {
            x0: self.x0.max(0).min(w),
            y0: self.y0.max(0).min(h),
            x1: self.x1.max(0).min(w),
            y1: self.y1.max(0).min(h),
        }
    }
}

/// One raster layer.
#[derive(Clone)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub visible: bool,
    pub opacity: u8,
    pub blend: BlendMode,
    pub offset: (i32, i32),
    pub w: i32,
    pub h: i32,
    /// `w * h * 4` straight RGBA bytes.
    pub px: Arc<Vec<u8>>,
    /// Optional `w * h` coverage mask (255 = opaque everywhere the layer has paint).
    pub mask: Option<Arc<Vec<u8>>>,
}

impl Layer {
    pub fn new(id: u32, name: &str, w: i32, h: i32) -> Layer {
        Layer {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 255,
            blend: BlendMode::Normal,
            offset: (0, 0),
            w,
            h,
            px: Arc::new(vec![0u8; (w as usize) * (h as usize) * 4]),
            mask: None,
        }
    }

    pub fn from_pixels(id: u32, name: &str, w: i32, h: i32, px: Vec<u8>) -> Layer {
        let need = (w as usize) * (h as usize) * 4;
        let mut px = px;
        px.resize(need, 0);
        Layer {
            id,
            name: name.to_string(),
            visible: true,
            opacity: 255,
            blend: BlendMode::Normal,
            offset: (0, 0),
            w,
            h,
            px: Arc::new(px),
            mask: None,
        }
    }

    /// Mutable access to the pixels, copying them only if a history snapshot still holds them.
    #[inline]
    pub fn pixels_mut(&mut self) -> &mut Vec<u8> {
        Arc::make_mut(&mut self.px)
    }

    #[inline]
    pub fn mask_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.mask.as_mut().map(Arc::make_mut)
    }

    #[inline]
    pub fn contains(&self, x: i32, y: i32) -> bool {
        let lx = x - self.offset.0;
        let ly = y - self.offset.1;
        lx >= 0 && ly >= 0 && lx < self.w && ly < self.h
    }

    #[inline]
    pub fn pixel(&self, lx: i32, ly: i32) -> [u8; 4] {
        if lx < 0 || ly < 0 || lx >= self.w || ly >= self.h {
            return [0, 0, 0, 0];
        }
        let o = ((ly as usize) * (self.w as usize) + lx as usize) * 4;
        [self.px[o], self.px[o + 1], self.px[o + 2], self.px[o + 3]]
    }

    /// The layer's own coverage mask at a layer-local pixel, or 255 when it has no mask.
    #[inline]
    pub fn mask_at(&self, lx: i32, ly: i32) -> u8 {
        match &self.mask {
            None => 255,
            Some(m) => {
                if lx < 0 || ly < 0 || lx >= self.w || ly >= self.h {
                    return 0;
                }
                m[((ly as usize) * (self.w as usize) + lx as usize).min(m.len().saturating_sub(1))]
            }
        }
    }

    /// The bounding box of non-transparent pixels, or an empty rect.
    pub fn content_bounds(&self) -> Rect {
        let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for y in 0..self.h {
            let row = (y as usize) * (self.w as usize) * 4;
            for x in 0..self.w {
                if self.px[row + (x as usize) * 4 + 3] != 0 {
                    x0 = x0.min(x);
                    y0 = y0.min(y);
                    x1 = x1.max(x + 1);
                    y1 = y1.max(y + 1);
                }
            }
        }
        if x1 <= x0 {
            return Rect::empty();
        }
        Rect::new(x0 + self.offset.0, y0 + self.offset.1, x1 + self.offset.0, y1 + self.offset.1)
    }
}

/// A history snapshot. Cloning it copies pointers, not pixels.
#[derive(Clone)]
pub struct Snapshot {
    pub name: String,
    pub layers: Vec<Layer>,
    pub active: usize,
    pub sel: Option<Arc<Vec<u8>>>,
}

/// A brush stroke in progress, so `pointerDown`/`pointerMove`/`pointerUp` can span JNI calls.
#[derive(Clone, Debug)]
pub struct StrokeState {
    pub active: bool,
    pub last: (f32, f32),
    pub last_pressure: f32,
    pub color: [u8; 4],
    pub size: f32,
    pub hardness: f32,
    pub flow: f32,
    pub opacity: f32,
    pub erase: bool,
    /// Distance carried over between segments, so spacing stays even across rapid moves.
    pub residual: f32,
    pub bounds: Rect,
}

impl Default for StrokeState {
    fn default() -> Self {
        StrokeState {
            active: false,
            last: (0.0, 0.0),
            last_pressure: 1.0,
            color: [0, 0, 0, 255],
            size: 24.0,
            hardness: 0.8,
            flow: 1.0,
            opacity: 1.0,
            erase: false,
            residual: 0.0,
            bounds: Rect::empty(),
        }
    }
}

/// A document: a canvas, a layer stack, a selection, history, and a composite cache.
pub struct Document {
    pub w: i32,
    pub h: i32,
    pub layers: Vec<Layer>,
    pub active: usize,
    /// Document-sized selection coverage (255 = fully selected). `None` means "everything".
    pub sel: Option<Arc<Vec<u8>>>,
    pub undo: Vec<Snapshot>,
    pub redo: Vec<Snapshot>,
    pub next_id: u32,
    pub name: String,
    pub stroke: StrokeState,
    /// The tool the current stroke belongs to: 0 brush, 1 eraser, 2 rectangle, 3 ellipse.
    pub stroke_tool: u8,

    cache: Option<Arc<Vec<u8>>>,
    cache_dirty: Rect,
}

impl Document {
    pub fn new(w: i32, h: i32) -> Document {
        let w = w.clamp(1, 16384);
        let h = h.clamp(1, 16384);
        let mut doc = Document {
            w,
            h,
            layers: Vec::new(),
            active: 0,
            sel: None,
            undo: Vec::new(),
            redo: Vec::new(),
            next_id: 1,
            name: "Untitled".to_string(),
            stroke: StrokeState::default(),
            stroke_tool: 0,
            cache: None,
            cache_dirty: Rect::new(0, 0, w, h),
        };
        doc.add_layer_internal("Background", false);
        doc
    }

    // --- layer management -----------------------------------------------------------------

    fn alloc_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_layer(&mut self, name: &str) {
        self.push_undo("New Layer");
        self.add_layer_internal(name, true);
    }

    fn add_layer_internal(&mut self, name: &str, activate: bool) {
        let id = self.alloc_id();
        let layer = Layer::new(id, name, self.w, self.h);
        let at = self.layers.len();
        self.layers.push(layer);
        if activate {
            self.active = at;
        }
        self.invalidate_all();
    }

    /// Insert a layer above the active one, with pixel data supplied by the caller (an import, or
    /// text rasterized by the platform).
    pub fn add_layer_from_pixels(&mut self, name: &str, w: i32, h: i32, px: Vec<u8>) -> usize {
        self.push_undo("Add Layer");
        let id = self.alloc_id();
        let mut layer = Layer::from_pixels(id, name, w, h, px);
        layer.offset = (0, 0);
        let at = self.layers.len();
        self.layers.push(layer);
        self.active = at;
        self.invalidate_all();
        at
    }

    pub fn delete_layer(&mut self, idx: usize) -> bool {
        if self.layers.len() <= 1 || idx >= self.layers.len() {
            return false;
        }
        self.push_undo("Delete Layer");
        self.layers.remove(idx);
        if self.active >= self.layers.len() {
            self.active = self.layers.len() - 1;
        }
        self.invalidate_all();
        true
    }

    pub fn duplicate_layer(&mut self, idx: usize) -> bool {
        if idx >= self.layers.len() {
            return false;
        }
        self.push_undo("Duplicate Layer");
        let mut copy = self.layers[idx].clone();
        copy.id = self.alloc_id();
        copy.name = format!("{} copy", copy.name);
        self.layers.insert(idx + 1, copy);
        self.active = idx + 1;
        self.invalidate_all();
        true
    }

    /// Merge `idx` down into `idx - 1`, respecting the upper layer's opacity, blend and mask.
    pub fn merge_down(&mut self, idx: usize) -> bool {
        if idx == 0 || idx >= self.layers.len() {
            return false;
        }
        self.push_undo("Merge Down");
        let upper = self.layers[idx].clone();
        let scale = upper.opacity as f32 / 255.0;
        let (dw, dh) = (self.w, self.h);
        let lower = &mut self.layers[idx - 1];
        for y in 0..dh {
            for x in 0..dw {
                let lx = x - upper.offset.0;
                let ly = y - upper.offset.1;
                if lx < 0 || ly < 0 || lx >= upper.w || ly >= upper.h {
                    continue;
                }
                let src = upper.pixel(lx, ly);
                if src[3] == 0 {
                    continue;
                }
                let m = upper.mask_at(lx, ly);
                if m == 0 {
                    continue;
                }
                let a = scale * (m as f32 / 255.0);
                let dlx = x - lower.offset.0;
                let dly = y - lower.offset.1;
                if dlx < 0 || dly < 0 || dlx >= lower.w || dly >= lower.h {
                    continue;
                }
                let dst = lower.pixel(dlx, dly);
                let out = composite_px(dst, src, upper.blend, a);
                let o = ((dly as usize) * (lower.w as usize) + dlx as usize) * 4;
                let buf = lower.pixels_mut();
                buf[o] = out[0];
                buf[o + 1] = out[1];
                buf[o + 2] = out[2];
                buf[o + 3] = out[3];
            }
        }
        self.layers.remove(idx);
        self.active = idx - 1;
        self.invalidate_all();
        true
    }

    pub fn move_layer(&mut self, from: usize, to: usize) -> bool {
        let n = self.layers.len();
        if from >= n || to >= n || from == to {
            return false;
        }
        self.push_undo("Reorder Layers");
        let layer = self.layers.remove(from);
        self.layers.insert(to, layer);
        self.active = to;
        self.invalidate_all();
        true
    }

    pub fn set_visible(&mut self, idx: usize, visible: bool) {
        if idx < self.layers.len() {
            self.push_undo(if visible { "Show Layer" } else { "Hide Layer" });
            self.layers[idx].visible = visible;
            self.invalidate_all();
        }
    }

    pub fn set_opacity(&mut self, idx: usize, opacity: u8) {
        if idx < self.layers.len() {
            self.layers[idx].opacity = opacity;
            self.invalidate_all();
        }
    }

    pub fn set_blend(&mut self, idx: usize, mode: BlendMode) {
        if idx < self.layers.len() {
            self.push_undo("Blend Mode");
            self.layers[idx].blend = mode;
            self.invalidate_all();
        }
    }

    pub fn set_layer_name(&mut self, idx: usize, name: &str) {
        if idx < self.layers.len() {
            self.layers[idx].name = sanitize(name);
        }
    }

    pub fn add_mask_to_layer(&mut self, idx: usize) -> bool {
        if idx >= self.layers.len() {
            return false;
        }
        self.push_undo("Add Layer Mask");
        let (w, h) = (self.layers[idx].w, self.layers[idx].h);
        self.layers[idx].mask = Some(Arc::new(vec![255u8; (w as usize) * (h as usize)]));
        self.invalidate_all();
        true
    }

    pub fn layer_index_by_id(&self, id: u32) -> Option<usize> {
        self.layers.iter().position(|l| l.id == id)
    }

    // --- cache ---------------------------------------------------------------------------

    /// Mark a document-space rectangle as needing recomposition.
    pub fn mark_dirty(&mut self, r: Rect) {
        let r = r.clamp(self.w, self.h);
        if r.is_empty() {
            return;
        }
        self.cache_dirty = self.cache_dirty.union(r);
    }

    pub fn invalidate_all(&mut self) {
        self.cache_dirty = Rect::new(0, 0, self.w, self.h);
        self.cache = None;
    }

    /// Recomposite whatever is dirty. Cheap when little changed: one edit recomposites only its
    /// own bounding box, not the document.
    pub fn ensure_cache(&mut self) {
        let full = (self.w as usize) * (self.h as usize) * 4;
        let need_new = match &self.cache {
            None => true,
            Some(c) => c.len() != full,
        };
        if need_new {
            self.cache = Some(Arc::new(vec![0u8; full]));
            self.cache_dirty = Rect::new(0, 0, self.w, self.h);
        }
        let dirty = std::mem::replace(&mut self.cache_dirty, Rect::empty());
        if dirty.is_empty() {
            return;
        }
        let mut cache = self.cache.take().unwrap();
        {
            let buf = Arc::make_mut(&mut cache);
            self.composite_rect(buf, dirty);
        }
        self.cache = Some(cache);
    }

    fn composite_rect(&self, buf: &mut [u8], r: Rect) {
        let r = r.clamp(self.w, self.h);
        if r.is_empty() {
            return;
        }
        // Clear the region to transparent first: the cache is the whole stack, not an overlay.
        for y in r.y0..r.y1 {
            let row = (y as usize) * (self.w as usize);
            for x in r.x0..r.x1 {
                let o = (row + x as usize) * 4;
                buf[o] = 0;
                buf[o + 1] = 0;
                buf[o + 2] = 0;
                buf[o + 3] = 0;
            }
        }
        for layer in self.layers.iter() {
            if !layer.visible || layer.opacity == 0 {
                continue;
            }
            let scale = layer.opacity as f32 / 255.0;
            // Clip the layer to the dirty rect, in layer-local coordinates.
            let lx0 = (r.x0 - layer.offset.0).max(0);
            let ly0 = (r.y0 - layer.offset.1).max(0);
            let lx1 = (r.x1 - layer.offset.0).min(layer.w);
            let ly1 = (r.y1 - layer.offset.1).min(layer.h);
            if lx1 <= lx0 || ly1 <= ly0 {
                continue;
            }
            for ly in ly0..ly1 {
                let dy = ly + layer.offset.1;
                let lrow = (ly as usize) * (layer.w as usize);
                let drow = (dy as usize) * (self.w as usize);
                for lx in lx0..lx1 {
                    let o = (lrow + lx as usize) * 4;
                    let a = layer.px[o + 3];
                    if a == 0 {
                        continue;
                    }
                    let m = match &layer.mask {
                        None => 255u8,
                        Some(m) => m[lrow + lx as usize],
                    };
                    if m == 0 {
                        continue;
                    }
                    let src = [layer.px[o], layer.px[o + 1], layer.px[o + 2], a];
                    let dx = lx + layer.offset.0;
                    let dofs = (drow + dx as usize) * 4;
                    let dst = [buf[dofs], buf[dofs + 1], buf[dofs + 2], buf[dofs + 3]];
                    let alpha = scale * (m as f32 / 255.0);
                    let out = composite_px(dst, src, layer.blend, alpha);
                    buf[dofs] = out[0];
                    buf[dofs + 1] = out[1];
                    buf[dofs + 2] = out[2];
                    buf[dofs + 3] = out[3];
                }
            }
        }
    }

    /// The cached composite, brought up to date.
    pub fn composite(&mut self) -> Arc<Vec<u8>> {
        self.ensure_cache();
        self.cache.clone().unwrap()
    }

    // --- viewport rendering ---------------------------------------------------------------

    /// Render the document region `(vx, vy, vw, vh)` into an ARGB buffer of `outw * outh`.
    ///
    /// `scale` is document pixels per output pixel: 1.0 is 1:1, 2.0 shows the document at 200%.
    /// Transparency is drawn as a checkerboard so the canvas reads as transparent, not black.
    pub fn render_viewport(
        &mut self,
        out: &mut [u32],
        outw: i32,
        outh: i32,
        vx: f32,
        vy: f32,
        scale: f32,
        checker: u32,
        checker2: u32,
    ) {
        if outw <= 0 || outh <= 0 || out.len() < (outw as usize) * (outh as usize) {
            return;
        }
        let src = self.composite();
        let scale = if scale.is_finite() && scale > 0.0001 { scale } else { 1.0 };
        let inv = 1.0 / scale;
        for oy in 0..outh {
            let fy = vy + (oy as f32) * inv;
            for ox in 0..outw {
                let fx = vx + (ox as f32) * inv;
                let c = sample_bilinear(&src, self.w, self.h, fx, fy);
                let bg = if ((ox >> 4) + (oy >> 4)) & 1 == 0 { checker } else { checker2 };
                out[(oy as usize) * (outw as usize) + ox as usize] = over_argb(bg, c);
            }
        }
    }

    /// Render the selection coverage the same way: 0 = unselected, 255 = selected, in the blue
    /// channel of an opaque ARGB pixel. The UI tints it and draws the marching-ants border.
    pub fn render_selection(
        &mut self,
        out: &mut [u32],
        outw: i32,
        outh: i32,
        vx: f32,
        vy: f32,
        scale: f32,
    ) {
        if outw <= 0 || outh <= 0 || out.len() < (outw as usize) * (outh as usize) {
            return;
        }
        let sel = self.sel.clone();
        let scale = if scale.is_finite() && scale > 0.0001 { scale } else { 1.0 };
        let inv = 1.0 / scale;
        for oy in 0..outh {
            let dy = (vy + (oy as f32) * inv) as i32;
            for ox in 0..outw {
                let dx = (vx + (ox as f32) * inv) as i32;
                let v = match &sel {
                    None => 255u8,
                    Some(m) => {
                        if dx < 0 || dy < 0 || dx >= self.w || dy >= self.h {
                            0
                        } else {
                            m[(dy as usize) * (self.w as usize) + dx as usize]
                        }
                    }
                };
                out[(oy as usize) * (outw as usize) + ox as usize] = 0xFF00_0000 | ((v as u32) << 8);
            }
        }
    }

    /// A layer thumbnail, box-filtered into `outw * outh` ARGB pixels.
    pub fn layer_thumb(&self, idx: usize, out: &mut [u32], outw: i32, outh: i32) {
        if idx >= self.layers.len() || outw <= 0 || outh <= 0 {
            return;
        }
        if out.len() < (outw as usize) * (outh as usize) {
            return;
        }
        let l = &self.layers[idx];
        for oy in 0..outh {
            let sy = ((oy as f32) * (l.h as f32) / (outh as f32)) as i32;
            for ox in 0..outw {
                let sx = ((ox as f32) * (l.w as f32) / (outw as f32)) as i32;
                let p = l.pixel(sx.min(l.w - 1), sy.min(l.h - 1));
                let m = l.mask_at(sx.min(l.w - 1), sy.min(l.h - 1));
                let a = ((p[3] as u32) * (m as u32) / 255) << 24;
                out[(oy as usize) * (outw as usize) + ox as usize] = a | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32);
            }
        }
    }

    /// The flattened document as opaque ARGB, for export. Transparent pixels become white paper.
    pub fn flatten_argb(&mut self) -> Vec<u32> {
        let src = self.composite();
        let mut out = vec![0xFFFF_FFFFu32; (self.w as usize) * (self.h as usize)];
        for i in 0..out.len() {
            let o = i * 4;
            let c = [src[o], src[o + 1], src[o + 2], src[o + 3]];
            out[i] = over_argb(0xFFFF_FFFF, c);
        }
        out
    }

    // --- selection -----------------------------------------------------------------------

    /// The selection buffer, created (all zero) on demand. Used by `ops`.
    pub(crate) fn selection_mask_mut_public(&mut self) -> &mut Vec<u8> {
        self.selection_mask_mut()
    }

    fn selection_mask_mut(&mut self) -> &mut Vec<u8> {
        let full = (self.w as usize) * (self.h as usize);
        let need_new = match &self.sel {
            None => true,
            Some(m) => m.len() != full,
        };
        if need_new {
            self.sel = Some(Arc::new(vec![0u8; full]));
        }
        Arc::make_mut(self.sel.as_mut().unwrap())
    }

    /// The selection coverage at a document pixel (255 when there is no selection).
    #[inline]
    pub fn selection_at(&self, x: i32, y: i32) -> u8 {
        match &self.sel {
            None => 255,
            Some(m) => {
                if x < 0 || y < 0 || x >= self.w || y >= self.h {
                    0
                } else {
                    m[(y as usize) * (self.w as usize) + x as usize]
                }
            }
        }
    }

    pub fn has_selection(&self) -> bool {
        self.sel.is_some()
    }

    /// The bounding box of the selected area, or the whole canvas when nothing is selected.
    pub fn selection_bounds(&self) -> Rect {
        match &self.sel {
            None => Rect::new(0, 0, self.w, self.h),
            Some(m) => {
                let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
                for y in 0..self.h {
                    let row = (y as usize) * (self.w as usize);
                    for x in 0..self.w {
                        if m[row + x as usize] > 0 {
                            x0 = x0.min(x);
                            y0 = y0.min(y);
                            x1 = x1.max(x + 1);
                            y1 = y1.max(y + 1);
                        }
                    }
                }
                if x1 <= x0 {
                    Rect::empty()
                } else {
                    Rect::new(x0, y0, x1, y1)
                }
            }
        }
    }

    // --- history -------------------------------------------------------------------------

    pub fn push_undo(&mut self, name: &str) {
        let snap = Snapshot {
            name: name.to_string(),
            layers: self.layers.clone(),
            active: self.active,
            sel: self.sel.clone(),
        };
        self.undo.push(snap);
        if self.undo.len() > HISTORY_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    pub fn undo(&mut self) -> bool {
        let mut prev = match self.undo.pop() {
            Some(s) => s,
            None => return false,
        };
        let cur = Snapshot {
            name: prev.name.clone(),
            layers: self.layers.clone(),
            active: self.active,
            sel: self.sel.clone(),
        };
        self.redo.push(cur);
        std::mem::swap(&mut self.layers, &mut prev.layers);
        self.active = prev.active.min(self.layers.len().saturating_sub(1));
        self.sel = prev.sel;
        self.invalidate_all();
        true
    }

    pub fn redo(&mut self) -> bool {
        let mut next = match self.redo.pop() {
            Some(s) => s,
            None => return false,
        };
        let cur = Snapshot {
            name: next.name.clone(),
            layers: self.layers.clone(),
            active: self.active,
            sel: self.sel.clone(),
        };
        self.undo.push(cur);
        std::mem::swap(&mut self.layers, &mut next.layers);
        self.active = next.active.min(self.layers.len().saturating_sub(1));
        self.sel = next.sel;
        self.invalidate_all();
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo_name(&self) -> &str {
        self.undo.last().map(|s| s.name.as_str()).unwrap_or("")
    }

    pub fn redo_name(&self) -> &str {
        self.redo.last().map(|s| s.name.as_str()).unwrap_or("")
    }

    // --- save / load ---------------------------------------------------------------------

    /// Write the document: a magic line, a text manifest, then each layer's pixels and mask as
    /// run-length encoded RGBA. Saving a document (not an image) keeps layers, masks, blend modes
    /// and the selection.
    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(MAGIC);
        let mut manifest = String::new();
        manifest.push_str(&format!("doc {} {} {} {}\n", self.w, self.h, self.active, self.layers.len()));
        for (i, l) in self.layers.iter().enumerate() {
            manifest.push_str(&format!(
                "layer {} {} {} {} {} {} {} {} {} {}\n",
                i,
                l.id,
                if l.visible { 1 } else { 0 },
                l.opacity,
                l.blend as u8,
                l.offset.0,
                l.offset.1,
                l.w,
                l.h,
                l.name.replace('\n', " ").replace('\r', " ")
            ));
        }
        let mb = manifest.into_bytes();
        out.extend_from_slice(&(mb.len() as u32).to_le_bytes());
        out.extend_from_slice(&mb);
        for l in self.layers.iter() {
            write_block(&mut out, &l.px)?;
            match &l.mask {
                None => out.extend_from_slice(&0u32.to_le_bytes()),
                Some(m) => write_block(&mut out, m)?,
            }
        }
        let tmp = format!("{}.tmp", path);
        {
            let mut f = fs::File::create(&tmp)?;
            f.write_all(&out)?;
            f.flush()?;
        }
        fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn load(path: &str) -> io::Result<Document> {
        let mut buf = Vec::new();
        fs::File::open(Path::new(path))?.read_to_end(&mut buf)?;
        Document::from_bytes(&buf)
    }

    pub fn from_bytes(buf: &[u8]) -> io::Result<Document> {
        let bad = |m: &str| io::Error::new(io::ErrorKind::InvalidData, m.to_string());
        if buf.len() < MAGIC.len() || &buf[..MAGIC.len()] != MAGIC {
            return Err(bad("not a REA Image Studio document"));
        }
        let mut p = MAGIC.len();
        if buf.len() < p + 4 {
            return Err(bad("truncated header"));
        }
        let mlen = u32::from_le_bytes([buf[p], buf[p + 1], buf[p + 2], buf[p + 3]]) as usize;
        p += 4;
        if buf.len() < p + mlen {
            return Err(bad("truncated manifest"));
        }
        let manifest = String::from_utf8_lossy(&buf[p..p + mlen]).to_string();
        p += mlen;

        let mut w = 1;
        let mut h = 1;
        let mut active = 0usize;
        let mut count = 0usize;
        let mut metas: Vec<(u32, bool, u8, BlendMode, i32, i32, i32, i32, String)> = Vec::new();
        for line in manifest.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "doc" if parts.len() >= 5 => {
                    w = parts[1].parse().unwrap_or(1);
                    h = parts[2].parse().unwrap_or(1);
                    active = parts[3].parse().unwrap_or(0);
                    count = parts[4].parse().unwrap_or(0);
                }
                "layer" if parts.len() >= 10 => {
                    let id = parts[2].parse().unwrap_or(1);
                    let vis = parts[3] == "1";
                    let op = parts[4].parse().unwrap_or(255);
                    let blend = BlendMode::from_u8(parts[5].parse().unwrap_or(0));
                    let ox = parts[6].parse().unwrap_or(0);
                    let oy = parts[7].parse().unwrap_or(0);
                    let lw = parts[8].parse().unwrap_or(w);
                    let lh = parts[9].parse().unwrap_or(h);
                    // The name is the rest of the line, so it may contain spaces.
                    let name = parts[10..].join(" ");
                    metas.push((id, vis, op, blend, ox, oy, lw, lh, name));
                }
                _ => {}
            }
        }
        if count == 0 || count != metas.len() {
            return Err(bad("manifest layer count mismatch"));
        }
        let mut layers = Vec::with_capacity(metas.len());
        for (id, vis, op, blend, ox, oy, lw, lh, name) in metas {
            let (px, np) = read_block(buf, p)?;
            p = np;
            let (mask, np2) = read_block(buf, p)?;
            p = np2;
            let need = (lw.max(0) as usize) * (lh.max(0) as usize) * 4;
            let mut px = px;
            px.resize(need, 0);
            layers.push(Layer {
                id,
                name: if name.trim().is_empty() { "Layer".to_string() } else { name },
                visible: vis,
                opacity: op,
                blend,
                offset: (ox, oy),
                w: lw,
                h: lh,
                px: Arc::new(px),
                mask: if mask.is_empty() { None } else { Some(Arc::new(mask)) },
            });
        }
        if layers.is_empty() {
            return Err(bad("no layers"));
        }
        let mut doc = Document {
            w,
            h,
            layers,
            active: active.min(count.saturating_sub(1)),
            sel: None,
            undo: Vec::new(),
            redo: Vec::new(),
            next_id: 1,
            name: "Untitled".to_string(),
            stroke: StrokeState::default(),
            stroke_tool: 0,
            cache: None,
            cache_dirty: Rect::new(0, 0, w, h),
        };
        doc.next_id = doc.layers.iter().map(|l| l.id).max().unwrap_or(0) + 1;
        Ok(doc)
    }
}

fn sanitize(s: &str) -> String {
    let cleaned: String = s.chars().filter(|c| *c != '\n' && *c != '\r').take(64).collect();
    if cleaned.trim().is_empty() {
        "Layer".to_string()
    } else {
        cleaned
    }
}

/// Run-length encode in units of 4-byte pixels: `u16 count` then the RGBA quadruple.
fn rle_encode(px: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(px.len() / 4);
    let n = px.len() / 4;
    let mut i = 0usize;
    while i < n {
        let c = &px[i * 4..i * 4 + 4];
        let mut run = 1usize;
        while i + run < n && run < 65535 && &px[(i + run) * 4..(i + run) * 4 + 4] == c {
            run += 1;
        }
        out.extend_from_slice(&(run as u16).to_le_bytes());
        out.extend_from_slice(c);
        i += run;
    }
    out
}

fn rle_decode(data: &[u8], pixels: usize) -> io::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(pixels * 4);
    let mut i = 0usize;
    while i + 6 <= data.len() && out.len() / 4 < pixels {
        let run = u16::from_le_bytes([data[i], data[i + 1]]) as usize;
        let px = [data[i + 2], data[i + 3], data[i + 4], data[i + 5]];
        i += 6;
        for _ in 0..run {
            out.extend_from_slice(&px);
        }
    }
    out.resize(pixels * 4, 0);
    Ok(out)
}

/// A length-prefixed RLE block. A zero length means "absent" (no mask).
fn write_block(out: &mut Vec<u8>, data: &[u8]) -> io::Result<()> {
    let enc = rle_encode(data);
    out.extend_from_slice(&(enc.len() as u32).to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(&enc);
    Ok(())
}

fn read_block(buf: &[u8], p: usize) -> io::Result<(Vec<u8>, usize)> {
    if p + 8 > buf.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "truncated block"));
    }
    let enc_len = u32::from_le_bytes([buf[p], buf[p + 1], buf[p + 2], buf[p + 3]]) as usize;
    let raw_len = u32::from_le_bytes([buf[p + 4], buf[p + 5], buf[p + 6], buf[p + 7]]) as usize;
    let start = p + 8;
    if enc_len == 0 {
        return Ok((Vec::new(), start));
    }
    if start + enc_len > buf.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "truncated block body"));
    }
    let pixels = raw_len / 4;
    let data = rle_decode(&buf[start..start + enc_len], pixels)?;
    Ok((data, start + enc_len))
}

#[inline]
fn sample_bilinear(src: &[u8], w: i32, h: i32, fx: f32, fy: f32) -> [u8; 4] {
    if w <= 0 || h <= 0 {
        return [0, 0, 0, 0];
    }
    if fx < 0.0 || fy < 0.0 || fx >= w as f32 || fy >= h as f32 {
        return [0, 0, 0, 0];
    }
    let x0 = fx.floor() as i32;
    let y0 = fy.floor() as i32;
    let tx = fx - x0 as f32;
    let ty = fy - y0 as f32;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let px = |x: i32, y: i32| -> [f32; 4] {
        let o = ((y as usize) * (w as usize) + x as usize) * 4;
        [src[o] as f32, src[o + 1] as f32, src[o + 2] as f32, src[o + 3] as f32]
    };
    let a = px(x0, y0);
    let b = px(x1, y0);
    let c = px(x0, y1);
    let d = px(x1, y1);
    let mut out = [0u8; 4];
    for i in 0..4 {
        let top = a[i] * (1.0 - tx) + b[i] * tx;
        let bot = c[i] * (1.0 - tx) + d[i] * tx;
        out[i] = (top * (1.0 - ty) + bot * ty).round().clamp(0.0, 255.0) as u8;
    }
    out
}

/// Source-over one straight-RGBA pixel onto an opaque ARGB background.
#[inline]
fn over_argb(bg: u32, c: [u8; 4]) -> u32 {
    let a = c[3] as u32;
    if a == 0 {
        return bg;
    }
    if a == 255 {
        return 0xFF00_0000 | ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | c[2] as u32;
    }
    let sa = a as f32 / 255.0;
    let br = ((bg >> 16) & 0xFF) as f32;
    let bgc = ((bg >> 8) & 0xFF) as f32;
    let bb = (bg & 0xFF) as f32;
    let r = c[0] as f32 * sa + br * (1.0 - sa);
    let g = c[1] as f32 * sa + bgc * (1.0 - sa);
    let b = c[2] as f32 * sa + bb * (1.0 - sa);
    0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn painted_doc() -> Document {
        let mut d = Document::new(64, 48);
        // Background is red, then a small blue square on a second layer.
        {
            let bg = &mut d.layers[0];
            let buf = bg.pixels_mut();
            for i in (0..buf.len()).step_by(4) {
                buf[i] = 255;
                buf[i + 3] = 255;
            }
        }
        d.add_layer("Blue");
        {
            let l = &mut d.layers[1];
            let buf = l.pixels_mut();
            for y in 10..20 {
                for x in 10..20 {
                    let o = (y * 64 + x) * 4;
                    buf[o] = 0;
                    buf[o + 1] = 0;
                    buf[o + 2] = 255;
                    buf[o + 3] = 255;
                }
            }
        }
        d.invalidate_all();
        d
    }

    #[test]
    fn new_document_has_one_layer_and_is_transparent() {
        let mut d = Document::new(32, 32);
        assert_eq!(d.layers.len(), 1);
        let c = d.composite();
        assert_eq!(c[3], 0);
    }

    #[test]
    fn composite_puts_the_upper_layer_on_top() {
        let mut d = painted_doc();
        let c = d.composite();
        let at = |x: usize, y: usize| {
            let o = (y * 64 + x) * 4;
            [c[o], c[o + 1], c[o + 2], c[o + 3]]
        };
        assert_eq!(at(5, 5), [255, 0, 0, 255], "backdrop should be red");
        assert_eq!(at(15, 15), [0, 0, 255, 255], "the square should be blue");
    }

    #[test]
    fn viewport_render_fills_every_output_pixel() {
        let mut d = painted_doc();
        let (ow, oh) = (40, 30);
        let mut out = vec![0u32; (ow * oh) as usize];
        d.render_viewport(&mut out, ow, oh, 0.0, 0.0, 1.0, 0xFF33_3333, 0xFF2A_2A2A);
        assert!(out.iter().all(|p| (*p >> 24) == 0xFF), "every pixel must be opaque");
        // 1:1 at the origin: pixel (12,12) is inside the blue square.
        assert_eq!(out[(12 * ow + 12) as usize], 0xFF00_00FF);
    }

    #[test]
    fn viewport_ateless_scaling_does_not_panic_on_empty_documents() {
        let mut d = Document::new(8, 8);
        let mut out = vec![0u32; 16];
        d.render_viewport(&mut out, 4, 4, -100.0, -100.0, 0.5, 0, 0);
        assert_eq!(out.len(), 16);
    }

    #[test]
    fn only_the_dirty_rect_is_recomposited() {
        let mut d = painted_doc();
        let _ = d.composite();
        // Change one pixel and mark only that rect dirty.
        {
            let l = &mut d.layers[1];
            let buf = l.pixels_mut();
            let o = (12 * 64 + 12) * 4;
            buf[o] = 0;
            buf[o + 1] = 255;
            buf[o + 2] = 0;
        }
        d.mark_dirty(Rect::new(12, 12, 13, 13));
        let c = d.composite();
        let o = (12 * 64 + 12) * 4;
        assert_eq!([c[o], c[o + 1], c[o + 2]], [0, 255, 0], "the edit should show");
        let o2 = (30 * 64 + 30) * 4;
        assert_eq!([c[o2], c[o2 + 1], c[o2 + 2]], [255, 0, 0], "far pixels keep the backdrop");
    }

    #[test]
    fn hiding_a_layer_removes_it_from_the_composite() {
        let mut d = painted_doc();
        d.set_visible(1, false);
        let c = d.composite();
        let o = (15 * 64 + 15) * 4;
        assert_eq!([c[o], c[o + 1], c[o + 2]], [255, 0, 0]);
    }

    #[test]
    fn opacity_halves_a_blend() {
        let mut d = painted_doc();
        d.set_opacity(1, 128);
        let c = d.composite();
        let o = (15 * 64 + 15) * 4;
        // Half blue over red.
        assert!(c[o] > 100 && c[o] < 160, "red channel {} should be mid", c[o]);
        assert!(c[o + 2] > 100, "blue channel {} should be mid", c[o + 2]);
    }

    #[test]
    fn undo_and_redo_restore_the_stack() {
        let mut d = painted_doc();
        d.add_layer("Third");
        assert_eq!(d.layers.len(), 3);
        assert!(d.undo());
        assert_eq!(d.layers.len(), 2);
        assert!(d.redo());
        assert_eq!(d.layers.len(), 3);
    }

    #[test]
    fn undo_restores_pixels_not_just_the_layer_list() {
        let mut d = painted_doc();
        d.push_undo("Paint");
        {
            let l = &mut d.layers[1];
            let buf = l.pixels_mut();
            let o = (12 * 64 + 12) * 4;
            buf[o + 1] = 255;
        }
        d.invalidate_all();
        let c = d.composite();
        assert_eq!(c[(12 * 64 + 12) * 4 + 1], 255);
        d.undo();
        let c = d.composite();
        assert_eq!(c[(12 * 64 + 12) * 4 + 1], 0, "the paint should be gone");
    }

    #[test]
    fn merge_down_folds_the_upper_layer_in() {
        let mut d = painted_doc();
        assert!(d.merge_down(1));
        assert_eq!(d.layers.len(), 1);
        let c = d.composite();
        let o = (15 * 64 + 15) * 4;
        assert_eq!([c[o], c[o + 2]], [0, 255]);
    }

    #[test]
    fn reading_and_writing_a_document_round_trips() {
        let mut d = painted_doc();
        d.layers[1].blend = BlendMode::Multiply;
        d.layers[1].opacity = 200;
        d.layers[0].name = "Backdrop layer".to_string();
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(MAGIC);
        let mut manifest = String::new();
        manifest.push_str(&format!("doc {} {} {} {}\n", d.w, d.h, d.active, d.layers.len()));
        for (i, l) in d.layers.iter().enumerate() {
            manifest.push_str(&format!(
                "layer {} {} {} {} {} {} {} {} {} {}\n",
                i, l.id, if l.visible { 1 } else { 0 }, l.opacity, l.blend as u8,
                l.offset.0, l.offset.1, l.w, l.h, l.name
            ));
        }
        let mb = manifest.into_bytes();
        buf.extend_from_slice(&(mb.len() as u32).to_le_bytes());
        buf.extend_from_slice(&mb);
        for l in d.layers.iter() {
            write_block(&mut buf, &l.px).unwrap();
            match &l.mask {
                None => buf.extend_from_slice(&0u32.to_le_bytes()),
                Some(m) => write_block(&mut buf, m).unwrap(),
            }
        }
        let back = Document::from_bytes(&buf).expect("should load");
        assert_eq!(back.w, d.w);
        assert_eq!(back.h, d.h);
        assert_eq!(back.layers.len(), 2);
        assert_eq!(back.layers[0].name, "Backdrop layer");
        assert_eq!(back.layers[1].blend, BlendMode::Multiply);
        assert_eq!(back.layers[1].opacity, 200);
        let a = d.layers[1].pixel(10, 10);
        let b = back.layers[1].pixel(10, 10);
        assert_eq!(a, b, "pixels must survive the round trip");
    }

    #[test]
    fn saving_and_loading_a_real_file_round_trips_masks_too() {
        let dir = std::env::temp_dir().join(format!("rea-test-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("doc.rea");
        let mut d = painted_doc();
        assert!(d.add_mask_to_layer(1));
        if let Some(m) = d.layers[1].mask_mut() {
            m[0] = 0;
        }
        let p = path.to_string_lossy().to_string();
        d.save(&p).expect("save");
        let back = Document::load(&p).expect("load");
        assert_eq!(back.layers.len(), 2);
        assert!(back.layers[1].mask.is_some(), "the mask must be kept");
        assert_eq!(back.layers[1].mask_at(0, 0), 0);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_wrong_magic_is_rejected() {
        assert!(Document::from_bytes(b"nonsense").is_err());
    }

    #[test]
    fn rle_round_trips_runs_and_singletons() {
        let mut px = Vec::new();
        for _ in 0..100 {
            px.extend_from_slice(&[1, 2, 3, 4]);
        }
        for i in 0..10u8 {
            px.extend_from_slice(&[i, i, i, i]);
        }
        let enc = rle_encode(&px);
        assert!(enc.len() < px.len(), "a long run should shrink");
        let dec = rle_decode(&enc, px.len() / 4).unwrap();
        assert_eq!(dec, px);
    }

    #[test]
    fn selection_bounds_cover_the_selected_pixels() {
        let mut d = Document::new(20, 20);
        let m = d.selection_mask_mut();
        for y in 5..10 {
            for x in 3..7 {
                m[(y * 20 + x) as usize] = 255;
            }
        }
        assert_eq!(d.selection_bounds(), Rect::new(3, 5, 7, 10));
        assert_eq!(d.selection_at(4, 6), 255);
        assert_eq!(d.selection_at(0, 0), 0);
    }

    #[test]
    fn no_selection_means_the_whole_canvas_is_selected() {
        let d = Document::new(20, 10);
        assert_eq!(d.selection_at(0, 0), 255);
        assert_eq!(d.selection_bounds(), Rect::new(0, 0, 20, 10));
    }

    #[test]
    fn flatten_produces_opaque_argb() {
        let mut d = painted_doc();
        let flat = d.flatten_argb();
        assert_eq!(flat.len(), (64 * 48) as usize);
        assert!(flat.iter().all(|p| (*p >> 24) == 0xFF));
    }
}
