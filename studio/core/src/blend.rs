//! Blend modes and source-over compositing.
//!
//! Colours are straight (not premultiplied) RGBA in `0..=255` at the edges of the module and
//! `0.0..=1.0` inside it. The compositing formula is the W3C one, so a layer blends against the
//! backdrop the way it does in Photoshop rather than the naive "mix the two colours" that loses
//! alpha.

/// The blend modes this editor implements. The numeric values are the on-disk and JNI encoding:
/// never renumber them, only append.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendMode {
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    HardLight = 8,
    SoftLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
}

impl BlendMode {
    pub fn from_u8(v: u8) -> BlendMode {
        match v {
            1 => BlendMode::Multiply,
            2 => BlendMode::Screen,
            3 => BlendMode::Overlay,
            4 => BlendMode::Darken,
            5 => BlendMode::Lighten,
            6 => BlendMode::ColorDodge,
            7 => BlendMode::ColorBurn,
            8 => BlendMode::HardLight,
            9 => BlendMode::SoftLight,
            10 => BlendMode::Difference,
            11 => BlendMode::Exclusion,
            12 => BlendMode::Hue,
            13 => BlendMode::Saturation,
            14 => BlendMode::Color,
            15 => BlendMode::Luminosity,
            _ => BlendMode::Normal,
        }
    }

    /// The label the UI shows. Kept here so the Kotlin side never has to duplicate the list.
    pub fn label(self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::HardLight => "Hard Light",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }

    /// Index -> mode, for building a mode picker in the UI.
    pub fn all() -> [BlendMode; 16] {
        [
            BlendMode::Normal,
            BlendMode::Multiply,
            BlendMode::Screen,
            BlendMode::Overlay,
            BlendMode::Darken,
            BlendMode::Lighten,
            BlendMode::ColorDodge,
            BlendMode::ColorBurn,
            BlendMode::HardLight,
            BlendMode::SoftLight,
            BlendMode::Difference,
            BlendMode::Exclusion,
            BlendMode::Hue,
            BlendMode::Saturation,
            BlendMode::Color,
            BlendMode::Luminosity,
        ]
    }
}

#[inline]
fn clamp01(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

#[inline]
fn lum(c: [f32; 3]) -> f32 {
    0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]
}

/// Saturation of an RGB colour, as Photoshop defines it: max minus min.
#[inline]
fn sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

/// Replace the luminosity of `c` with `l`, keeping its hue and saturation. This is the classic
/// `ClipColor`/`SetLum` pair from the compositing spec.
fn set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - lum(c);
    clip_color([c[0] + d, c[1] + d, c[2] + d])
}

fn clip_color(c: [f32; 3]) -> [f32; 3] {
    let l = lum(c);
    let n = c[0].min(c[1]).min(c[2]);
    let x = c[0].max(c[1]).max(c[2]);
    let mut out = c;
    if n < 0.0 {
        if (l - n).abs() > f32::EPSILON {
            for v in out.iter_mut() {
                *v = l + (*v - l) * l / (l - n);
            }
        } else {
            out = [l, l, l];
        }
    }
    if x > 1.0 {
        if (x - l).abs() > f32::EPSILON {
            for v in out.iter_mut() {
                *v = l + (*v - l) * (1.0 - l) / (x - l);
            }
        } else {
            out = [l, l, l];
        }
    }
    [clamp01(out[0]), clamp01(out[1]), clamp01(out[2])]
}

fn set_sat(c: [f32; 3], s: f32) -> [f32; 3] {
    let mut idx = [0usize, 1, 2];
    // Sort the channel indices by value: index 0 is the mid channel, 1 the max, 2 the min.
    if c[idx[0]] > c[idx[1]] {
        idx.swap(0, 1);
    }
    if c[idx[1]] > c[idx[2]] {
        idx.swap(1, 2);
    }
    if c[idx[0]] > c[idx[1]] {
        idx.swap(0, 1);
    }
    let (min_i, mid_i, max_i) = (idx[0], idx[1], idx[2]);
    let mut out = c;
    if c[max_i] > c[min_i] {
        out[mid_i] = (c[mid_i] - c[min_i]) * s / (c[max_i] - c[min_i]);
        out[max_i] = s;
    } else {
        out[mid_i] = 0.0;
        out[max_i] = 0.0;
    }
    out[min_i] = 0.0;
    out
}

/// The blend function `B(Cb, Cs)` for one pixel, straight colour, no alpha.
pub fn blend_rgb(mode: BlendMode, cb: [f32; 3], cs: [f32; 3]) -> [f32; 3] {
    let mut out = [0.0f32; 3];
    match mode {
        BlendMode::Normal => return cs,
        BlendMode::Hue => {
            let r = set_lum(set_sat(cs, sat(cb)), lum(cb));
            return r;
        }
        BlendMode::Saturation => {
            let r = set_lum(set_sat(cb, sat(cs)), lum(cb));
            return r;
        }
        BlendMode::Color => {
            let r = set_lum(cs, lum(cb));
            return r;
        }
        BlendMode::Luminosity => {
            let r = set_lum(cb, lum(cs));
            return r;
        }
        _ => {}
    }
    for i in 0..3 {
        let b = cb[i];
        let s = cs[i];
        out[i] = match mode {
            BlendMode::Multiply => b * s,
            BlendMode::Screen => b + s - b * s,
            BlendMode::Overlay => {
                if b <= 0.5 {
                    2.0 * b * s
                } else {
                    1.0 - 2.0 * (1.0 - b) * (1.0 - s)
                }
            }
            BlendMode::Darken => b.min(s),
            BlendMode::Lighten => b.max(s),
            BlendMode::ColorDodge => {
                if b <= 0.0 {
                    0.0
                } else if s >= 1.0 {
                    1.0
                } else {
                    clamp01(b / (1.0 - s))
                }
            }
            BlendMode::ColorBurn => {
                if b >= 1.0 {
                    1.0
                } else if s <= 0.0 {
                    0.0
                } else {
                    clamp01(1.0 - (1.0 - b) / s)
                }
            }
            BlendMode::HardLight => {
                if s <= 0.5 {
                    2.0 * b * s
                } else {
                    1.0 - 2.0 * (1.0 - b) * (1.0 - s)
                }
            }
            BlendMode::SoftLight => {
                // W3C soft-light, which matches Photoshop closely enough for a layer stack.
                if s <= 0.5 {
                    b - (1.0 - 2.0 * s) * b * (1.0 - b)
                } else {
                    let d = if b <= 0.25 {
                        ((16.0 * b - 12.0) * b + 4.0) * b
                    } else {
                        b.sqrt()
                    };
                    b + (2.0 * s - 1.0) * (d - b)
                }
            }
            BlendMode::Difference => (b - s).abs(),
            BlendMode::Exclusion => b + s - 2.0 * b * s,
            _ => s,
        };
    }
    [clamp01(out[0]), clamp01(out[1]), clamp01(out[2])]
}

/// Composite `src` over `dst`, both straight RGBA in `0..=1`, with the given blend mode.
///
/// This is the W3C formula, which is what makes a partially transparent layer behave: with the
/// naive lerp, a semi-transparent multiply layer either loses its alpha or darkens the backdrop
/// twice.
pub fn composite_over(dst: [f32; 4], src: [f32; 4], mode: BlendMode) -> [f32; 4] {
    let (as_, ab) = (src[3], dst[3]);
    if as_ <= 0.0 {
        return dst;
    }
    let ao = as_ + ab * (1.0 - as_);
    if ao <= 0.0 {
        return [0.0, 0.0, 0.0, 0.0];
    }
    let cb = [dst[0], dst[1], dst[2]];
    let cs = [src[0], src[1], src[2]];
    let blended = blend_rgb(mode, cb, cs);
    let mut out = [0.0f32; 4];
    for i in 0..3 {
        let num = (1.0 - ab) * as_ * cs[i] + ab * as_ * blended[i] + (1.0 - as_) * ab * cb[i];
        out[i] = clamp01(num / ao);
    }
    out[3] = clamp01(ao);
    out
}

/// Composite a straight RGBA byte pixel over another, with a source alpha multiplier (layer
/// opacity and mask coverage folded together).
#[inline]
pub fn composite_px(dst: [u8; 4], src: [u8; 4], mode: BlendMode, alpha_scale: f32) -> [u8; 4] {
    if src[3] == 0 || alpha_scale <= 0.0 {
        return dst;
    }
    let d = [
        dst[0] as f32 / 255.0,
        dst[1] as f32 / 255.0,
        dst[2] as f32 / 255.0,
        dst[3] as f32 / 255.0,
    ];
    let s = [
        src[0] as f32 / 255.0,
        src[1] as f32 / 255.0,
        src[2] as f32 / 255.0,
        (src[3] as f32 / 255.0) * clamp01(alpha_scale),
    ];
    let o = composite_over(d, s, mode);
    [
        (o[0] * 255.0 + 0.5) as u8,
        (o[1] * 255.0 + 0.5) as u8,
        (o[2] * 255.0 + 0.5) as u8,
        (o[3] * 255.0 + 0.5) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.002
    }

    #[test]
    fn normal_over_opaque_backdrop_replaces_the_colour() {
        let out = composite_over([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0], BlendMode::Normal);
        assert!(approx(out[0], 0.0) && approx(out[2], 1.0) && approx(out[3], 1.0));
    }

    #[test]
    fn half_transparent_source_mixes_halfway() {
        let out = composite_over([0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 0.5], BlendMode::Normal);
        assert!(approx(out[0], 0.5), "got {:?}", out);
        assert!(approx(out[3], 1.0));
    }

    #[test]
    fn multiply_and_screen_are_dual() {
        let cb = [0.5, 0.5, 0.5];
        let cs = [0.5, 0.5, 0.5];
        let mul = blend_rgb(BlendMode::Multiply, cb, cs);
        let scr = blend_rgb(BlendMode::Screen, cb, cs);
        assert!(approx(mul[0], 0.25), "multiply {:?}", mul);
        assert!(approx(scr[0], 0.75), "screen {:?}", scr);
    }

    #[test]
    fn overlay_and_hard_light_agree_on_mid_grey_backdrop() {
        let cb = [0.5, 0.5, 0.5];
        let cs = [0.75, 0.75, 0.75];
        let ov = blend_rgb(BlendMode::Overlay, cb, cs);
        // Overlay with a 0.5 backdrop is 1 - 2*(1-b)*(1-s) = 1 - 2*0.5*0.25 = 0.75.
        assert!(approx(ov[0], 0.75), "overlay {:?}", ov);
        let hl = blend_rgb(BlendMode::HardLight, cs, cb);
        assert!(approx(hl[0], 0.75), "hard light {:?}", hl);
    }

    #[test]
    fn difference_of_identical_colours_is_black() {
        let c = [0.3, 0.6, 0.9];
        let d = blend_rgb(BlendMode::Difference, c, c);
        assert!(approx(d[0], 0.0) && approx(d[1], 0.0) && approx(d[2], 0.0));
    }

    #[test]
    fn luminosity_keeps_the_backdrop_hue() {
        // A blue backdrop with a bright grey source keeps blue but takes the brightness.
        let out = blend_rgb(BlendMode::Luminosity, [0.0, 0.0, 1.0], [0.5, 0.5, 0.5]);
        assert!(out[2] >= out[0], "blue should stay the dominant channel: {:?}", out);
        assert!(approx(lum(out), 0.5), "luminosity should be the source's: {:?}", out);
    }

    #[test]
    fn hue_and_saturation_are_not_luminosity() {
        let out = blend_rgb(BlendMode::Hue, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        // The result carries the source's hue at the backdrop's luminosity and saturation.
        assert!(approx(lum(out), lum([1.0, 0.0, 0.0])), "luminosity changed: {:?}", out);
        assert!(sat(out) > 0.0, "saturation was lost: {:?}", out);
    }

    #[test]
    fn transparent_source_leaves_the_backdrop_alone() {
        let out = composite_over([0.2, 0.4, 0.6, 1.0], [1.0, 0.0, 0.0, 0.0], BlendMode::Multiply);
        assert!(approx(out[0], 0.2) && approx(out[1], 0.4) && approx(out[2], 0.6));
    }

    #[test]
    fn byte_compositing_round_trips_through_the_float_path() {
        let out = composite_px([0, 0, 0, 255], [255, 255, 255, 255], BlendMode::Normal, 0.5);
        // 0.5 of white over black is 127.5, and the byte path rounds half up: 128.
        assert_eq!(out, [128, 128, 128, 255]);
    }

    #[test]
    fn every_mode_is_reachable_from_its_number() {
        for (i, m) in BlendMode::all().iter().enumerate() {
            assert_eq!(BlendMode::from_u8(i as u8), *m);
            assert!(!m.label().is_empty());
        }
    }

    #[test]
    fn blend_modes_never_leave_the_unit_range() {
        for m in BlendMode::all() {
            for &b in &[0.0f32, 0.25, 0.5, 0.75, 1.0] {
                for &s in &[0.0f32, 0.25, 0.5, 0.75, 1.0] {
                    let out = blend_rgb(m, [b, b, b], [s, s, s]);
                    for c in out {
                        assert!(c.is_finite(), "{:?} produced {}", m, c);
                        assert!((0.0..=1.0).contains(&c), "{:?} produced {}", m, c);
                    }
                }
            }
        }
    }
}
