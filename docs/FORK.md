# What this fork changes

REA Image Studio is a fork of [PhotoDroid](https://github.com/Idea-atmosphere/PhotoDroid) at its
`main` (cloned 2026-10-29, commit `8b20475a295fa3ed837ac8bed8ba1555386a73ab`), which in turn is an Android port of
[PhotoCraft](https://github.com/storytold/photocraft) at upstream `cece50d5`.

The engine, the UI and the Android shell are upstream's code. This fork changes **identity and
packaging only**; no editing behaviour was altered in this revision. That is deliberate: the
engine is ~2,500 files of Rust, and rebranding is the change that can be verified by reading it.

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

## Third-party identity not yet replaced

`assets/app-icon/` and `assets/brand/` still hold PhotoCraft's icon artwork, embedded at build time
for the app-icon service. It is redistributed under MIT/Apache-2.0 with attribution as before, but it
is **not** this app's launcher icon. Replacing it is follow-up work; it is listed here so the
repository does not imply the artwork is original.
