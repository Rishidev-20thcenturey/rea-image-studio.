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

## Pricing and monetisation

- **Free download** with the editor usable without an account. A paid app is a harder sell and
  fewer installs to reach the closed-test threshold with.
- **One purchase, not a subscription**, for the pro tier: PSD/PSB export, batch export, and the
  larger brush preset packs. One-time purchases are what this audience asks for, and a subscription
  for a local editor invites one-star reviews.
- No ads, ever. The audience that pays for an image editor is exactly the audience ads make leave.
- Price anchors: the paid tier sits below a desktop editor's one-month subscription.

## The gate that actually decides the launch date

A new personal developer account must run a **closed test with at least 12 testers for 14 continuous
days** before it can go to production. Recruit the 12 now, not after the build is perfect: the clock
starts when the first person installs, and it cannot be shortened.

## Still missing before submission

- A **release-signed AAB** (every build so far is debug-signed) and an upload key that is backed up.
- A **release** build profile: debug symbols stripped, LTO on, and the APK/AAB size measured.
  The debug APK is ~68 MiB, which is too big to ship without work.
- **64-bit arm only** today. Play accepts that, but a phone with a 32-bit-only install path will
  not run it; decide whether to add `armeabi-v7a` or state the requirement.
- **Privacy policy URL** — mandatory even for an app that collects nothing; it must say the app
  keeps files on the device.
- The accessibility and colour-profile gaps in the description above must match the build.
