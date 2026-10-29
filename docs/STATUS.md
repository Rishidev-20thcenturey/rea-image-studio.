# Status and verification log

This file records what has actually been checked, when, and what has not. It is updated as evidence
arrives; an entry is only added after the result exists.

## Environment this project is developed in

There is **no Android toolchain in the development container** — no JDK, no Android SDK, no Android
NDK and no Rust compiler, and its network egress allows only GitHub, npm and PyPI (not `dl.google.com`
or Maven Central). GitHub Actions is therefore the only build path, and it is the only place an APK
has ever been produced from this repository.

There is also **no Android device or emulator** available, and no KVM to run one. Everything below
that says "verified" was verified statically, from the packaged artifact, or by host test — never by
launching the app.

## Verification matrix

| Claim | Evidence | Status |
|---|---|---|
| The workspace cross-compiles for `aarch64-linux-android` | CI job `apk`, `cargo build --profile android` | see [CI runs](#ci-runs) |
| Host-side code passes its tests (save protocol, file naming, crash recovery, diagnostic log) | CI job `tests` | see [CI runs](#ci-runs) |
| The APK carries `applicationId com.rea.imagestudio`, label "REA Image Studio", `minSdk 29`, `arm64-v8a` | CI `aapt2 dump badging` | see [CI runs](#ci-runs) |
| The native library exports `android_main` and `ANativeActivity_onCreate` | CI `llvm-readelf --dyn-syms` | see [CI runs](#ci-runs) |
| The Rust→Java entry point (`photocraftCall`) and `com/rea/imagestudio/MainActivity` are in the dex | CI `strings` over `classes.dex` | see [CI runs](#ci-runs) |
| The app launches, renders, accepts touch, opens files, saves to Downloads | **no evidence — needs a device** | ✗ not verified |
| Stylus pressure/tilt, soft keyboard, GPU path | **no evidence — needs a device** | ✗ not verified |

## CI runs

| Run | Result | APK |
|---|---|---|
| [first build](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions) | pending | pending |

## Known limitations

Inherited from upstream, and true of this fork because the shell and engine are the same code:

- **arm64-v8a only.** The APK contains one ABI. x86_64 tablets and emulator images cannot run it.
- **Save always writes to `Downloads/REAImageStudio/`.** There is no location picker; a document
  opened from elsewhere is re-saved as a new file there rather than written back.
- **Placing files, importing scripts/presets/notes** need a synchronous file picker, which Android
  does not offer; those menu items report that and do nothing.
- **The canvas composites on the CPU on phones.** Edits on large documents are slow, and a document
  larger than 4096 px is shown scaled down.
- **No clipboard images, no stylus tilt, no display colour-profile handling.**
- **Layers, brushes and groups cannot be dragged to reorder** in the phone layout; use
  Menu › Layer › Arrange.
- **TalkBack is unsupported** — AccessKit is off on Android, because eframe does not allow it with
  the NativeActivity backend.
- **The debug-signed APK changes key between CI runs**, so installing a new build over an old one
  requires an uninstall, which deletes app data.

Added by this fork:

- **No device test has been performed.** Upstream is in the same position; neither project has
  evidence of runtime behaviour on hardware.
- **`assets/app-icon/` still contains PhotoCraft's artwork** (correctly attributed, but not this
  app's identity). Replacing it is open work.
- **Translations of the two new UI strings** ("REA Image Studio" in the About blurb and the
  start-page button) fall back to English in the 20 non-English catalogues until they are translated.

## How to verify on your own hardware

```sh
adb install -r REAImageStudio-debug.apk
adb logcat -s REAImageStudio:V            # the shell logs under this tag
```

The app writes a diagnostic log to `Downloads/REAImageStudio/` (Help › Save Diagnostic Log saves it
immediately). Send that file along with any bug report — it records each open, each text edit and any
panic, which is the difference between "it crashed" and a fixable bug.
