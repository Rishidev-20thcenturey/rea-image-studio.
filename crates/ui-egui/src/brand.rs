//! The app's own brand mark, at the left of the title bar where Photoshop shows its "Ps" tile.
//!
//! It is **drawn, not shipped**: three stacked rounded squares (layers, which is what this editor
//! is for) on a tile in the theme's accent colour. That is the same construction as the launcher
//! icon, so the tile in the app and the icon on the home screen are one mark.
//!
//! Drawing it means three things that matter for a product:
//!
//! - No brand artwork is compiled into the binary. The first revision of this fork painted the
//!   upstream project's kitsune icon here; it is licensed MIT/Apache-2.0, so that was legal, but it
//!   is *their* mark and this is not their app.
//! - It recolours with the theme: the tile takes the accent, so it is ink blue in REA Ink and lifts
//!   in REA Ink Dark without a second asset.
//! - It is crisp at every size and display scale, because it is geometry rather than a bitmap.

use egui::{Color32, Context, Id, Rect, Stroke, Ui, pos2};

/// How much of the tile one square takes, and how far each step of the stack moves.
const SIDE: f32 = 0.46;
const STEP: f32 = 0.075;

/// The rect the mark was last painted into (tests and screenshots).
fn rect_id() -> Id {
    Id::unique("rea-brand-mark-rect")
}

/// The three squares, back to front, inside a square tile `r`. Each is `SIDE` of the tile and each
/// step is `STEP` further in, so the stack is centred: the union of the three spans the same margin
/// on every side.
fn squares(r: Rect) -> [Rect; 3] {
    let side = r.width() * SIDE;
    let step = r.width() * STEP;
    // The stack's union is `SIDE + 2 * STEP` wide; its top-left is that much in from the centre.
    let first = r.center() - egui::vec2(side + 2.0 * step, side + 2.0 * step) * 0.5;
    std::array::from_fn(|i| Rect::from_min_size(first + egui::vec2(step, step) * i as f32, egui::vec2(side, side)))
}

/// The corner radius shared by the tile and its squares: a fixed fraction, so the mark looks the
/// same at 16 pt and at 512 px.
fn corner(r: Rect) -> u8 {
    (r.width() * 0.22).round().clamp(1.0, 255.0) as u8
}

/// Paint the mark into `r` (square).
pub fn paint_mark(ui: &Ui, r: Rect) {
    let t = crate::theme::Tokens::get(ui.ctx());
    let painter = ui.painter();
    painter.rect_filled(r, corner(r), t.accent);
    // A hairline in the accent's own border colour keeps the tile's edge from dissolving into a
    // light chrome, and it disappears in a dark one.
    painter.rect_stroke(r, corner(r), Stroke::new(1.0, t.accent_border), egui::StrokeKind::Inside);
    let [back, mid, front] = squares(r);
    let each = (r.width() * SIDE * 0.22).round();
    painter.rect_filled(back, each, Color32::WHITE.gamma_multiply(0.38));
    painter.rect_filled(mid, each, Color32::WHITE.gamma_multiply(0.62));
    painter.rect_filled(front, each, Color32::WHITE);
    ui.ctx().data_mut(|d| d.insert_temp(rect_id(), r));
}

/// The rect the mark was last painted into.
#[cfg(test)]
pub fn mark_rect(ctx: &Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(rect_id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The stack is centred, each square is inside the tile with a margin, and each step moves the
    /// same distance right and down: the mark reads the same at any size.
    #[test]
    fn the_three_squares_are_centred_and_inside_the_tile() {
        for size in [16.0, 20.0, 64.0, 512.0] {
            let r = Rect::from_min_size(pos2(10.0, 10.0), egui::vec2(size, size));
            let sq = squares(r);
            assert_eq!(sq.len(), 3);
            for s in sq {
                assert!(s.width() > 0.0 && (s.width() - s.height()).abs() < 1e-3, "{size}: square");
                assert!(r.contains_rect(s), "{size}: {s:?} escapes {r:?}");
            }
            // Back to front, each square is further right and down, and none is degenerate.
            assert!(sq[1].min.x > sq[0].min.x && sq[2].min.x > sq[1].min.x, "{size}: steps");
            assert!(sq[2].min.y > sq[1].min.y, "{size}: steps");
            // The union is centred on the tile.
            let union = sq[0].union(sq[1]).union(sq[2]);
            assert!((union.center().x - r.center().x).abs() < 0.51, "{size}: centred x");
            assert!((union.center().y - r.center().y).abs() < 0.51, "{size}: centred y");
        }
    }

    /// The mark is geometry: painting it must upload no texture and ship no bitmap, which is the
    /// point of replacing the artwork this fork started with.
    #[test]
    fn the_title_bar_paints_the_mark_without_uploading_any_texture() {
        let ctx = Context::default();
        crate::PhotocraftApp::setup_context(&ctx, crate::theme::ThemeKind::Ink);
        let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        // The system title bar hides the in-app mark (the OS draws its own icon), so paint the
        // app-drawn bar here.
        app.custom_titlebar = true;
        let mut frame = || {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
            let uploads = out.textures_delta.set.len();
            out.textures_delta.clear();
            uploads
        };
        assert_eq!(frame(), 0, "the mark is drawn, not uploaded");
        assert!(mark_rect(&ctx).is_some());
        assert_eq!(frame(), 0, "and it stays that way");
    }
}
