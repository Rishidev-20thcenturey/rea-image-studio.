# What this fork changes

REA Image Studio is a fork of [PhotoDroid](https://github.com/Idea-atmosphere/PhotoDroid) at its
`main` (cloned 2026-10-29, commit `8b20475a295fa3ed837ac8bed8ba1555386a73ab`), which in turn is an Android port of
[PhotoCraft](https://github.com/storytold/photocraft) at upstream `cece50d5`.

The engine, the UI and the Android shell are upstream's code. The first revision of this fork
changed **identity and packaging only**, because the engine is ~2,500 files of Rust and rebranding
is the change that can be verified by reading it. Later revisions (see
[`PLAN-B.md`](PLAN-B.md)) add the product's own appearance: the **REA Ink** light and **REA Ink
Dark** themes, a restyled phone panels sheet, and an in-app Licences screen. Editing behaviour is
still upstream's; the theme and the notice are ours.

## Application identity

| Before (PhotoDroid) | Now (REA Image Studio) |
|---|---|
| `applicationId` / `namespace` `IA.photodroid` | `com.rea.imagestudio` |
| Java class `IA.photodroid.PhotoCraftActivity` | `com.rea.imagestudio.MainActivity` |
| App label `PhotoDroid` | `REA Image Studio` |
| Window/UI title `PhotoDroid` | `REA Image Studio` |
| Launcher icon: PhotoDroid's raster mipmaps + SVG | Original vector adaptive icon (this repo) |
| Start-page button → Telegram channel | Start-page button → this repository |
| About/Help GitHub + Issues links → `Idea-atmosphere/PhotoDroid` | → `Rishidev-20thcenturey/rea-image-studio.` |
| Save folder `Downloads/PhotoCraft` | `Downloads/REAImageStudio` |
| Default document name `PhotoCraft` | `REA Image Studio` |
| Preferences file `photodroid_prefs` | `rea_image_studio_prefs` |
| First-run card: donation link to the porter | First-run card: what the app is, its licences and attribution, link to this repository |

The `photocraftCall` JNI method name and the `libphotocraft_android.so` library name are unchanged:
they are internal ABI between the Java shell and the Rust library, not branding, and renaming them
would risk a runtime-only failure that no test here can catch.

## Kept on purpose

- `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` (with an added fork section), `ATTRIBUTION.md`,
  `assets/*/LICENSE-*` and the translation catalogue's licence — all unchanged.
- Upstream's copyright headers in every source file.
- The i18n catalogues: the `PhotoDroid` key was renamed to `REA Image Studio` in the code *and* in
  all 20 catalogues, so translations stay reachable.
- Upstream's READMEs, in `docs/upstream/`, as the authoritative description of the engine and of
  its known limitations.
- `crates/ui-egui/src/credits.rs` and `contributors/`: the people who wrote this engine stay
  credited in the app's About dialog.

## Removed

- `.github/workflows/release.yml` — invoked PhotoDroid's own signing secrets and produced
  PhotoDroid-named assets; replaced by the `release` job in `.github/workflows/android.yml`.
- `.github/CODEOWNERS` — named upstream's maintainer as the owner of paths in *this* repository.
- `.github/dependabot.yml` — dependency PRs against a vendored, revision-pinned tree are noise.
- `android/app/src/main/res/mipmap-*/ic_launcher.png` and `android/icon/ic_launcher.svg` —
  upstream's icon artwork, replaced by this fork's own icon. `minSdk` is 29, so an adaptive icon
  needs no raster fallback.

## Our own appearance (branch `arena/9b1fa066`)

| Area | What changed | Where |
|---|---|---|
| Palette | `REA Ink` and `REA Ink Dark`: warm paper-white surfaces, near-black ink type, one saturated ink-blue accent, 8/12/18 px radii, no bevels | `crates/ui-egui/src/theme.rs` |
| Preference wiring | Both are saved themes with their own slots in the appearance system, first in the picker | `crates/engine/src/prefs.rs`, `crates/ui-egui/src/prefs_ui.rs` |
| First run | The app writes the two themes once, behind a marker file, instead of forcing a theme on every launch | `apps/photocraft-android/src/shell.rs` |
| Phone chrome | The panels sheet rises as a card: rounded top, hairline edge, grab handle. Touch targets and geometry unchanged | `crates/ui-egui/src/mobile.rs` |
| Notice | About ▸ Licences: components and full licence texts, baked in from this repository at build time | `crates/ui-egui/src/licences.rs` |
| Gate | `deny.toml` + `.github/workflows/deny.yml` fail the build on a non-permissive licence or an unexpected git source | `deny.toml` |

The **icon set is still Lucide** (ISC), listed on the Licences screen: swapping 169 SVGs is asset
work that is deliberately deferred until the palette is settled on a real screen.

## The brand mark

The first revision of this fork painted **upstream's kitsune app icon** at the left of the title bar
(`crates/ui-egui/src/brand.rs`). It is licensed MIT/Apache-2.0 by its own `LICENSE.txt`, so that was
legal — but it is *their* mark, and this is not their app.

It is now **drawn, not shipped**: `brand.rs` paints three stacked rounded squares on a tile in the
theme's accent colour, with no bitmap and no upstream artwork compiled into the binary. The Android
launcher icon (`ic_launcher_foreground.xml`, `ic_launcher_background.xml`) uses the same construction
in the same colours, so the icon on the home screen and the tile inside the app are one mark, and it
recolours with the theme (ink blue in REA Ink, lifted in REA Ink Dark).

`assets/app-icon/` still holds upstream's desktop icon artwork. Nothing in the Android app reads it
— the desktop application's build scripts do — so it is kept for the tree's sake, under the licence
it came with, and it is **not** part of what this app ships.
