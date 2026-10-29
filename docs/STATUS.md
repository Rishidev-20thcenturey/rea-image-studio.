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

## The from-scratch app (`studio/`)

The app described in the matrix below is the fork. The original app in [`studio/`](STUDIO.md) has its
own evidence, written by its own workflows, and its own limitations:

| Claim | Evidence | Status |
|---|---|---|
| The core's image operations behave as specified | `cargo test --release` in `studio/core`: 56 tests, all passing ([CI](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/workflows/studio-core.yml)) | ✅ verified |
| It cross-compiles for `aarch64-linux-android` and `x86_64-linux-android` | NDK clang builds in `studio-core.yml` and `studio-smoke.yml` | ✅ verified |
| The JNI surface is complete and exported | 56 `Java_com_rea_studio_NativeCore_*` symbols in the packaged library | ✅ verified |
| The APK is ours and installable in shape | `package: name='com.rea.studio'`, label, `arm64-v8a`, `classes.dex` carries `MainActivity`/`NativeCore`/`EditorView` — see [`docs/ci/studio-build.md`](ci/studio-build.md) | ✅ verified |
| The app launches, paints, and does not crash | four emulator smoke attempts (hand-rolled AVD; emulator action with acceleration; the same with software emulation) all timed out before a booted device — logs in [`docs/ci/studio-emulator/`](ci/studio-emulator/) | ❌ not verified |

## Verification matrix

First green build: [run 40865238933](https://github.com/Rishidev-20thcenturey/rea-image-studio./actions/runs/40865238933)
(2026-10-29), commit `6c808cd`. The full report the build wrote about itself is in
[`docs/ci/last-build.md`](ci/last-build.md) and is updated by every run.

| Claim | Evidence (verbatim from the build) | Status |
|---|---|---|
| The workspace cross-compiles for `aarch64-linux-android` | `cargo build --profile android` succeeded | ✅ verified |
| Host-side code passes its tests | CI job `Host tests` green: save protocol, file naming, crash recovery, diagnostic log | ✅ verified |
| The APK carries our identity | `package: name='com.rea.imagestudio' versionCode='1' versionName='1.0.0'` | ✅ verified |
| Our activity is the launcher | `launchable-activity: name='com.rea.imagestudio.MainActivity'` | ✅ verified |
| The app is branded | `application-label:'REA Image Studio'` | ✅ verified |
| The APK targets a real device | `minSdk 29` (Android 10), `targetSdkVersion:'35'`, `native-code: 'arm64-v8a'` | ✅ verified |
| The native library is packaged and loadable | `lib/arm64-v8a/libphotocraft_android.so` with `android_main` and `ANativeActivity_onCreate` exported | ✅ verified |
| The Rust→Java bridge survived the rebrand | `photocraftCall` found in `classes2.dex` | ✅ verified |
| The Java shell compiled | `Lcom/rea/imagestudio/MainActivity;` in `classes2.dex` | ✅ verified |
| **The app launches, renders, accepts touch, opens files, saves** | **no evidence — needs a device or emulator** | ❌ not verified |
| Stylus pressure/tilt, soft keyboard, GPU path | no evidence | ❌ not verified |

A note on the first two failed runs, since "it failed then passed" deserves an explanation: the
first failure was a faulty check of mine — the app is **multidex** (`MainActivity` lands in
`classes2.dex`), and the check only scanned `classes.dex`. The build itself was never broken. The
check now enumerates every dex entry and runs `dexdump` over each, so a packaging regression cannot
hide behind the same mistake.

### How this was verified without a device

The build inspects its own artifact with `aapt2 dump badging`, `unzip`, `llvm-readelf --dyn-syms`
and `dexdump`, writes the result to `docs/ci/last-build.md`, and fails the job if any required fact
is missing. That is strong evidence that the APK *is what it claims to be*. It is not evidence that
the program behaves correctly when started — for that, see "How to verify on your own hardware".

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
