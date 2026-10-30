//! The About window's Licences tab: every open-source component in the app, what it is, the licence
//! it is distributed under, and that licence's full text.
//!
//! The texts are baked in with `include_str!` from the files in this repository, so the screen and
//! the source tree can never disagree about what is being shipped. That is the point: MIT and
//! Apache-2.0 both allow this app to be sold as closed source, and what they ask in exchange is
//! that the notice and the licence texts travel with it (Apache-2.0 §4(a)–(d)).
//!
//! Adding a dependency or an asset with its own licence means adding it here in the same change.

use egui::RichText;

/// What each component is, and the licence it is under. The texts below follow in full.
pub const COMPONENTS: &[(&str, &str, &str)] = &[
    ("PhotoCraft engine", "The editor's engine and user interface. This app is a fork of storytold/photocraft, modified.", "MIT OR Apache-2.0"),
    ("PhotoDroid", "The Android shell this app is built on. Idea-atmosphere/PhotoDroid.", "MIT OR Apache-2.0"),
    ("egui, eframe, epaint", "The Rust UI toolkit the interface is drawn with.", "MIT OR Apache-2.0"),
    ("wgpu, naga", "The GPU abstraction the canvas renders through.", "MIT OR Apache-2.0"),
    ("Inter", "The interface typeface.", "SIL Open Font License 1.1"),
    ("JetBrains Mono", "The typeface for numbers, measurements and code.", "SIL Open Font License 1.1"),
    ("Lucide", "The icon set, derived from Feather.", "ISC (Feather: MIT)"),
    ("SCOWL", "The English spelling dictionary used by the spell checker.", "Public domain / permissive"),
    ("Ubuntu font", "The display font egui's default text stack starts from.", "Ubuntu Font Licence 1.0"),
    ("Hack", "The monospace fallback in that same stack.", "MIT"),
    ("Noto Emoji", "Emoji in text.", "SIL Open Font License 1.1"),
    ("Emoji Icon Font, egui-icons", "The icon glyphs egui ships as a fallback font.", "MIT"),
    ("libfacedetection", "Face detection for the Liquify and healing tools.", "BSD 3-Clause"),
    ("X11 rgb.txt", "The named-colour table.", "MIT"),
    ("Solarized", "Ethan Schoonover's colour palette, one of the built-in themes.", "MIT"),
    ("Translations", "The interface translations contributed by the community.", "See the translations licence"),
    ("Application icon", "This app's own icon.", "This project"),
];

/// The licence texts, in full, in the order they appear on the screen.
pub const TEXTS: &[(&str, &str)] = &[
    ("MIT License (PhotoCraft)", include_str!("../../../LICENSE-MIT")),
    ("Apache License 2.0 (PhotoCraft)", include_str!("../../../LICENSE-APACHE")),
    ("PhotoCraft NOTICE", include_str!("../../../NOTICE")),
    ("Attribution (per-asset authors and licences)", include_str!("../../../ATTRIBUTION.md")),
    ("SIL Open Font License 1.1 — Inter", include_str!("../../../assets/fonts/OFL-Inter.txt")),
    ("SIL Open Font License 1.1 — JetBrains Mono", include_str!("../../../assets/fonts/OFL-JetBrainsMono.txt")),
    ("Ubuntu Font Licence 1.0", include_str!("../../../assets/fonts/bundled/LICENSE-Ubuntu-font-1.0.txt")),
    ("SIL Open Font License 1.1 — Noto Emoji", include_str!("../../../assets/fonts/bundled/LICENSE-OFL-1.1-NotoEmoji.txt")),
    ("Hack font", include_str!("../../../assets/fonts/bundled/LICENSE-Hack.txt")),
    ("Emoji Icon Font (MIT)", include_str!("../../../assets/fonts/bundled/LICENSE-emoji-icon-font.txt")),
    ("egui-icons", include_str!("../../../assets/fonts/bundled/README-egui-icons.txt")),
    ("ISC License — Lucide icons", include_str!("../../../assets/icons/LICENSE-lucide.txt")),
    ("Magnetic lasso icon", include_str!("../../../assets/icons/LICENSE-noun-magnetic-lasso.txt")),
    ("Symmetry axes icon", include_str!("../../../assets/icons/LICENSE-symmetry-axes.txt")),
    ("BSD 3-Clause — libfacedetection", include_str!("../../algo/data/LICENSE-libfacedetection.txt")),
    ("SCOWL dictionary", include_str!("../../../assets/dict/LICENSE-SCOWL.txt")),
    ("Application icon", include_str!("../../../assets/app-icon/LICENSE.txt")),
    ("MIT — Solarized palette", include_str!("LICENSE-solarized.txt")),
    ("Translations", include_str!("i18n/LICENSE-translations.txt")),
];

/// The Licences tab: the statement of changes, what is in the app, and each licence in full.
pub fn ui(ui: &mut egui::Ui) {
    ui.label(RichText::new("Open-source software in this app").strong());
    ui.add_space(2.0);
    ui.weak(
        "REA Image Studio is built on open-source software, and this screen is the notice it owes \
         those authors. Every licence below is reproduced in full.",
    );
    ui.add_space(6.0);
    // Apache-2.0 §4(b): say that the work was modified, and what was done to it.
    ui.label(
        RichText::new(
            "Changes: this app is a modified fork of PhotoCraft and PhotoDroid. The engine is \
             theirs, with our own theme, branding, Android packaging and bug fixes on top. The \
             upstream copyright lines are kept in every source file and in the texts below.",
        )
        .small(),
    );
    ui.add_space(8.0);

    ui.label(RichText::new("Components").strong());
    ui.add_space(2.0);
    egui::ScrollArea::vertical().max_height(150.0).id_salt("licence-components").show(ui, |ui| {
        egui::Grid::new("licence-components-grid").num_columns(3).striped(true).spacing([10.0, 4.0]).show(ui, |ui| {
            for (name, what, licence) in COMPONENTS {
                ui.label(RichText::new(*name).small().strong());
                ui.label(RichText::new(*what).small());
                ui.label(RichText::new(*licence).small());
                ui.end_row();
            }
        });
    });
    ui.add_space(10.0);

    ui.label(RichText::new("Licence texts").strong());
    ui.add_space(2.0);
    egui::ScrollArea::vertical().max_height(260.0).id_salt("licence-texts").show(ui, |ui| {
        for (title, text) in TEXTS {
            egui::CollapsingHeader::new(RichText::new(*title).small()).id_salt(*title).show(ui, |ui| {
                if ui.button(RichText::new("Copy").small()).clicked() {
                    ui.ctx().copy_text((*text).to_string());
                }
                ui.add(
                    egui::Label::new(RichText::new(*text).font(crate::theme::mono(11.0)))
                        .selectable(true)
                        .wrap(),
                );
            });
        }
    });
    ui.add_space(6.0);
    ui.weak("Full licence files: LICENSE-MIT, LICENSE-APACHE, NOTICE and ATTRIBUTION.md in the project repository.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The screen is only a notice if the texts are really there: every entry must carry text, the
    /// two engine licences must be the real ones, the upstream notice must name upstream, and each
    /// third-party asset we ship must be listed with a licence.
    #[test]
    fn every_licence_text_is_present_and_the_notice_names_upstream() {
        for (title, text) in TEXTS {
            assert!(!title.is_empty());
            assert!(text.len() > 200, "{title}: {} bytes is too short to be a licence", text.len());
        }
        let get = |title: &str| TEXTS.iter().find(|(t, _)| *t == title).map(|(_, text)| *text).unwrap_or_default();
        assert!(get("MIT License (PhotoCraft)").contains("MIT License"));
        assert!(get("Apache License 2.0 (PhotoCraft)").contains("Apache License"));
        assert!(get("Apache License 2.0 (PhotoCraft)").contains("Version 2.0"));
        let notice = get("PhotoCraft NOTICE");
        assert!(notice.contains("PhotoCraft"), "the upstream NOTICE must be reproduced");
        assert!(notice.contains("ArtCraft") || notice.contains("photocraft"), "the upstream NOTICE names its owner");
        assert!(get("SIL Open Font License 1.1 — Inter").contains("SIL OPEN FONT LICENSE"), "OFL 1.1 for Inter");
        assert!(get("ISC License — Lucide icons").to_lowercase().contains("permission to use, copy"));
        // The Ubuntu font is bundled inside the binary by epaint; its licence must travel with it.
        assert!(get("Ubuntu Font Licence 1.0").contains("UBUNTU FONT LICENCE"));
        assert!(get("SIL Open Font License 1.1 — Noto Emoji").contains("SIL Open Font License"));
        assert!(get("Hack font").to_lowercase().contains("hack project"));

        // Nothing ships without a stated licence, and the two engine components say where they
        // came from: that is the attribution a store listing points at.
        for (name, what, licence) in COMPONENTS {
            assert!(!name.is_empty() && !licence.is_empty(), "{name} has no licence");
            assert!(what.len() > 10, "{name} is not described");
        }
        assert!(COMPONENTS.iter().any(|(n, _, l)| *n == "PhotoCraft engine" && l.contains("Apache-2.0")));
        assert!(COMPONENTS.iter().any(|(n, _, l)| *n == "PhotoDroid" && l.contains("Apache-2.0")));
    }
}
