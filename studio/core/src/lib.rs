//! REA Image Studio's editing core.
//!
//! This crate is the original engine behind the app: a document model with layers, a brush, a
//! selection, adjustments, filters, history and its own file format. It has no dependencies at all
//! on the host, which is what lets its test suite run natively in a couple of minutes; the JNI
//! bridge that the Kotlin UI calls is compiled only for Android.
//!
//! Layout:
//! - [`blend`] — blend modes and the W3C source-over compositing formula
//! - [`doc`] — layers, the incremental composite cache, history, `.rea` load/save, viewport render
//! - [`ops`] — brush, shapes, bucket fill, selections, adjustments, filters, transforms, canvas

pub mod blend;
pub mod doc;
pub mod ops;

pub use blend::BlendMode;
pub use doc::{Document, Layer, Rect};

/// The core's version, shown in the app's About panel and recorded in the diagnostic log.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A one-line description of the core, for the About panel.
pub fn about() -> String {
    format!("REA Image Studio core {} (original implementation)", VERSION)
}

#[cfg(target_os = "android")]
mod jni_bridge;

/// Exercise the whole core the way the UI does, and report what happened, one line per check.
///
/// This exists because the app is developed on a machine that cannot run it: when something is
/// wrong on a device, the app runs this and shows the result, so the failure is a sentence rather
/// than a crash. `scratch` must be a directory the app may write to.
pub fn self_test(scratch: &str) -> String {
    use std::fmt::Write as _;

    let mut report = String::new();
    let mut failures = 0usize;
    let mut check = |name: &str, ok: bool, detail: String| {
        if !ok {
            failures += 1;
        }
        let _ = writeln!(report, "[{}] {} {}", if ok { "ok" } else { "FAIL" }, name, detail);
    };

    // 1. A document, a layer, a stroke.
    let mut d = Document::new(96, 64);
    let layers_before = d.layers.len();
    d.stroke_begin(8.0, 8.0, 1.0, 0, [255, 0, 0, 255], 14.0, 1.0, 1.0, 1.0);
    d.stroke_to(88.0, 56.0, 1.0);
    d.stroke_end();
    let painted = (0..64)
        .flat_map(|y| (0..96).map(move |x| (x, y)))
        .filter(|(x, y)| {
            let i = ((*y as usize) * 96 + *x as usize) * 4;
            d.layers[0].px[i + 3] > 0
        })
        .count();
    check("document", d.w == 96 && d.h == 64 && layers_before == 1, format!("{}x{}, {} layer(s)", d.w, d.h, layers_before));
    check("brush", painted > 100, format!("{painted} pixels painted"));

    // 2. Compositing and the viewport renderer at a zoom the UI really uses.
    let mut out = vec![0u32; 80 * 60];
    let zoom = 0.83f32;
    d.render_viewport(&mut out, 80, 60, -3.0, -2.0, 1.0 / zoom, 0xFF32_3232, 0xFF28_2828);
    let opaque = out.iter().filter(|p| (**p >> 24) == 0xFF).count();
    check("render", opaque == out.len(), format!("{opaque}/{} pixels opaque", out.len()));

    // 3. History.
    let before_undo = d.layers[0].px.iter().filter(|b| **b != 0).count();
    let undone = d.undo();
    let after_undo = d.layers[0].px.iter().filter(|b| **b != 0).count();
    check("undo", undone && after_undo < before_undo, format!("{before_undo} -> {after_undo} non-zero bytes"));

    // 4. Selections and an adjustment.
    d.select_rect(10.0, 10.0, 60.0, 40.0, 0.0);
    let bounds = d.selection_bounds();
    d.adjust(0, &[20.0, 0.0]);
    check("selection", d.has_selection() && bounds.width() == 50 && bounds.height() == 30, format!("{}x{}", bounds.width(), bounds.height()));

    // 5. The document format: save, load, compare.
    let path = format!("{}/selftest.rea", scratch.trim_end_matches('/'));
    let saved = d.save(&path).is_ok();
    let reloaded = Document::load(&path).ok();
    let same = match &reloaded {
        Some(r) => r.w == d.w && r.h == d.h && r.layers.len() == d.layers.len(),
        None => false,
    };
    let _ = std::fs::remove_file(&path);
    check("save/load", saved && same, format!("{:?}", reloaded.as_ref().map(|r| (r.w, r.h))));

    // 6. Flatten for export.
    let flat = d.flatten_argb();
    check("export", flat.len() == (d.w * d.h) as usize && flat.iter().all(|p| p >> 24 == 0xFF), format!("{} pixels", flat.len()));

    drop(check);
    if failures == 0 {
        report.push_str("all checks passed\n");
    } else {
        let _ = writeln!(report, "{failures} check(s) failed");
    }
    report
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_passes_on_the_host() {
        let dir = std::env::temp_dir().join("rea-selftest-host");
        std::fs::create_dir_all(&dir).unwrap();
        let report = self_test(dir.to_str().unwrap());
        assert!(
            report.contains("all checks passed"),
            "self-test reported a failure:\n{report}"
        );
    }

    #[test]
    fn the_core_reports_a_version() {
        assert!(about().contains(VERSION));
    }

    #[test]
    fn a_full_edit_session_behaves_end_to_end() {
        // The kind of sequence a user actually performs, exercised through the public API only.
        let mut d = Document::new(200, 150);
        d.add_layer("Paint");
        d.stroke_begin(20.0, 20.0, 1.0, 0, [255, 80, 0, 255], 30.0, 0.7, 1.0, 1.0);
        d.stroke_to(120.0, 90.0, 0.8);
        d.stroke_end();

        d.select_rect(0.0, 0.0, 200.0, 150.0, 0.0);
        d.adjust(0, &[10.0, 15.0]);
        d.invert_selection();
        d.select_none();

        d.add_layer("Art");
        d.bucket_fill(10, 10, [0, 40, 90, 255], 200);
        d.set_blend(2, BlendMode::Screen);
        d.set_opacity(2, 180);

        let mut out = vec![0u32; 100 * 75];
        d.render_viewport(&mut out, 100, 75, 0.0, 0.0, 2.0, 0xFF33_3333, 0xFF2A_2A2A);
        assert!(out.iter().all(|p| (*p >> 24) == 0xFF));

        assert!(d.can_undo());
        let mut steps = 0;
        while d.undo() {
            steps += 1;
            assert!(steps < 100, "undo must terminate");
        }
        assert!(steps >= 5, "each operation should be undoable, got {}", steps);
        while d.redo() {}
        assert_eq!(d.layers.len(), 3);
    }
}
