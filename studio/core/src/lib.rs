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

#[cfg(test)]
mod tests {
    use super::*;

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
