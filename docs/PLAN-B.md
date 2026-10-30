# Route B — ship the restyled fork as the product

Decided by the owner. The from-scratch app (`studio/`, currently 0.1.7) stays alive as its own
line; the paid Play Store product is built on this fork's engine, with a UI and identity that are
ours and visibly different from upstream. This document is the plan and the evidence for it.

Nothing in here is claimed as done until it is checked on a device or in CI.

## 1. Where the fork stands today

Proven by reading the tree (commit `3ffcf2d` + this branch):

| Area | State | Evidence |
|---|---|---|
| Engine (layers, masks, brushes, PSD/PSB, adjustments, filters) | Upstream, complete | `crates/` — 26 crates, ~2,500 files |
| Android shell | Upstream PhotoDroid, rebranded | `android/app/src/main/java/com/rea/imagestudio/MainActivity.java` |
| App name / package | Ours | `REA Image Studio`, `com.rea.imagestudio` |
| Launcher icon | Ours (vector adaptive) | `android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml` |
| First-run licence card | Ours | `android/app/src/main/res/values/strings.xml` (`welcome_message`) |
| Licence texts + NOTICE + ATTRIBUTION | Kept, with a fork section | `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md`, `docs/FORK.md` |
| **Editor visual language** | **Upstream** (Photoshop-style charcoal, `ThemeKind::Pro`) | `crates/ui-egui/src/theme.rs`, `apps/photocraft-android/src/shell.rs:52` |
| **Icon set** | **Upstream** (169 Lucide-derived SVGs) | `assets/icons/` |
| Phone layout | Upstream, written and unit-tested | `crates/ui-egui/src/mobile.rs` |

So: the *identity* is already ours (name, icon, links, licence card). The *look inside the
editor* is still a Photoshop clone. That is the gap Route B closes.

## 2. Question 1 — can it be used on a phone?

Yes — a phone layout already exists in the engine; it is not something we have to write.

- `crates/ui-egui/src/mobile.rs`, ~80 lines of module docs: top bar (menus, document name and
  status, undo, redo, toggles, panels button), bottom tool strip with **44 pt touch targets** and
  long-press grouping, an options row, panels as a **sheet** (Layers / History / Color /
  Properties / Navigator), menus as sheets with a search field, virtual mouse for touch.
- `fits_phone(size)` triggers on the **shorter** side `< PHONE_MAX_SIDE` (600 pt), so it holds in
  portrait *and* landscape. Unit test `a_narrow_window_is_a_phone_in_either_orientation`
  asserts `411x891` and `891x411` are phones and `800x1280`, `1440x900` are not.
- It is switched on for Android: `apps/photocraft-android/src/shell.rs:58` sets
  `photo.phone_layout = true`; `mobile::active()` then needs only the window to be under 600 pt.
  A modern phone is 360–430 dp wide, so it engages automatically — including on tablets in a
  split-screen half-window.

**What is not proven:** that this path is comfortable on the owner's actual phone. The fork's APK
(`v1.0.0`, 71,639,919 B, arm64-v8a) has never been installed on it; the phone screenshots so far
are of the from-scratch app. Treat "phone-ready" as *written and unit-tested*, not *verified*.

Verification step (first thing to do, ~10 minutes): install `REAImageStudio-debug.apk` from the
`v1.0.0` release, open an image, and check portrait + landscape: tool strip reachable with a
thumb, panels sheet opens and closes, menu sheet search focuses when the keyboard appears, pinch
and two-finger pan behave, stylus pressure works. Anything broken there is a `mobile.rs` bug, and
`crates/ui-egui/src/mobile.rs` is the only file it lives in.

Known limitations inherited from upstream that matter on a phone, per the fork's `docs/STATUS.md`:
no screen-reader (TalkBack) support, no display colour profiles. Both are recorded as gaps, not
planned for v1.

## 3. Question 2 — can it look like ours, not like PhotoCraft?

Yes, and the code is arranged so this is a theming job rather than a rewrite.

**Why it is cheap:** every widget reads colours and radii from `theme::Tokens::get(ctx)`, and
`get()` reads a `Tokens` value out of egui's temp data (`theme.rs:766`). `Tokens` is a flat struct
of ~40 fields — `chrome`, `canvas`, `dock`, `card`, `field`, `hover`, `pressed`, `text`,
`text_dim`, `text_faint`, `icon`, `accent`, `accent_soft`, `accent_border`, `accent_text`,
`separator`, `shadow`, `radius_sm/radius/radius_lg`, `bevel`, `round_chips`, … So a new palette
plus our own radii change the whole app at once, with **no edits to the 248k lines of editor UI**.

**What we change (in this order):**

1. **Palette** — a new `ThemeKind` arm (or a token overlay applied at startup in
   `apps/photocraft-android/src/shell.rs`), set as the default for a fresh install. Distinct
   surface treatment (e.g. flat near-black with a single saturated accent and generous radii)
   rather than Photoshop's grey bevels. Registered in `ThemeKind::ALL`/`id()` so saved
   preferences still load; the upstream themes stay selectable for users who want them.
2. **Typography** — `theme.rs` loads the fonts. Today that is only Inter and JetBrains Mono
   (`assets/fonts/`, both OFL). Headings get our own OFL display face added to that folder, with
   its licence file, and an OFL-1.1 entry on the Licences screen.
3. **Icon set** — replace `assets/icons/*.svg` (169 icons) with our own strokes on one grid.
   The SVGs are loaded by name, so this is an asset swap plus a checklist that no name is
   missing. Same licence position: our own artwork, no Lucide files left in the tree.
4. **Phone chrome** — restyle `mobile.rs`'s own drawing (bar heights stay at their accessibility
   values: 48 top / 44 options / 56 tool strip / 44 touch targets — we change looks, not
   ergonomics), including the panels sheet handle, the tool strip selection pill and the menu
   sheet's search field.
5. **Start page / About / Licences** — our own start page instead of the editor-on-launch, an
   About with our name, version, changelog and a **Licences** section generated from the licence
   files at build time (`crates/ui-egui/src/credits.rs` is the About window to hang it on).
6. **Store assets** — our screenshots, feature graphic, icon at Play sizes, description. Upstream's
   own brand folder and demo screenshots (`docs/brand/`, `docs/images/` in their repository) are
   never copied in: the NOTICE excludes the ArtCraft name, wordmark and logos from the licence. We
   keep only their *code*, never their *brand*.

**What we deliberately do not do:** rename internals that risk a runtime-only failure
(`photocraftCall` JNI method, `libphotocraft_android.so`), strip upstream copyright headers, or
remove `credits.rs` / `contributors/`. Those cost nothing commercially and removing them would be
a licence breach.

## 4. Licence obligations — the Play Store checklist

MIT and Apache-2.0 both allow **paid, closed-source** distribution. The obligations:

| # | Obligation | Where it is satisfied | Status |
|---|---|---|---|
| 1 | Ship MIT + Apache-2.0 texts with copyright lines | `LICENSE-MIT`, `LICENSE-APACHE`, and in-app Licences screen | Files kept; screen **to build** |
| 2 | Reproduce upstream `NOTICE` (Apache-2.0 §4(d)) | `NOTICE` (with a fork section) + same text in the Licences screen | File kept; screen **to build** |
| 3 | State that files were changed (§4(b)) | `docs/FORK.md`, About screen, Play listing ("engine derived from PhotoCraft, modified") | Doc kept; listing **to write** |
| 4 | No upstream trademarks | Name, icon, listing, screenshots all ours; `docs/brand/` unused | Done for name/icon |
| 5 | Bundled third-party assets keep their licences | Inter / JetBrains Mono (OFL 1.1), Lucide (ISC), SCOWL, libfacedetection (BSD-3), X11 `rgb.txt` (MIT) — all listed in `ATTRIBUTION.md` | Kept; must appear in Licences screen |
| 6 | No new non-permissive dependency sneaks in | `cargo-deny` in CI on `deny.toml` with an allow-list of permissive licences | **To add** |

Play's own rules that shape the release: new personal developer accounts must run a **closed test
with 12 testers for 14 continuous days** before production; data-safety form must declare the
local-only filesystem access; and the app must not request permissions it does not use.

## 5. Work packages

Each is independently shippable and reviewable. `PLAN-B.md` is updated in the same commit as the
work, so this table is the state of the branch, not a wish list.

| # | Work package | State |
|---|---|---|
| B0 | Verify the phone path on a real phone | **Needs the owner** — install `v1.0.1` and walk §2 |
| B1 | Palette: REA Ink + REA Ink Dark | **Done** — host tests and the arm64 APK build pass |
| B2 | Our own brand mark | **Done** — the title-bar tile and the launcher icon are ours, drawn from tokens |
| B2b | Our own icon set (169 glyphs) | **Deferred** — Lucide (ISC) stays for now, see below |
| B3 | Phone chrome | **Partly done** — the panels sheet is a card with rounded top, hairline and grab handle |
| B4 | Licences screen | **Done** — About ▸ Licences, 19 texts baked in from the repository |
| B5 | `cargo-deny` gate | **Done and passing** — it caught four real issues on the way in |
| B6 | Store assets and listing | Not started (needs the palette on a real screen first) |

### What B1 actually changed

`ThemeKind::Ink` and `ThemeKind::InkDark` are first-class themes, not a paint-over: they have
preference variants (`Theme`, `DarkTheme`, `LightTheme` in `crates/engine/src/prefs.rs`), entries in
`ThemeKind::ALL`/`label`/`id`/`from_name`, the `prefs.set` appearance mapping, and the top of the
theme picker in both appearance families. Tests: `rea_ink_is_the_brand_palette_and_meets_text_contrast`
(contrast on every surface the tokens paint, one accent, the flat rounded layout) and
`rea_ink_is_chosen_by_preference_and_fixes_the_matching_mode` (picking it by saved id selects it and
pins the appearance mode).

Android now writes them once, on first launch, guarded by a marker file
(`crates/ui-egui`-side code in `apps/photocraft-android/src/shell.rs`). Before this, every launch
forced a theme on again and a user's choice could not survive a restart; that is fixed as part of
the branding work.

### What B2 changed

The title bar used to paint **upstream's kitsune app icon** (`crates/ui-egui/src/brand.rs`, from
`assets/app-icon/`). Its own `LICENSE.txt` puts it under MIT/Apache-2.0, so it was legal — but it is
their brand mark, and shipping it in a different paid product is exactly what §4 of the licence
checklist says not to do.

`brand.rs` now **draws** the mark: three stacked rounded squares on a tile in the theme accent, no
bitmap and no upstream artwork in the binary. It recolours with the theme and stays crisp at any
scale. The Android launcher icon uses the same construction and colours, sized to the 66 dp circular
mask a launcher may apply, so the home-screen icon and the in-app tile are one mark. A test asserts
that painting the title bar uploads **no texture**, which is what keeps artwork out of the build.

### What B2b waits for

Replacing 169 icons is asset work, not theming: each SVG is referenced by name from the UI, so a
partial swap is worse than none. Lucide is ISC — it may be shipped in a paid app, and it is listed
on the Licences screen. The honest position for v1 is that the *palette, shape language, sheet,
Licences screen and identity* are ours while the *glyphs* are still Lucide's; a full icon set is a
design-decision-per-icon job best done after the palette is settled on a real screen.

### What B5 found

The gate paid for itself immediately. In order: a JS-source-not-allowed (a fourth pinned git
repository, `storytold/winprint.rs`), an **Ubuntu Font Licence** font compiled into the binary by
`epaint` that nobody had noticed, an IJG-licensed JPEG encoder, and a CDLA-Permissive-2.0
certificate list. All four are permissive and all four now have their notices reproduced or their
source named in `deny.toml`; the run is green.

- **B0 — verify the phone path** (no code): install `v1.0.0`, walk the checklist in §2, file what
  breaks.
- **B1 — palette + our default theme** (`crates/ui-egui/src/theme.rs`,
  `apps/photocraft-android/src/shell.rs`): one new `ThemeKind`, our radii, default on Android.
  Verify by screenshot from CI emulator at `wm size 411x891`.
- **B2 — icons** (`assets/icons/`): our set, name-for-name, with a build-time check that every
  icon referenced by the UI exists.
- **B3 — phone chrome polish** (`crates/ui-egui/src/mobile.rs`).
- **B4 — Licences screen + About** (`crates/ui-egui/src/credits.rs` + generated include).
- **B5 — `cargo-deny` gate** (`deny.toml`, `.github/workflows/`).
- **B6 — store assets + listing text** (no code).

B5 is worth doing early: a transitive crate under a copyleft licence would be a distribution
problem, and it is one workflow file to find out.

## 6. Risks, stated plainly

- **Build latency.** No Rust toolchain exists in this workspace, so every compile is a CI
  round-trip (~20–40 min for this tree). Theme work is therefore batched per work package, not
  per tweak.
- **APK size.** 71.6 MB debug, arm64 only. Play wants an App Bundle and ideally a universal or
  per-ABI split; stripping debug symbols and enabling release LTO needs its own pass, and its size
  will only be known after a release build.
- **The phone layout is an unproven path on device.** It has unit tests, not a QA pass.
- **No TalkBack support, no colour profiles** — inherited gaps, must be disclosed in the listing
  if we claim accessibility.
- **Upstream moves.** We track a specific commit; merging upstream later means re-checking the
  palette and icon diffs.
- **Time.** Realistically 3–5 weeks of work plus Play's 14-day closed test, and the 12-tester
  requirement is a real gate for a solo developer.

## 7. What "done" looks like for the paid build

A signed AAB on Play internal testing that: launches into our start page, uses the phone layout
on a phone, carries our palette and icons throughout, lists every licence in an in-app screen,
passes `cargo-deny`, exports a PSD, and whose screenshots contain nothing from upstream's brand.
