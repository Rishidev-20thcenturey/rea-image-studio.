# Play Store listing — draft copy and the answers the form will ask for

Everything here is copy that can be pasted, plus the facts a store form demands. Where a claim is
not yet true it says so, because a store listing that promises something the app does not do is the
fastest way to lose the account.

Nothing in this file is published yet.

## Identity

- **App name** (30 max): `REA Image Studio`
- **Package**: `com.rea.imagestudio`
- **Category**: Art & Design (primary), Photography (secondary is not offered; pick one)
- **Content rating**: Everyone (no user-generated content sharing, no ads, no purchases of
  physical goods)
- **Ads**: none — declare "No ads"
- **In-app purchases**: yes if the export tier ships (see Pricing)

## Short description (80 max, 74 here)

```
Layers, masks, brushes and PSD files — a real image editor for your phone.
```

## Full description (4000 max)

```
REA Image Studio is a full image editor built for a phone, not squashed onto one.

LAYERS THAT BEHAVE
Stack layers and groups, reorder them with a thumb, and give any of them a mask. Layer styles,
clipping, opacity and blend modes work the way you expect them to. The Layers panel opens as a card
over the picture, so you can see what you are editing while you edit it.

BRUSHES THAT FOLLOW YOUR HAND
Pen pressure works with a stylus or a digital eraser: press harder, the stroke gets wider and darker.
Paint with a second finger resting on the screen without the stroke jumping. The brush engine is the
one from the desktop-class engine underneath, with the presets that come with it.

SELECTIONS AND MASKS
Lasso, magnetic lasso, rectangular and elliptical marquees, quick selection, and colour range.
Feather, expand, contract, invert, or turn a selection into a mask.

ADJUSTMENTS AND FILTERS
Curves, levels, hue and saturation, colour balance, black and white, gradient maps, blur, sharpen,
noise and more — non-destructive where the engine allows it.

REAL PSD SUPPORT
Open PSD and PSB documents with their layers, groups, masks and blend modes intact, and save them
back as PSD. Import and export PNG, JPEG, TIFF, WebP and more.

MADE TO BE TOUCHED
44 pt tool targets, a tool strip within a thumb's reach, panels that rise as a card, and a layout
that adapts when you turn the phone. Undo and redo are always one tap away.

WHAT IT IS BUILT ON
The engine underneath is PhotoCraft and PhotoDroid, open-source software under the MIT and
Apache-2.0 licences. This app is a fork: their engine, our interface, our theme and our fixes. The
full licence texts and the credits of everyone who wrote it are in About ▸ Licences, in the app.

WHAT IT DOES NOT DO YET
We would rather say this here than have you find out after installing. This version has no
screen-reader support, does not manage display colour profiles, and cannot install plugins. Text,
shapes and vector paths are present but less polished than the paint and layer tools. If those
matter more to you than the rest, wait for a later version.
```

Word of warning for the copywriter: the last paragraph must match whatever the release actually
does at submission time. Re-read it against `docs/STATUS.md` before every submission.

## Data safety form — the truthful answers

| Question | Answer |
|---|---|
| Does the app collect or share user data? | **No** |
| Data types collected | None |
| Is data encrypted in transit? | Not applicable — nothing leaves the device |
| Can users request data deletion? | Not applicable — no account, no server |
| Permissions | Photos/media read (opening documents), storage write (saving to Downloads). No internet permission is requested for editing. |
| Ads / analytics SDKs | None |
| Crash reporting | Local only: a log the user can view and copy from the app's Diagnostics menu, never uploaded |

If a crash reporter or an analytics SDK is ever added, this table changes *before* the release, not
after.

## Screenshot plan (phone, 1080 × 2400 or larger)

Eight shots, in this order, each with a one-line caption baked into the image:

1. The editor with a photograph open — palette and tool strip visible.
2. The Layers card open over the picture, showing a group, a mask and a layer style.
3. A stylus stroke mid-paint, with the brush options row.
4. Curves or Levels open, showing the graph.
5. A selection being made with the magnetic lasso.
6. The menus sheet with the search field focused.
7. PSD import: a layered document open with its group structure.
8. About ▸ Licences, showing the components table. (Differentiator: an app that publishes its
   obligations is one you can trust with your files.)

Rules for every shot: our own palette only, our own icon, no upstream screenshot or artwork, no
device frame that imitates a competitor, and no text claiming a feature that is not in that build.

Also needed: a 1024 × 500 feature graphic, a 512 × 512 icon, and a 30 s video (optional; a screen
recording of a stroke plus an export is enough).

## Pricing — paid download, decided

The owner's decision: **paid download from day one, no in-app purchase.** That is also the simplest
and cleanest option for this app:

- Play handles the payment; the app needs no billing library, no entitlement check and no server.
- The manifest requests **no permissions at all** — not even `INTERNET`. Nothing to buy inside the
  app is consistent with that, and the data-safety form is a truthful "collects nothing".
- No ads, ever, and no subscription. For a local editor a subscription invites one-star reviews,
  and ads make exactly the audience that pays for an image editor leave.
- The trade-off to accept knowingly: a paid download gets far fewer installs than a free one, and
  refunds inside Google's two-hour window are automatic. The first screen has to earn the sale,
  which is why the store listing's screenshots carry more weight than usual.
- No trial is possible without IAP or a second app. If a trial is ever wanted, the route is a free
  "lite" listing alongside, not billing inside this one.

Price guidance: pick a price the local market will pay without thinking — the cost of one coffee
rather than the cost of a desktop editor — and raise it after the first hundred sales if refunds are
low. Changing the price later is free and instant.

## The gate that actually decides the launch date

A new personal developer account must run a **closed test with at least 12 testers for 14 continuous
days** before it can go to production. Recruit the 12 now, not after the build is perfect: the clock
starts when the first person installs, and it cannot be shortened.

## Still missing before submission

- **The upload key.** The CI now builds a release-signed bundle when four repository secrets exist
  (`ANDROID_KEYSTORE_BASE64`, `ANDROID_STORE_PASSWORD`, `ANDROID_KEY_ALIAS`,
  `ANDROID_KEY_PASSWORD`); without them the bundle is built unsigned and the job says so. Generate
  the key once, back it up somewhere you will still have in five years, and never regenerate it —
  losing it means losing the ability to update the app.
- A **release build profile**: debug symbols are already stripped, but LTO is off
  (`[profile.android]` in `Cargo.toml`) and the debug APK is ~68 MiB. Measure the AAB before
  deciding whether that is worth a slower build.
- **64-bit arm only** today. Play accepts that, and Android has required 64-bit support since 2019,
  so this is fine. A 32-bit-only device simply will not be offered the app.
- **Privacy policy URL** — written (`docs/store/PRIVACY.md`), needs a monitored contact address and
  a public URL before it can be pasted into the console.
- The accessibility and colour-profile gaps in the description above must match the build.

## Done on this branch, for the upload

- `targetSdk` / `compileSdk` **36** — required for new apps since 2026-08-31; the CI SDK install was
  moved from 35 to 36 with it.
- A **release App Bundle** is built on every run (`bundleRelease`) and attached to tagged releases
  next to the APK: Play has accepted only AABs for new apps since 2021.
- The APK's **versionCode/versionName** now come from the git tag, so a second upload is possible.
- **512 × 512 store icon** drawn from the same mark as the launcher icon:
  `docs/store/icon-512.png`.
