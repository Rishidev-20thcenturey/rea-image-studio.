# The from-scratch app (`studio/`)

REA Image Studio also exists as an app written from nothing: **Kotlin UI + my own Rust image core,
called over JNI.** No code, asset or design is taken from PhotoCraft, PhotoDroid or any other
editor. The fork that ships as v1.0.0 keeps its own history and its own licences; this document is
about `studio/` only.

| | v1.0.0 (the fork) | this app |
|---|---|---|
| Where | `android/`, upstream Rust workspace | `studio/core` (Rust), `studio/app` (Kotlin) |
| Who wrote the engine | PhotoCraft / PhotoDroid | this repository |
| applicationId | `com.rea.imagestudio` | `com.rea.studio` |
| Package size | 68 MiB | ~2 MiB |
| Feature depth | desktop-class | a focused editor (see the table below) |

## What it is

A single-activity Android editor. The screen is the document: no action bar, no permanent sidebars,
system bars hidden with a swipe to reveal. A floating rail on the left selects the tool, a contextual
strip along the bottom shows that tool's own controls, the top bar holds undo/redo, the layer panel
and the command menu, and a colour swatch opens a hue/saturation picker. The layers panel slides in
over the canvas and carries live thumbnails, visibility, blend mode, opacity and mask operations.

The core is a plain Rust library with no Android dependency beyond the JNI bindings, so its whole
behaviour is testable on the host. It owns the pixels; the UI owns nothing but the view transform and
the touch state. That is why the unit tests below can speak for the editor's actual image operations.

## Architecture

```
studio/core/src/
  blend.rs      16 blend modes (normal, multiply, screen, overlay, darken, lighten, colour dodge,
                colour burn, hard light, soft light, difference, exclusion, hue, saturation,
                colour, luminosity), W3C compositing maths
  doc.rs        Document, Layer, Rect, selections, undo/redo, the .rea file format
  ops.rs        painting, shapes, bucket fill, selections, adjustments, filters, transforms
  jni_bridge.rs the 56 Kotlin-facing entry points (Android only)
  lib.rs        version, capability description, host tests
studio/app/src/main/kotlin/com/rea/studio/
  NativeCore.kt every JNI entry point, one to one
  EditorView.kt the canvas: fit/zoom/pan, rendering, touch → core calls
  MainActivity.kt the chrome: rail, top bar, options strip, layers panel, menus, dialogs, file I/O
  Widgets.kt    the drawn icons, colour picker and slider row (no image assets ship)
```

Design points worth knowing:

- **Straight (non-premultiplied) RGBA8**, `Arc<Vec<u8>>` per layer. An undo snapshot is a pointer
  copy; a layer buffer is cloned only when a stroke first writes to it.
- **Dirty-rectangle composite cache.** A stroke marks the rectangles it touched; rendering only
  recomposites those.
- **Bounded history.** 40 steps, trimmed further to 192 MiB of pixel data so a large document cannot
  exhaust memory.
- **Straight-through rendering.** The view asks the core for exactly the visible region at the
  current zoom into one reused bitmap; a stroke allocates no frames.
- **`.rea` documents** — magic `REASTUDIO1`, a text manifest, then per-layer pixel and mask blocks
  with RLE compression. Save/load round-trips are unit tests.

## Features, with the evidence for each

Everything in this table is a control in the app that calls a real core operation. "Host tests"
means a `#[test]` in `studio/core` that asserts the pixels; "CI" means it was exercised by the
workflow listed at the bottom.

| Area | What it does | Evidence |
|---|---|---|
| Layers | add, add-from-pixels, delete, duplicate, merge down, move up/down, rename, visibility, opacity, 16 blend modes | host tests (`merge_down_folds_the_upper_layer_in`, blend-mode tests) + CI |
| Layer masks | mask from the current selection; masks composite into the render | host tests |
| Brush | size, hardness, flow, opacity, pressure-scaled radius, spacing-correct interpolation between touch samples | host tests |
| Eraser | the same stroke path, removing alpha | `the_eraser_removes_alpha` |
| Shapes | rectangle and ellipse drag previews that follow the finger, including shrinking | CI build; preview logic is view-side |
| Bucket fill | flood fill from the tapped pixel with a tolerance | host tests |
| Selections | rectangle, ellipse, lasso, all, none, invert, feather, crop to selection, fill, clear, to mask | `rect_and_ellipse_selections_cover_expected_pixels`, `painting_outside_the_selection_does_nothing` |
| Adjustments | brightness/contrast, levels, curves (gamma+lift), hue/saturation, threshold, invert, desaturate, auto levels, temperature | `levels_lift_the_black_point`, `threshold_is_black_or_white_only`, `invert_flips_the_channels`, `hue_rotation_changes_the_colour_but_not_the_alpha` |
| Filters | gaussian blur, unsharp mask, noise, pixelate | `pixelate_makes_neighbours_identical`, `sharpen_keeps_the_image_readable` |
| Transforms | flip horizontal/vertical, rotate 90°, move the active layer, resize canvas, crop | `flip_horizontal_mirrors_pixels`, `rotate90_swaps_the_layer_dimensions`, `resize_canvas_scales_the_layer_buffers` |
| History | undo/redo with the name of the step, the memory cap above | host tests + `a_full_edit_session_behaves_end_to_end` |
| Files | open PNG/JPEG through the system picker, export PNG/JPEG, save/open `.rea` | `.rea` round-trip tests; the picker is UI-side |
| Text | the platform's text engine rasterises the typed text into a new layer | not covered by a test; it is a one-way import of a rendered bitmap |

`cargo test` in `studio/core` runs **56 tests**; all of them pass in CI.

### What has not been verified, and one attempt to fix that

**The app has never been launched.** The CI runner this project builds on has not produced a booted
emulator in four attempts: a hand-rolled `avdmanager` + `adb` boot, the maintained
`ReactiveCircus/android-emulator-runner` action with hardware acceleration at API 34 and again at API
30 with a longer budget, and finally the same action at API 29 asking for software emulation
explicitly. The first two timed out waiting for a booted device; the last failed before the device
appeared. The machine that runs these jobs can build and inspect an APK but will not boot one. Each
attempt's run is recorded in [`docs/ci/studio-emulator/`](ci/studio-emulator/). So the honest
statement is:

- The engine's behaviour is covered by 56 host tests: it is the part I can prove.
- The APK's **contents** are inspected byte for byte in CI (identity, ABI, library, JNI symbols,
  classes in dex).
- The **interface** — touch handling, view transforms, the panels — is the part with no test behind
  it. It compiles, it is statically consistent with the core's 56 entry points, and it is the first
  thing to look at on a device.

If you install the APK and something is wrong, that failure is not a surprise this document hid.

## What it deliberately does not do

Stated plainly, because the alternative is a product that looks more capable than it is:

- **No PSD/PSB, no Camera Raw, no TIFF/WebP.** `.rea`, PNG and JPEG only. The fork is the app for
  PSD.
- **No editable text.** Typed text is rasterised into a pixel layer immediately, and that layer is
  ordinary pixels afterwards. No font picker, no re-editing.
- **No adjustment layers, smart objects, smart filters, layer groups or clipping masks.** Edits are
  applied to pixels; the history is the way back.
- **No GPU.** Compositing, blending and filters are CPU code. A 2048×1536 document is comfortable,
  the maximum is 8192 px per side, and a full-canvas 60 px blur on a large document will feel slow.
- **No pen tilt, no colour management, no CMYK, no non-destructive crop.**
- **One ABI per build.** The released APK is `arm64-v8a`; the emulator build is `x86_64`.
- **The layer panel reorders with up/down buttons, not a drag gesture.** Dragging a row is not
  implemented; the buttons are, and they move the active layer one place per tap.
- **The legacy fork is still the feature-rich app.** This one is the app I can defend line by line.

## Building it

Everything is driven by GitHub Actions (the development container has no Android toolchain):

- `.github/workflows/studio-core.yml` — `cargo test --release` on the host, then a cross-compile of
  the arm64 library with a check that the JNI surface is actually exported.
- `.github/workflows/studio.yml` — builds `librea_core.so` for `arm64-v8a`, puts it in
  `studio/app/src/main/jniLibs/`, builds the APK with Gradle, inspects it with `aapt2`,
  `llvm-readelf` and the dex files, and publishes `docs/ci/studio-build.md`.
- `.github/workflows/studio-smoke.yml` — an x86_64 build, installed on an emulator, launched, drawn
  on with real touch events, screenshotted; evidence in `docs/ci/studio-emulator/`.

Locally, with an SDK/NDK and Gradle 8.10+ installed:

```bash
cd studio/core
cargo build --release --target aarch64-linux-android     # needs the NDK linker in your env
cd ..
mkdir -p app/src/main/jniLibs/arm64-v8a
cp core/target/aarch64-linux-android/release/librea_core.so app/src/main/jniLibs/arm64-v8a/
gradle -p . :app:assembleDebug
```

## Licence and provenance

The code in `studio/` is this repository's own work, written for this project: the Rust core, the
Kotlin UI, the drawn icon set, the CI. No third-party source, font, icon or artwork was copied into
it. Android, Kotlin, Gradle and the Rust `jni` crate are used under their own licences; the app
depends on no other libraries at runtime.

The fork in `android/` is a different matter, documented in [FORK.md](FORK.md) and in the README's
"Provenance and licence" section: it is upstream code under MIT OR Apache-2.0, and it says so.
