# REA Image Studio

A professional raster image editor for Android, with a layer-based document model and native
PSD/PSB support. Built in Rust and shipped as a single arm64 APK.

REA Image Studio is a fork of [PhotoDroid](https://github.com/Idea-atmosphere/PhotoDroid), which
ports the [PhotoCraft](https://github.com/storytold/photocraft) engine to Android. The engine and
the egui interface are shared with the desktop editor; this fork supplies its own application
identity, launcher icon, storage layout and documentation. Both upstream projects are
MIT OR Apache-2.0 — see [Provenance and licence](#provenance-and-licence).

## Status

**Built and packaged by CI; not yet run on a physical device.** I am being precise about that on
purpose, because "it compiles" and "it works on your tablet" are different claims:

| Verified | How |
|---|---|
| The Rust workspace compiles for `aarch64-linux-android` | GitHub Actions, `cargo build --profile android` |
| The host-side code (save protocol, crash recovery, file naming) passes its tests | `cargo test -p photocraft-android` in CI |
| The APK is produced, packaged for `arm64-v8a`, with our `applicationId` and label | `aapt2 dump badging` in CI |
| The native library exports `android_main` and `ANativeActivity_onCreate` | `llvm-readelf --dyn-syms` in CI |
| The Java entry point and the JNI method the Rust side calls are in the dex | `strings` over `classes.dex` in CI |

| **Not verified** | Why |
|---|---|
| Rendering, touch input, the file picker, saving to `Downloads` | No Android device or emulator was available to this build. Static inspection cannot prove runtime behaviour. |
| Stylus pressure/tilt, soft keyboard behaviour | Same — device-dependent. |
| Performance on large documents | Same. Upstream reports the phone canvas composites on the CPU; a desktop-sized document will be slow. |

The honest summary: **you get an installable APK, and every claim about its contents is backed by a
CI artifact.** Claims about how it feels in your hand are not backed by anything yet. Your tablet is
the test rig.

## Features

The editor targets Photoshop-class workflows. These are implemented in the engine this app ships:

- **Documents** — layers, groups, layer masks, clipping and linked layers, smart objects
- **Painting** — brushes with size/flow/hardness presets, eraser, clone and healing tools
- **Selection** — rectangular/elliptical/lasso/magnetic lasso, quick selection, feathering, grow/shrink
- **Transform** — free transform, warp, perspective, rotate/flip, non-destructive transforms
- **Blending** — the standard Photoshop blend-mode set, opacity and fill
- **Text** — point and paragraph type, font/kerning/leading/tracking, OpenType features, styles
- **Vector** — pen, shapes, paths, strokes, fills, boolean operations
- **Adjustments** — levels, curves, hue/saturation, colour balance, black & white, gradient map,
  selective colour, shadows/highlights, exposure, vibrance, and more
- **Filters** — blur gallery, sharpen, noise, distort, stylise, pixelate, and smart filters
- **History** — unbounded undo/redo with a history panel
- **Files** — open and save PSD/PSB, PNG, JPEG, TIFF, WebP, GIF, BMP, TGA; export; Affinity files

Feature parity is the upstream engine's, and this fork has not removed any of it. What this fork
changes is listed in [docs/FORK.md](docs/FORK.md). Upstream's own list of limitations — and they are
substantial and specific — is preserved in
[docs/upstream/README-photodroid.md](docs/upstream/README-photodroid.md#known-limitations). Read it
before you plan a workflow around this app.

## Install

Download `REAImageStudio-debug.apk` from the latest [release](../../releases) or from a successful
CI run's **Artifacts**, then:

```sh
adb install -r REAImageStudio-debug.apk
```

Requirements: Android 10 (API 29) or newer, arm64-v8a CPU. There are **no storage permissions** —
files are read through the system picker and written through MediaStore into
`Downloads/REAImageStudio/`.

### Signing

The CI artifact is **debug-signed** with the key Gradle generates per runner. That installs fine and
is reproducible, but: a new build has a different key, so installing over the previous version
requires uninstalling first, which deletes the app's saved data and preferences. A release keystore
is a follow-up; `android/app/build.gradle` already reads `KEYSTORE_PATH`, `STORE_PASSWORD`,
`KEY_ALIAS` and `KEY_PASSWORD` from the environment when they are present.

## Build

The build has two stages: Cargo compiles the Rust library, Gradle packages it into the APK.

```sh
# 1. The native library (NDK 27.2.12479018, Rust 1.95+, aarch64-linux-android target)
export NDK="$ANDROID_HOME/ndk/27.2.12479018"
export BIN="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$BIN/aarch64-linux-android29-clang"
export CC_aarch64_linux_android="$BIN/aarch64-linux-android29-clang"
export CXX_aarch64_linux_android="$BIN/aarch64-linux-android29-clang++"
export AR_aarch64_linux_android="$BIN/llvm-ar"

cargo build -p photocraft-android --lib --target aarch64-linux-android --profile android
mkdir -p android/app/src/main/jniLibs/arm64-v8a
cp target/aarch64-linux-android/android/libphotocraft_android.so android/app/src/main/jniLibs/arm64-v8a/

# 2. The APK (JDK 17, SDK platform 35, build-tools 35.0.0)
gradle -p android :app:assembleDebug
```

CI does exactly this in [.github/workflows/android.yml](.github/workflows/android.yml), then
inspects the result and uploads it. Pushing a tag `v*` publishes it as a release.

## Project layout

| Path | What it is |
|---|---|
| `crates/` | The shared PhotoCraft engine and egui UI: document model, compositing, PSD codecs, text, vector, colour management |
| `apps/photocraft-android/` | The Android shell: a Rust `cdylib` run by `android-native-activity`, plus its JNI bridge and storage services |
| `android/` | The Gradle project: `MainActivity` (Java), manifest, launcher icon |
| `assets/` | Fonts, icons and dictionary embedded at build time, each with its licence |
| `docs/` | Fork notes, status, and the upstream READMEs kept for provenance |
| `ATTRIBUTION.md` | Per-file third-party attribution (inherited, unchanged) |

## Provenance and licence

This fork is distributed under **MIT OR Apache-2.0**, the same terms as both upstreams.
`LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` and `ATTRIBUTION.md` are kept, with an added fork section
in `NOTICE`. Upstream's own READMEs are preserved in `docs/upstream/`. Every third-party asset keeps
its original licence; the PhotoCraft/PhotoDroid names and logos are their owners' trademarks and are
not used as this app's identity. See [docs/FORK.md](docs/FORK.md) for the exact list of changes.
