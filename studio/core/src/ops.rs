//! Editing operations: brush strokes, shapes, bucket fill, selections, adjustments, filters,
//! transforms and canvas changes. Everything here pushes one undo step and reports the region it
//! changed, so the composite only ever recomputes what it must.

use crate::doc::{Document, Layer, Rect};
use std::sync::Arc;

/// An adjustment the UI can apply to the active layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adjust {
    BrightnessContrast = 0,
    Levels = 1,
    Curves = 2,
    HueSaturation = 3,
    Threshold = 4,
    Invert = 5,
    Desaturate = 6,
    AutoLevels = 7,
    Temperature = 8,
}

/// A filter the UI can apply to the active layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter {
    Blur = 0,
    Sharpen = 1,
    Noise = 2,
    Pixelate = 3,
}

fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// Coverage of the selection at a document pixel, from a snapshot taken before the edit started.
#[inline]
fn cov(sel: &Option<Arc<Vec<u8>>>, w: i32, h: i32, x: i32, y: i32) -> f32 {
    match sel {
        None => 1.0,
        Some(m) => {
            if x < 0 || y < 0 || x >= w || y >= h {
                0.0
            } else {
                m[(y as usize) * (w as usize) + x as usize] as f32 / 255.0
            }
        }
    }
}

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    let t = clamp01(t);
    ((a as f32) * (1.0 - t) + (b as f32) * t).round().clamp(0.0, 255.0) as u8
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let (rf, gf, bf) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let l = (max + min) / 2.0;
    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == rf {
        ((gf - bf) / d + if gf < bf { 6.0 } else { 0.0 }) / 6.0
    } else if max == gf {
        ((bf - rf) / d + 2.0) / 6.0
    } else {
        ((rf - gf) / d + 4.0) / 6.0
    };
    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    if s <= 0.0 {
        let v = (clamp01(l) * 255.0).round() as u8;
        return (v, v, v);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let f = |t: f32| -> f32 {
        let mut t = t;
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    (
        (clamp01(f(h + 1.0 / 3.0)) * 255.0).round() as u8,
        (clamp01(f(h)) * 255.0).round() as u8,
        (clamp01(f(h - 1.0 / 3.0)) * 255.0).round() as u8,
    )
}

impl Document {
    /// Apply `f(x, y, pixel, coverage)` to every pixel of the active layer inside the selection,
    /// as one undo step. `f` receives the selection coverage so an adjustment fades at a feathered
    /// edge instead of stopping dead.
    pub fn edit_active_pixels<F>(&mut self, name: &str, mut f: F)
    where
        F: FnMut(i32, i32, [u8; 4], f32) -> [u8; 4],
    {
        if self.layers.is_empty() {
            return;
        }
        let idx = self.active.min(self.layers.len() - 1);
        let sel_bounds = self.selection_bounds();
        if sel_bounds.is_empty() {
            return;
        }
        self.push_undo(name);
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let bounds = {
            let layer = &mut self.layers[idx];
            let lx0 = (sel_bounds.x0 - layer.offset.0).max(0);
            let ly0 = (sel_bounds.y0 - layer.offset.1).max(0);
            let lx1 = (sel_bounds.x1 - layer.offset.0).min(layer.w);
            let ly1 = (sel_bounds.y1 - layer.offset.1).min(layer.h);
            let (lw, ox, oy) = (layer.w, layer.offset.0, layer.offset.1);
            let buf = layer.pixels_mut();
            for ly in ly0..ly1 {
                for lx in lx0..lx1 {
                    let o = ((ly as usize) * (lw as usize) + lx as usize) * 4;
                    let px = [buf[o], buf[o + 1], buf[o + 2], buf[o + 3]];
                    if px[3] == 0 {
                        continue;
                    }
                    let c = cov(&sel, dw, dh, lx + ox, ly + oy);
                    if c <= 0.0 {
                        continue;
                    }
                    let out = f(lx + ox, ly + oy, px, c);
                    buf[o] = out[0];
                    buf[o + 1] = out[1];
                    buf[o + 2] = out[2];
                    buf[o + 3] = out[3];
                }
            }
            Rect::new(lx0 + ox, ly0 + oy, lx1 + ox, ly1 + oy)
        };
        self.mark_dirty(bounds);
    }

    // --- brush ---------------------------------------------------------------------------

    /// Begin a stroke. `tool`: 0 = brush, 1 = eraser, 2 = rectangle fill, 3 = ellipse fill.
    /// For the shape tools the stroke is a drag: `begin`, then `stroke_to`, then `stroke_end`.
    pub fn stroke_begin(
        &mut self,
        x: f32,
        y: f32,
        pressure: f32,
        tool: u8,
        color: [u8; 4],
        size: f32,
        hardness: f32,
        flow: f32,
        opacity: f32,
    ) {
        if self.layers.is_empty() {
            return;
        }
        let label = match tool {
            1 => "Erase",
            2 => "Rectangle",
            3 => "Ellipse",
            _ => "Brush",
        };
        self.push_undo(label);
        self.stroke = crate::doc::StrokeState {
            active: true,
            last: (x, y),
            last_pressure: pressure.clamp(0.02, 1.0),
            color,
            size: size.max(1.0),
            hardness: clamp01(hardness),
            flow: clamp01(flow),
            opacity: clamp01(opacity),
            erase: tool == 1,
            residual: 0.0,
            bounds: Rect::empty(),
        };
        self.stroke_tool = tool;
        if tool <= 1 {
            self.stamp_at(x, y, self.stroke.last_pressure);
        }
    }

    pub fn stroke_to(&mut self, x: f32, y: f32, pressure: f32) {
        if !self.stroke.active {
            return;
        }
        let p = pressure.clamp(0.02, 1.0);
        let (x0, y0) = self.stroke.last;
        match self.stroke_tool {
            2 | 3 => {
                // Shapes repaint from scratch each move: preview by restoring the pre-drag pixels.
                self.reshape_preview(x0, y0, x, y);
            }
            _ => {
                let spacing = (self.stroke.size * 0.12).max(0.7);
                let dx = x - x0;
                let dy = y - y0;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 0.0001 {
                    return;
                }
                let mut travelled = -self.stroke.residual;
                while travelled + spacing <= dist {
                    travelled += spacing;
                    let t = travelled / dist;
                    let sx = x0 + dx * t;
                    let sy = y0 + dy * t;
                    let pr = self.stroke.last_pressure + (p - self.stroke.last_pressure) * t;
                    self.stamp_at(sx, sy, pr);
                }
                self.stroke.residual = dist - travelled;
            }
        }
        self.stroke.last = (x, y);
        self.stroke.last_pressure = p;
    }

    pub fn stroke_end(&mut self) {
        self.stroke.active = false;
        self.stroke.residual = 0.0;
        let b = self.stroke.bounds;
        if !b.is_empty() {
            self.mark_dirty(b.expand_clamped(2, self.w, self.h));
        }
        self.stroke.bounds = Rect::empty();
    }

    /// Draw one round brush stamp at a document position.
    fn stamp_at(&mut self, cx: f32, cy: f32, pressure: f32) {
        if self.layers.is_empty() {
            return;
        }
        let idx = self.active.min(self.layers.len() - 1);
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let s = self.stroke.clone();
        let radius = (s.size * 0.5 * (0.35 + 0.65 * pressure)).max(0.6);
        let hard = s.hardness;
        let (color, flow, erase) = (s.color, s.flow, s.erase);
        let mut touched = Rect::empty();
        {
            let layer = &mut self.layers[idx];
            let lx0 = ((cx - radius - layer.offset.0 as f32).floor() as i32).max(0);
            let ly0 = ((cy - radius - layer.offset.1 as f32).floor() as i32).max(0);
            let lx1 = ((cx + radius - layer.offset.0 as f32).ceil() as i32 + 1).min(layer.w);
            let ly1 = ((cy + radius - layer.offset.1 as f32).ceil() as i32 + 1).min(layer.h);
            if lx1 <= lx0 || ly1 <= ly0 {
                return;
            }
            let (lw, ox, oy) = (layer.w, layer.offset.0, layer.offset.1);
            let buf = layer.pixels_mut();
            for ly in ly0..ly1 {
                let dy = ly as f32 + 0.5 + oy as f32 - cy;
                for lx in lx0..lx1 {
                    let dx = lx as f32 + 0.5 + ox as f32 - cx;
                    let d = (dx * dx + dy * dy).sqrt();
                    if d > radius {
                        continue;
                    }
                    // hardness 1 = hard edge, 0 = linear falloff to nothing at the rim.
                    let falloff = if hard >= 0.999 {
                        1.0
                    } else {
                        clamp01((radius - d) / (radius * (1.0 - hard) + 0.0001))
                    };
                    let mut a = flow * falloff;
                    let c = cov(&sel, dw, dh, lx + ox, ly + oy);
                    a *= c;
                    if a <= 0.0 {
                        continue;
                    }
                    let o = ((ly as usize) * (lw as usize) + lx as usize) * 4;
                    if erase {
                        let na = (buf[o + 3] as f32) * (1.0 - a);
                        buf[o + 3] = na.round().clamp(0.0, 255.0) as u8;
                    } else {
                        // Source-over of the brush colour onto the layer pixel.
                        let sa = a * (color[3] as f32 / 255.0);
                        let da = buf[o + 3] as f32 / 255.0;
                        let oa = sa + da * (1.0 - sa);
                        if oa > 0.0 {
                            for k in 0..3 {
                                let sc = color[k] as f32;
                                let dc = buf[o + k] as f32;
                                buf[o + k] = ((sc * sa + dc * da * (1.0 - sa)) / oa)
                                    .round()
                                    .clamp(0.0, 255.0) as u8;
                            }
                            buf[o + 3] = (oa * 255.0).round().clamp(0.0, 255.0) as u8;
                        }
                    }
                    touched = touched.union(Rect::new(lx + ox, ly + oy, lx + ox + 1, ly + oy + 1));
                }
            }
        }
        self.stroke.bounds = self.stroke.bounds.union(touched);
        if !touched.is_empty() {
            self.mark_dirty(touched.expand_clamped(2, self.w, self.h));
        }
    }

    /// Redraw a shape drag from its anchor each time the finger moves, on top of the pixels the
    /// drag started with.
    fn reshape_preview(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        let idx = self.active.min(self.layers.len() - 1);
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let s = self.stroke.clone();
        let is_ellipse = self.stroke_tool == 3;
        let (bx0, by0, bx1, by1) = (
            x0.min(x1).floor(),
            y0.min(y1).floor(),
            x0.max(x1).ceil(),
            y0.max(y1).ceil(),
        );
        let mut touched = Rect::empty();
        {
            let layer = &mut self.layers[idx];
            let (lw, ox, oy) = (layer.w, layer.offset.0, layer.offset.1);
            let cx = (x0 + x1) * 0.5;
            let cy = (y0 + y1) * 0.5;
            let rx = ((x1 - x0).abs() * 0.5).max(0.5);
            let ry = ((y1 - y0).abs() * 0.5).max(0.5);
            let lx0 = ((bx0 as i32) - ox).max(0);
            let ly0 = ((by0 as i32) - oy).max(0);
            let lx1 = ((bx1 as i32) - ox + 1).min(lw);
            let ly1 = ((by1 as i32) - oy + 1).min(layer.h);
            let buf = layer.pixels_mut();
            for ly in ly0..ly1 {
                for lx in lx0..lx1 {
                    let dx = lx as f32 + 0.5 + ox as f32;
                    let dy = ly as f32 + 0.5 + oy as f32;
                    let inside = if is_ellipse {
                        let nx = (dx - cx) / rx;
                        let ny = (dy - cy) / ry;
                        nx * nx + ny * ny <= 1.0
                    } else {
                        dx >= bx0 && dx <= bx1 && dy >= by0 && dy <= by1
                    };
                    if !inside {
                        continue;
                    }
                    let c = cov(&sel, dw, dh, lx + ox, ly + oy);
                    if c <= 0.0 {
                        continue;
                    }
                    let o = ((ly as usize) * (lw as usize) + lx as usize) * 4;
                    let sa = c * ((s.color[3] as f32 / 255.0) * 1.0);
                    let da = buf[o + 3] as f32 / 255.0;
                    let oa = sa + da * (1.0 - sa);
                    if oa > 0.0 {
                        for k in 0..3 {
                            let sc = s.color[k] as f32;
                            let dc = buf[o + k] as f32;
                            buf[o + k] = ((sc * sa + dc * da * (1.0 - sa)) / oa)
                                .round()
                                .clamp(0.0, 255.0) as u8;
                        }
                        buf[o + 3] = (oa * 255.0).round().clamp(0.0, 255.0) as u8;
                    }
                    touched = touched.union(Rect::new(lx + ox, ly + oy, lx + ox + 1, ly + oy + 1));
                }
            }
        }
        self.stroke.bounds = self.stroke.bounds.union(touched);
        if !touched.is_empty() {
            self.mark_dirty(touched.expand_clamped(1, self.w, self.h));
        }
    }

    // --- fill ----------------------------------------------------------------------------

    /// Bucket fill on the active layer from a seed point, flood-limited by `tolerance` and clipped
    /// to the selection.
    pub fn bucket_fill(&mut self, x: i32, y: i32, color: [u8; 4], tolerance: u8) {
        if self.layers.is_empty() {
            return;
        }
        let idx = self.active.min(self.layers.len() - 1);
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let mut touched = Rect::empty();
        {
            let layer = &mut self.layers[idx];
            let lx = x - layer.offset.0;
            let ly = y - layer.offset.1;
            if lx < 0 || ly < 0 || lx >= layer.w || ly >= layer.h {
                return;
            }
            let seed = layer.pixel(lx, ly);
            let (lw, lh) = (layer.w, layer.h);
            let (ox, oy) = (layer.offset.0, layer.offset.1);
            let buf = layer.pixels_mut();
            let tol = tolerance as i32;
            let mut seen = vec![false; (lw as usize) * (lh as usize)];
            let mut stack: Vec<(i32, i32)> = vec![(lx, ly)];
            while let Some((px, py)) = stack.pop() {
                if px < 0 || py < 0 || px >= lw || py >= lh {
                    continue;
                }
                let si = (py as usize) * (lw as usize) + px as usize;
                if seen[si] {
                    continue;
                }
                seen[si] = true;
                let o = si * 4;
                let cur = [buf[o], buf[o + 1], buf[o + 2], buf[o + 3]];
                let d = (cur[0] as i32 - seed[0] as i32).abs()
                    .max((cur[1] as i32 - seed[1] as i32).abs())
                    .max((cur[2] as i32 - seed[2] as i32).abs())
                    .max((cur[3] as i32 - seed[3] as i32).abs());
                if d > tol {
                    continue;
                }
                let c = cov(&sel, dw, dh, px + ox, py + oy);
                if c <= 0.0 {
                    continue;
                }
                let sa = c * (color[3] as f32 / 255.0);
                let da = buf[o + 3] as f32 / 255.0;
                let oa = sa + da * (1.0 - sa);
                if oa > 0.0 {
                    for k in 0..3 {
                        let sc = color[k] as f32;
                        let dc = buf[o + k] as f32;
                        buf[o + k] = ((sc * sa + dc * da * (1.0 - sa)) / oa).round().clamp(0.0, 255.0) as u8;
                    }
                    buf[o + 3] = (oa * 255.0).round().clamp(0.0, 255.0) as u8;
                }
                touched = touched.union(Rect::new(px + ox, py + oy, px + ox + 1, py + oy + 1));
                stack.push((px + 1, py));
                stack.push((px - 1, py));
                stack.push((px, py + 1));
                stack.push((px, py - 1));
            }
        }
        if !touched.is_empty() {
            self.mark_dirty(touched.expand_clamped(1, self.w, self.h));
        }
    }

    /// Fill the whole selection on the active layer with a colour.
    pub fn fill_selection(&mut self, color: [u8; 4]) {
        self.edit_active_pixels("Fill", move |_x, _y, px, c| {
            let sa = c * (color[3] as f32 / 255.0);
            let da = px[3] as f32 / 255.0;
            let oa = sa + da * (1.0 - sa);
            if oa <= 0.0 {
                return px;
            }
            let mut out = [0u8; 4];
            for k in 0..3 {
                out[k] = ((color[k] as f32 * sa + px[k] as f32 * da * (1.0 - sa)) / oa)
                    .round()
                    .clamp(0.0, 255.0) as u8;
            }
            out[3] = (oa * 255.0).round().clamp(0.0, 255.0) as u8;
            out
        });
    }

    /// Clear the selected pixels of the active layer to transparent.
    pub fn clear_selection(&mut self) {
        self.edit_active_pixels("Clear", |_x, _y, _px, _c| [0, 0, 0, 0]);
    }

    // --- selection -----------------------------------------------------------------------

    pub fn select_all(&mut self) {
        self.push_undo("Select All");
        self.sel = None;
        self.invalidate_all();
    }

    pub fn select_none(&mut self) {
        if self.sel.is_some() {
            self.push_undo("Deselect");
            self.sel = None;
            self.invalidate_all();
        }
    }

    pub fn invert_selection(&mut self) {
        self.push_undo("Invert Selection");
        let w = self.w;
        let h = self.h;
        let current = self.sel.clone();
        let mask = self.selection_mask_mut_public();
        for y in 0..h {
            for x in 0..w {
                let i = (y as usize) * (w as usize) + x as usize;
                let v = match &current {
                    None => 0u8,
                    Some(m) => m[i],
                };
                mask[i] = 255 - v;
            }
        }
        self.invalidate_all();
    }

    pub fn select_rect(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, feather: f32) {
        self.push_undo("Rectangular Selection");
        let (bx0, by0, bx1, by1) = (
            x0.min(x1).floor() as i32,
            y0.min(y1).floor() as i32,
            x0.max(x1).ceil() as i32,
            y0.max(y1).ceil() as i32,
        );
        let (w, h) = (self.w, self.h);
        let mask = self.selection_mask_mut_public();
        for v in mask.iter_mut() {
            *v = 0;
        }
        for y in by0.max(0)..by1.min(h) {
            for x in bx0.max(0)..bx1.min(w) {
                mask[(y as usize) * (w as usize) + x as usize] = 255;
            }
        }
        if feather > 0.5 {
            self.feather_selection(0.0, feather);
        }
        self.invalidate_all();
    }

    pub fn select_ellipse(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, feather: f32) {
        self.push_undo("Elliptical Selection");
        let cx = (x0 + x1) * 0.5;
        let cy = (y0 + y1) * 0.5;
        let rx = ((x1 - x0).abs() * 0.5).max(0.5);
        let ry = ((y1 - y0).abs() * 0.5).max(0.5);
        let w = self.w;
        let h = self.h;
        let mask = self.selection_mask_mut_public();
        for v in mask.iter_mut() {
            *v = 0;
        }
        for y in 0..h {
            for x in 0..w {
                let nx = (x as f32 + 0.5 - cx) / rx;
                let ny = (y as f32 + 0.5 - cy) / ry;
                if nx * nx + ny * ny <= 1.0 {
                    mask[(y as usize) * (w as usize) + x as usize] = 255;
                }
            }
        }
        if feather > 0.5 {
            self.feather_selection(0.0, feather);
        }
        self.invalidate_all();
    }

    /// A polygonal (lasso) selection from a flat `x0, y0, x1, y1, ...` list.
    pub fn select_polygon(&mut self, points: &[f32], feather: f32) {
        if points.len() < 6 || points.len() % 2 != 0 {
            return;
        }
        self.push_undo("Lasso Selection");
        let w = self.w;
        let h = self.h;
        let mask = self.selection_mask_mut_public();
        for v in mask.iter_mut() {
            *v = 0;
        }
        let n = points.len() / 2;
        for y in 0..h {
            let py = y as f32 + 0.5;
            for x in 0..w {
                let px = x as f32 + 0.5;
                // Even-odd crossing test.
                let mut inside = false;
                let mut j = n - 1;
                for i in 0..n {
                    let (xi, yi) = (points[i * 2], points[i * 2 + 1]);
                    let (xj, yj) = (points[j * 2], points[j * 2 + 1]);
                    if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
                        inside = !inside;
                    }
                    j = i;
                }
                if inside {
                    mask[(y as usize) * (w as usize) + x as usize] = 255;
                }
            }
        }
        if feather > 0.5 {
            self.feather_selection(0.0, feather);
        }
        self.invalidate_all();
    }

    /// Soften the selection edge over `radius` pixels (a separable box blur, twice).
    pub fn feather_selection(&mut self, _unused: f32, radius: f32) {
        let r = radius.round().clamp(1.0, 64.0) as i32;
        let (w, h) = (self.w, self.h);
        let mut src = {
            let m = self.selection_mask_mut_public();
            m.clone()
        };
        for _ in 0..2 {
            src = box_blur_u8(&src, w, h, r);
        }
        let mask = self.selection_mask_mut_public();
        mask.copy_from_slice(&src);
        self.invalidate_all();
    }

    /// Turn the current selection into a layer mask on the active layer.
    pub fn selection_to_mask(&mut self) {
        if self.layers.is_empty() {
            return;
        }
        self.push_undo("Apply Selection as Mask");
        let w = self.w;
        let h = self.h;
        let sel = self.sel.clone();
        let idx = self.active.min(self.layers.len() - 1);
        let layer = &mut self.layers[idx];
        let (lw, lh, ox, oy) = (layer.w, layer.h, layer.offset.0, layer.offset.1);
        let mut mask = vec![0u8; (lw as usize) * (lh as usize)];
        for ly in 0..lh {
            for lx in 0..lw {
                let dx = lx + ox;
                let dy = ly + oy;
                let v = match &sel {
                    None => 255u8,
                    Some(m) => {
                        if dx < 0 || dy < 0 || dx >= w || dy >= h {
                            0
                        } else {
                            m[(dy as usize) * (w as usize) + dx as usize]
                        }
                    }
                };
                mask[(ly as usize) * (lw as usize) + lx as usize] = v;
            }
        }
        layer.mask = Some(Arc::new(mask));
        self.invalidate_all();
    }

    // --- adjustments ---------------------------------------------------------------------

    /// `params` meaning per adjustment:
    /// - BrightnessContrast: [brightness -100..100, contrast -100..100]
    /// - Levels: [black 0..255, white 0..255, gamma 0.1..3, out_black 0..255, out_white 0..255]
    /// - Curves: [gamma 0.1..3, lift -100..100]
    /// - HueSaturation: [hue -180..180, saturation -100..100, lightness -100..100]
    /// - Threshold: [level 0..255]
    /// - Temperature: [-100 cool .. 100 warm]
    /// - Invert / Desaturate / AutoLevels: no parameters
    pub fn adjust(&mut self, kind: u8, params: &[f32]) {
        let p = |i: usize, d: f32| -> f32 { *params.get(i).unwrap_or(&d) };
        match kind {
            0 => {
                let bright = p(0, 0.0) * 2.55;
                let contrast = p(1, 0.0) * 2.55;
                let cf = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast));
                let lut: Vec<u8> = (0..256)
                    .map(|i| (cf * (i as f32 + bright - 128.0) + 128.0).clamp(0.0, 255.0) as u8)
                    .collect();
                self.edit_active_pixels("Brightness/Contrast", move |_x, _y, px, c| {
                    [
                        lerp_u8(px[0], lut[px[0] as usize], c),
                        lerp_u8(px[1], lut[px[1] as usize], c),
                        lerp_u8(px[2], lut[px[2] as usize], c),
                        px[3],
                    ]
                });
            }
            1 => {
                let black = p(0, 0.0).clamp(0.0, 254.0);
                let white = p(1, 255.0).clamp(black + 1.0, 255.0);
                let gamma = p(2, 1.0).clamp(0.1, 3.0);
                let ob = p(3, 0.0).clamp(0.0, 255.0);
                let ow = p(4, 255.0).clamp(ob, 255.0);
                let lut: Vec<u8> = (0..256)
                    .map(|i| {
                        let mut v = (i as f32 - black) / (white - black);
                        v = v.clamp(0.0, 1.0).powf(1.0 / gamma);
                        (ob + v * (ow - ob)).clamp(0.0, 255.0) as u8
                    })
                    .collect();
                self.edit_active_pixels("Levels", move |_x, _y, px, c| {
                    [
                        lerp_u8(px[0], lut[px[0] as usize], c),
                        lerp_u8(px[1], lut[px[1] as usize], c),
                        lerp_u8(px[2], lut[px[2] as usize], c),
                        px[3],
                    ]
                });
            }
            2 => {
                let gamma = p(0, 1.0).clamp(0.1, 3.0);
                let lift = p(1, 0.0) * 1.28;
                let lut: Vec<u8> = (0..256)
                    .map(|i| {
                        let v = (i as f32 / 255.0).powf(1.0 / gamma) * 255.0 + lift;
                        v.clamp(0.0, 255.0) as u8
                    })
                    .collect();
                self.edit_active_pixels("Curves", move |_x, _y, px, c| {
                    [
                        lerp_u8(px[0], lut[px[0] as usize], c),
                        lerp_u8(px[1], lut[px[1] as usize], c),
                        lerp_u8(px[2], lut[px[2] as usize], c),
                        px[3],
                    ]
                });
            }
            3 => {
                let dh = p(0, 0.0) / 360.0;
                let ds = p(1, 0.0) / 100.0;
                let dl = p(2, 0.0) / 100.0;
                self.edit_active_pixels("Hue/Saturation", move |_x, _y, px, c| {
                    let (h, s, l) = rgb_to_hsl(px[0], px[1], px[2]);
                    let mut h2 = h + dh;
                    if h2 < 0.0 {
                        h2 += 1.0;
                    }
                    if h2 > 1.0 {
                        h2 -= 1.0;
                    }
                    let s2 = clamp01(s + ds * (if ds >= 0.0 { 1.0 - s } else { s }));
                    let l2 = clamp01(l + dl * (if dl >= 0.0 { 1.0 - l } else { l }));
                    let (r, g, b) = hsl_to_rgb(h2, s2, l2);
                    [
                        lerp_u8(px[0], r, c),
                        lerp_u8(px[1], g, c),
                        lerp_u8(px[2], b, c),
                        px[3],
                    ]
                });
            }
            4 => {
                let level = p(0, 128.0).clamp(0.0, 255.0);
                self.edit_active_pixels("Threshold", move |_x, _y, px, c| {
                    let lum = 0.299 * px[0] as f32 + 0.587 * px[1] as f32 + 0.114 * px[2] as f32;
                    let v = if lum >= level { 255u8 } else { 0u8 };
                    [lerp_u8(px[0], v, c), lerp_u8(px[1], v, c), lerp_u8(px[2], v, c), px[3]]
                });
            }
            5 => {
                self.edit_active_pixels("Invert", |_x, _y, px, c| {
                    [
                        lerp_u8(px[0], 255 - px[0], c),
                        lerp_u8(px[1], 255 - px[1], c),
                        lerp_u8(px[2], 255 - px[2], c),
                        px[3],
                    ]
                });
            }
            6 => {
                self.edit_active_pixels("Desaturate", |_x, _y, px, c| {
                    let lum = (0.299 * px[0] as f32 + 0.587 * px[1] as f32 + 0.114 * px[2] as f32)
                        .round()
                        .clamp(0.0, 255.0) as u8;
                    [lerp_u8(px[0], lum, c), lerp_u8(px[1], lum, c), lerp_u8(px[2], lum, c), px[3]]
                });
            }
            7 => {
                // Auto levels: stretch each channel between its 1% and 99% percentiles.
                let mut luts: [Vec<u8>; 3] = [vec![0; 256], vec![0; 256], vec![0; 256]];
                if !self.layers.is_empty() {
                    let idx = self.active.min(self.layers.len() - 1);
                    let layer = &self.layers[idx];
                    let mut hist = [[0u32; 256]; 3];
                    let mut total = 0u32;
                    for i in (0..layer.px.len()).step_by(4) {
                        if layer.px[i + 3] == 0 {
                            continue;
                        }
                        for k in 0..3 {
                            hist[k][layer.px[i + k] as usize] += 1;
                        }
                        total += 1;
                    }
                    if total > 0 {
                        for k in 0..3 {
                            let low = (total / 100).max(1);
                            let high = total - low;
                            let mut acc = 0u32;
                            let mut lo = 0usize;
                            let mut hi = 255usize;
                            for v in 0..256 {
                                acc += hist[k][v];
                                if acc >= low {
                                    lo = v;
                                    break;
                                }
                            }
                            acc = 0;
                            for v in (0..256).rev() {
                                acc += hist[k][v];
                                if acc >= total - high.min(total - 1) {
                                    hi = v;
                                    break;
                                }
                            }
                            if hi <= lo {
                                hi = (lo + 1).min(255);
                            }
                            for v in 0..256 {
                                let t = ((v as f32 - lo as f32) / (hi as f32 - lo as f32)).clamp(0.0, 1.0);
                                luts[k][v] = (t * 255.0).round() as u8;
                            }
                        }
                    }
                }
                self.edit_active_pixels("Auto Levels", move |_x, _y, px, c| {
                    [
                        lerp_u8(px[0], luts[0][px[0] as usize], c),
                        lerp_u8(px[1], luts[1][px[1] as usize], c),
                        lerp_u8(px[2], luts[2][px[2] as usize], c),
                        px[3],
                    ]
                });
            }
            8 => {
                let t = p(0, 0.0).clamp(-100.0, 100.0) / 100.0;
                let rl = (t * 40.0) as i32;
                let bl = (-t * 40.0) as i32;
                self.edit_active_pixels("Temperature", move |_x, _y, px, c| {
                    let rr = (px[0] as i32 + rl).clamp(0, 255) as u8;
                    let bb = (px[2] as i32 + bl).clamp(0, 255) as u8;
                    [lerp_u8(px[0], rr, c), px[1], lerp_u8(px[2], bb, c), px[3]]
                });
            }
            _ => {}
        }
    }

    // --- filters -------------------------------------------------------------------------

    /// `params`, per filter:
    /// - Blur: [radius px]
    /// - Sharpen: [amount 0..200]
    /// - Noise: [amount 0..100, seed]
    /// - Pixelate: [cell px]
    pub fn filter(&mut self, kind: u8, params: &[f32]) {
        match kind {
            0 => {
                let radius = params.first().copied().unwrap_or(4.0).clamp(0.5, 120.0);
                self.blur_layer(radius);
            }
            1 => {
                let amount = params.first().copied().unwrap_or(60.0).clamp(0.0, 200.0) / 100.0;
                self.sharpen_layer(amount);
            }
            2 => {
                let amount = params.first().copied().unwrap_or(12.0).clamp(0.0, 100.0);
                let seed = params.get(1).copied().unwrap_or(1.0) as u32;
                let mut state = seed.wrapping_mul(2654435761).wrapping_add(1);
                self.edit_active_pixels("Noise", move |_x, _y, px, c| {
                    state ^= state << 13;
                    state ^= state >> 17;
                    state ^= state << 5;
                    let noise = ((state & 0xFF) as f32 / 255.0 - 0.5) * 2.0 * amount * 2.55;
                    let mut out = px;
                    for k in 0..3 {
                        let v = (px[k] as f32 + noise).clamp(0.0, 255.0) as u8;
                        out[k] = lerp_u8(px[k], v, c);
                    }
                    out
                });
            }
            3 => {
                let cell = params.first().copied().unwrap_or(12.0).clamp(2.0, 128.0);
                self.pixelate_layer(cell);
            }
            _ => {}
        }
    }

    fn blur_layer(&mut self, radius: f32) {
        if self.layers.is_empty() {
            return;
        }
        let idx = self.active.min(self.layers.len() - 1);
        self.push_undo("Blur");
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let r = radius.round().max(1.0) as i32;
        let bounds = {
            let layer = &mut self.layers[idx];
            let blurred = box_blur_rgba(&layer.px, layer.w, layer.h, r);
            let (lw, lh, ox, oy) = (layer.w, layer.h, layer.offset.0, layer.offset.1);
            let buf = layer.pixels_mut();
            for ly in 0..lh {
                for lx in 0..lw {
                    let c = cov(&sel, dw, dh, lx + ox, ly + oy);
                    if c <= 0.0 {
                        continue;
                    }
                    let o = ((ly as usize) * (lw as usize) + lx as usize) * 4;
                    for k in 0..4 {
                        buf[o + k] = lerp_u8(buf[o + k], blurred[o + k], c);
                    }
                }
            }
            Rect::new(0, 0, self.w, self.h)
        };
        self.mark_dirty(bounds);
    }

    fn sharpen_layer(&mut self, amount: f32) {
        if self.layers.is_empty() {
            return;
        }
        let idx = self.active.min(self.layers.len() - 1);
        self.push_undo("Sharpen");
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let bounds = {
            let layer = &mut self.layers[idx];
            let blurred = box_blur_rgba(&layer.px, layer.w, layer.h, 2);
            let (lw, lh, ox, oy) = (layer.w, layer.h, layer.offset.0, layer.offset.1);
            let buf = layer.pixels_mut();
            for ly in 0..lh {
                for lx in 0..lw {
                    let c = cov(&sel, dw, dh, lx + ox, ly + oy);
                    if c <= 0.0 {
                        continue;
                    }
                    let o = ((ly as usize) * (lw as usize) + lx as usize) * 4;
                    for k in 0..3 {
                        let orig = buf[o + k] as f32;
                        let blur = blurred[o + k] as f32;
                        let sharp = (orig + amount * (orig - blur)).clamp(0.0, 255.0);
                        buf[o + k] = lerp_u8(buf[o + k], sharp as u8, c);
                    }
                }
            }
            Rect::new(0, 0, self.w, self.h)
        };
        self.mark_dirty(bounds);
    }

    fn pixelate_layer(&mut self, cell: f32) {
        if self.layers.is_empty() {
            return;
        }
        let idx = self.active.min(self.layers.len() - 1);
        self.push_undo("Pixelate");
        let sel = self.sel.clone();
        let (dw, dh) = (self.w, self.h);
        let c = cell.round().max(1.0) as i32;
        let bounds = {
            let layer = &mut self.layers[idx];
            let src = layer.px.clone();
            let (lw, lh, ox, oy) = (layer.w, layer.h, layer.offset.0, layer.offset.1);
            let buf = layer.pixels_mut();
            for by in (0..lh).step_by(c as usize) {
                for bx in (0..lw).step_by(c as usize) {
                    // Average the cell, weighted by alpha so transparent pixels do not bleed in.
                    let (mut r, mut g, mut b, mut a, mut n) = (0f32, 0f32, 0f32, 0f32, 0f32);
                    for y in by..(by + c).min(lh) {
                        for x in bx..(bx + c).min(lw) {
                            let o = ((y as usize) * (lw as usize) + x as usize) * 4;
                            let al = src[o + 3] as f32 / 255.0;
                            r += src[o] as f32 * al;
                            g += src[o + 1] as f32 * al;
                            b += src[o + 2] as f32 * al;
                            a += al;
                            n += 1.0;
                        }
                    }
                    if n == 0.0 || a <= 0.0 {
                        continue;
                    }
                    let px = [
                        (r / a).clamp(0.0, 255.0) as u8,
                        (g / a).clamp(0.0, 255.0) as u8,
                        (b / a).clamp(0.0, 255.0) as u8,
                        ((a / n) * 255.0).clamp(0.0, 255.0) as u8,
                    ];
                    for y in by..(by + c).min(lh) {
                        for x in bx..(bx + c).min(lw) {
                            let cv = cov(&sel, dw, dh, x + ox, y + oy);
                            if cv <= 0.0 {
                                continue;
                            }
                            let o = ((y as usize) * (lw as usize) + x as usize) * 4;
                            for k in 0..4 {
                                buf[o + k] = lerp_u8(buf[o + k], px[k], cv);
                            }
                        }
                    }
                }
            }
            Rect::new(0, 0, self.w, self.h)
        };
        self.mark_dirty(bounds);
    }

    // --- transforms ----------------------------------------------------------------------

    pub fn flip(&mut self, horizontal: bool) {
        let sel_bounds = self.selection_bounds();
        if self.layers.is_empty() {
            return;
        }
        self.push_undo(if horizontal { "Flip Horizontal" } else { "Flip Vertical" });
        let idx = self.active.min(self.layers.len() - 1);
        let bounds = {
            let layer = &mut self.layers[idx];
            let src = layer.px.clone();
            let (lw, lh) = (layer.w, layer.h);
            let buf = layer.pixels_mut();
            for y in 0..lh {
                for x in 0..lw {
                    let (sx, sy) = if horizontal { (lw - 1 - x, y) } else { (x, lh - 1 - y) };
                    let so = ((sy as usize) * (lw as usize) + sx as usize) * 4;
                    let dofs = ((y as usize) * (lw as usize) + x as usize) * 4;
                    for k in 0..4 {
                        buf[dofs + k] = src[so + k];
                    }
                }
            }
            sel_bounds
        };
        self.mark_dirty(bounds);
    }

    /// Rotate the active layer 90° clockwise; its buffer swaps width and height.
    pub fn rotate90(&mut self) {
        if self.layers.is_empty() {
            return;
        }
        self.push_undo("Rotate 90°");
        let idx = self.active.min(self.layers.len() - 1);
        let layer = &mut self.layers[idx];
        let (lw, lh) = (layer.w, layer.h);
        let src = layer.px.clone();
        let mut out = vec![0u8; (lw as usize) * (lh as usize) * 4];
        for y in 0..lh {
            for x in 0..lw {
                // (x, y) -> (lh - 1 - y, x) in the rotated image.
                let nx = lh - 1 - y;
                let ny = x;
                let so = ((y as usize) * (lw as usize) + x as usize) * 4;
                let dofs = ((ny as usize) * (lh as usize) + nx as usize) * 4;
                for k in 0..4 {
                    out[dofs + k] = src[so + k];
                }
            }
        }
        layer.w = lh;
        layer.h = lw;
        layer.px = Arc::new(out);
        let old_mask = layer.mask.clone();
        if let Some(m) = &old_mask {
            let mut nm = vec![0u8; m.len()];
            for y in 0..lh {
                for x in 0..lw {
                    let nx = lh - 1 - y;
                    let ny = x;
                    nm[(ny as usize) * (lh as usize) + nx as usize] = m[(y as usize) * (lw as usize) + x as usize];
                }
            }
            layer.mask = Some(Arc::new(nm));
        }
        self.invalidate_all();
    }

    pub fn nudge_layer(&mut self, dx: i32, dy: i32) {
        if self.layers.is_empty() {
            return;
        }
        self.push_undo("Move Layer");
        let idx = self.active.min(self.layers.len() - 1);
        let layer = &mut self.layers[idx];
        layer.offset.0 += dx;
        layer.offset.1 += dy;
        self.invalidate_all();
    }

    // --- canvas --------------------------------------------------------------------------

    /// Crop the document to `r`, moving every layer into the new coordinate space.
    pub fn crop(&mut self, r: Rect) {
        let r = r.clamp(self.w, self.h);
        if r.is_empty() {
            return;
        }
        self.push_undo("Crop");
        for layer in self.layers.iter_mut() {
            let nx0 = (r.x0 - layer.offset.0).clamp(0, layer.w);
            let ny0 = (r.y0 - layer.offset.1).clamp(0, layer.h);
            let nx1 = (r.x1 - layer.offset.0).clamp(0, layer.w);
            let ny1 = (r.y1 - layer.offset.1).clamp(0, layer.h);
            let (nw, nh) = (r.width(), r.height());
            if nx1 <= nx0 || ny1 <= ny0 {
                layer.w = nw;
                layer.h = nh;
                layer.px = Arc::new(vec![0u8; (nw as usize) * (nh as usize) * 4]);
                layer.mask = None;
                layer.offset = (0, 0);
                continue;
            }
            let mut out = vec![0u8; (nw as usize) * (nh as usize) * 4];
            let mut out_mask: Option<Vec<u8>> = layer.mask.as_ref().map(|_| vec![0u8; (nw as usize) * (nh as usize)]);
            for y in ny0..ny1 {
                for x in nx0..nx1 {
                    let so = ((y as usize) * (layer.w as usize) + x as usize) * 4;
                    let dx = x + layer.offset.0 - r.x0;
                    let dy = y + layer.offset.1 - r.y0;
                    if dx < 0 || dy < 0 || dx >= nw || dy >= nh {
                        continue;
                    }
                    let dofs = ((dy as usize) * (nw as usize) + dx as usize) * 4;
                    for k in 0..4 {
                        out[dofs + k] = layer.px[so + k];
                    }
                    if let (Some(m), Some(om)) = (&layer.mask, out_mask.as_mut()) {
                        om[(dy as usize) * (nw as usize) + dx as usize] =
                            m[(y as usize) * (layer.w as usize) + x as usize];
                    }
                }
            }
            layer.w = nw;
            layer.h = nh;
            layer.px = Arc::new(out);
            layer.mask = out_mask.map(Arc::new);
            layer.offset = (0, 0);
        }
        self.w = r.width();
        self.h = r.height();
        self.sel = None;
        self.invalidate_all();
    }

    /// Resize the canvas, scaling every layer bilinearly and anchoring at the top-left.
    pub fn resize_canvas(&mut self, w: i32, h: i32) {
        let w = w.clamp(1, 16384);
        let h = h.clamp(1, 16384);
        if w == self.w && h == self.h {
            return;
        }
        self.push_undo("Image Size");
        for layer in self.layers.iter_mut() {
            let mut out = vec![0u8; (w as usize) * (h as usize) * 4];
            for y in 0..h {
                let sy = (y as f32) * (layer.h as f32) / (h as f32);
                for x in 0..w {
                    let sx = (x as f32) * (layer.w as f32) / (w as f32);
                    let x0 = sx.floor() as i32;
                    let y0 = sy.floor() as i32;
                    let tx = sx - x0 as f32;
                    let ty = sy - y0 as f32;
                    let x1 = (x0 + 1).min(layer.w - 1);
                    let y1 = (y0 + 1).min(layer.h - 1);
                    let p00 = layer.pixel(x0.min(layer.w - 1), y0.min(layer.h - 1));
                    let p10 = layer.pixel(x1, y0.min(layer.h - 1));
                    let p01 = layer.pixel(x0.min(layer.w - 1), y1);
                    let p11 = layer.pixel(x1, y1);
                    let dofs = ((y as usize) * (w as usize) + x as usize) * 4;
                    for k in 0..4 {
                        let top = p00[k] as f32 * (1.0 - tx) + p10[k] as f32 * tx;
                        let bot = p01[k] as f32 * (1.0 - tx) + p11[k] as f32 * tx;
                        out[dofs + k] = (top * (1.0 - ty) + bot * ty).round().clamp(0.0, 255.0) as u8;
                    }
                }
            }
            layer.offset = (0, 0);
            layer.w = w;
            layer.h = h;
            layer.px = Arc::new(out);
            layer.mask = None;
        }
        self.w = w;
        self.h = h;
        self.sel = None;
        self.invalidate_all();
    }
}

/// Separable box blur of a single-channel mask, `passes` is fixed at one here; the feather calls
/// it twice for a near-Gaussian falloff.
fn box_blur_u8(src: &[u8], w: i32, h: i32, r: i32) -> Vec<u8> {
    let mut tmp = vec![0u8; src.len()];
    let mut out = vec![0u8; src.len()];
    let rad = r.max(1);
    // Horizontal.
    for y in 0..h {
        let row = (y as usize) * (w as usize);
        for x in 0..w {
            let mut acc = 0u32;
            let mut n = 0u32;
            for k in -rad..=rad {
                let xx = x + k;
                if xx < 0 || xx >= w {
                    continue;
                }
                acc += src[row + xx as usize] as u32;
                n += 1;
            }
            tmp[row + x as usize] = if n > 0 { (acc / n) as u8 } else { 0 };
        }
    }
    // Vertical.
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0u32;
            let mut n = 0u32;
            for k in -rad..=rad {
                let yy = y + k;
                if yy < 0 || yy >= h {
                    continue;
                }
                acc += tmp[(yy as usize) * (w as usize) + x as usize] as u32;
                n += 1;
            }
            out[(y as usize) * (w as usize) + x as usize] = if n > 0 { (acc / n) as u8 } else { 0 };
        }
    }
    out
}

/// Separable box blur of an RGBA buffer. Two passes approximate a Gaussian closely enough for a
/// filter, and it is O(n·r) instead of the true Gaussian's larger constant.
fn box_blur_rgba(src: &[u8], w: i32, h: i32, r: i32) -> Vec<u8> {
    let rad = r.max(1);
    let mut cur = src.to_vec();
    for _ in 0..2 {
        let mut tmp = vec![0u8; cur.len()];
        for y in 0..h {
            let row = (y as usize) * (w as usize);
            for x in 0..w {
                let (mut acc, mut n) = ([0u32; 4], 0u32);
                for k in -rad..=rad {
                    let xx = x + k;
                    if xx < 0 || xx >= w {
                        continue;
                    }
                    let o = (row + xx as usize) * 4;
                    for c in 0..4 {
                        acc[c] += cur[o + c] as u32;
                    }
                    n += 1;
                }
                let o = (row + x as usize) * 4;
                for c in 0..4 {
                    tmp[o + c] = if n > 0 { (acc[c] / n) as u8 } else { 0 };
                }
            }
        }
        let mut out = vec![0u8; tmp.len()];
        for y in 0..h {
            for x in 0..w {
                let (mut acc, mut n) = ([0u32; 4], 0u32);
                for k in -rad..=rad {
                    let yy = y + k;
                    if yy < 0 || yy >= h {
                        continue;
                    }
                    let o = ((yy as usize) * (w as usize) + x as usize) * 4;
                    for c in 0..4 {
                        acc[c] += tmp[o + c] as u32;
                    }
                    n += 1;
                }
                let o = ((y as usize) * (w as usize) + x as usize) * 4;
                for c in 0..4 {
                    out[o + c] = if n > 0 { (acc[c] / n) as u8 } else { 0 };
                }
            }
        }
        cur = out;
    }
    cur
}

/// Re-exported so the JNI layer and the tests can name a layer without importing `doc`.
pub type LayerRef = Layer;

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_with_paint() -> Document {
        let mut d = Document::new(64, 64);
        d.layers[0].pixels_mut().clone_from(&vec![0u8; 64 * 64 * 4]);
        {
            let l = &mut d.layers[0];
            let buf = l.pixels_mut();
            for i in (0..buf.len()).step_by(4) {
                buf[i] = 255;
                buf[i + 3] = 255;
            }
        }
        d.invalidate_all();
        d
    }

    fn px_at(d: &mut Document, x: usize, y: usize) -> [u8; 4] {
        let c = d.composite();
        let o = (y * d.w as usize + x) * 4;
        [c[o], c[o + 1], c[o + 2], c[o + 3]]
    }

    #[test]
    fn a_brush_stroke_paints_the_document_colour() {
        let mut d = doc_with_paint();
        d.stroke_begin(32.0, 32.0, 1.0, 0, [0, 0, 255, 255], 10.0, 1.0, 1.0, 1.0);
        d.stroke_to(40.0, 32.0, 1.0);
        d.stroke_end();
        let p = px_at(&mut d, 36, 32);
        assert_eq!(&p[..3], &[0, 0, 255], "the stroke should be blue at (36,32)");
    }

    #[test]
    fn a_stroke_is_one_undo_step() {
        let mut d = doc_with_paint();
        let before = d.undo.len();
        d.stroke_begin(10.0, 10.0, 1.0, 0, [0, 255, 0, 255], 8.0, 1.0, 1.0, 1.0);
        d.stroke_to(20.0, 10.0, 1.0);
        d.stroke_to(30.0, 20.0, 1.0);
        d.stroke_end();
        assert_eq!(d.undo.len(), before + 1, "a whole stroke is a single step");
    }

    #[test]
    fn the_eraser_removes_alpha() {
        let mut d = doc_with_paint();
        d.stroke_begin(32.0, 32.0, 1.0, 1, [0, 0, 0, 255], 16.0, 1.0, 1.0, 1.0);
        d.stroke_end();
        let p = px_at(&mut d, 32, 32);
        assert_eq!(p[3], 0, "the eraser should clear alpha");
    }

    #[test]
    fn painting_outside_the_selection_does_nothing() {
        let mut d = doc_with_paint();
        d.select_rect(0.0, 0.0, 32.0, 32.0, 0.0);
        d.stroke_begin(50.0, 50.0, 1.0, 0, [0, 255, 0, 255], 12.0, 1.0, 1.0, 1.0);
        d.stroke_end();
        let inside = px_at(&mut d, 50, 50);
        assert_eq!(&inside[..3], &[255, 0, 0], "outside the selection stays red");
        d.stroke_begin(10.0, 10.0, 1.0, 0, [0, 255, 0, 255], 12.0, 1.0, 1.0, 1.0);
        d.stroke_end();
        let outside = px_at(&mut d, 10, 10);
        assert_eq!(&outside[..3], &[0, 255, 0], "inside the selection paints");
    }

    #[test]
    fn bucket_fill_spreads_over_similar_pixels_only() {
        let mut d = doc_with_paint();
        // A hard blue wall splits the canvas.
        {
            let l = &mut d.layers[0];
            let buf = l.pixels_mut();
            for y in 0..64 {
                for x in 30..34 {
                    let o = (y * 64 + x) * 4;
                    buf[o] = 0;
                    buf[o + 1] = 0;
                    buf[o + 2] = 255;
                }
            }
        }
        d.invalidate_all();
        d.bucket_fill(5, 5, [0, 255, 0, 255], 10);
        assert_eq!(&px_at(&mut d, 5, 5)[..3], &[0, 255, 0], "the near side fills");
        assert_eq!(&px_at(&mut d, 60, 60)[..3], &[255, 0, 0], "the wall stops the fill");
    }

    #[test]
    fn rect_and_ellipse_selections_cover_expected_pixels() {
        let mut d = Document::new(40, 40);
        d.select_rect(5.0, 5.0, 15.0, 15.0, 0.0);
        assert_eq!(d.selection_at(10, 10), 255);
        assert_eq!(d.selection_at(20, 20), 0);
        d.select_ellipse(20.0, 20.0, 38.0, 38.0, 0.0);
        assert_eq!(d.selection_at(29, 29), 255, "the ellipse centre is selected");
        assert_eq!(d.selection_at(21, 21), 0, "the ellipse corner is not");
    }

    #[test]
    fn a_lasso_selection_is_closed_and_filled() {
        let mut d = Document::new(20, 20);
        d.select_polygon(&[2.0, 2.0, 16.0, 2.0, 16.0, 16.0, 2.0, 16.0], 0.0);
        assert_eq!(d.selection_at(9, 9), 255);
        assert_eq!(d.selection_at(0, 0), 0);
    }

    #[test]
    fn inverting_a_selection_swaps_inside_and_outside() {
        let mut d = Document::new(20, 20);
        d.select_rect(0.0, 0.0, 10.0, 20.0, 0.0);
        d.invert_selection();
        assert_eq!(d.selection_at(2, 2), 0);
        assert_eq!(d.selection_at(15, 5), 255);
    }

    #[test]
    fn feathering_softens_the_edge() {
        let mut d = Document::new(40, 40);
        d.select_rect(0.0, 0.0, 20.0, 40.0, 0.0);
        let hard = d.selection_at(19, 20);
        d.feather_selection(0.0, 3.0);
        let soft = d.selection_at(19, 20);
        assert_eq!(hard, 255);
        assert!(soft < 255 && soft > 0, "the edge should be partial: {}", soft);
    }

    #[test]
    fn a_selection_can_become_a_layer_mask() {
        let mut d = doc_with_paint();
        d.select_rect(0.0, 0.0, 20.0, 20.0, 0.0);
        d.selection_to_mask();
        assert!(d.layers[0].mask.is_some());
        let p = px_at(&mut d, 30, 30);
        assert_eq!(p[3], 0, "masked-out pixels disappear from the composite");
    }

    #[test]
    fn levels_lift_the_black_point() {
        let mut d = doc_with_paint();
        d.adjust(1, &[128.0, 255.0, 1.0, 0.0, 255.0]);
        let p = px_at(&mut d, 10, 10);
        assert!(p[0] > 200, "a red of 255 stays near white after a black point of 128: {:?}", p);
    }

    #[test]
    fn invert_flips_the_channels() {
        let mut d = doc_with_paint();
        d.adjust(5, &[]);
        let p = px_at(&mut d, 10, 10);
        assert_eq!(&p[..3], &[0, 255, 255], "red inverted is cyan");
    }

    #[test]
    fn desaturate_makes_channels_equal() {
        let mut d = doc_with_paint();
        d.adjust(6, &[]);
        let p = px_at(&mut d, 10, 10);
        assert_eq!(p[0], p[1]);
        assert_eq!(p[1], p[2]);
    }

    #[test]
    fn threshold_is_black_or_white_only() {
        let mut d = doc_with_paint();
        d.adjust(4, &[128.0]);
        let p = px_at(&mut d, 10, 10);
        assert!(p[0] == 0 || p[0] == 255);
    }

    #[test]
    fn hue_rotation_changes_the_colour_but_not_the_alpha() {
        let mut d = doc_with_paint();
        d.adjust(3, &[120.0, 0.0, 0.0]);
        let p = px_at(&mut d, 10, 10);
        assert_eq!(p[3], 255);
        assert_ne!(&p[..3], &[255, 0, 0], "the hue should have moved");
    }

    #[test]
    fn brightness_raises_every_channel() {
        let mut d = doc_with_paint();
        let before = px_at(&mut d, 10, 10)[0];
        d.adjust(0, &[40.0, 0.0]);
        let after = px_at(&mut d, 10, 10)[0];
        assert!(after >= before);
    }

    #[test]
    fn blur_smooths_a_hard_edge() {
        let mut d = Document::new(32, 32);
        {
            let l = &mut d.layers[0];
            let buf = l.pixels_mut();
            for y in 0..32 {
                for x in 0..32 {
                    let o = (y * 32 + x) * 4;
                    let v = if x < 16 { 0u8 } else { 255u8 };
                    buf[o] = v;
                    buf[o + 1] = v;
                    buf[o + 2] = v;
                    buf[o + 3] = 255;
                }
            }
        }
        d.invalidate_all();
        d.filter(0, &[6.0]);
        let p = px_at(&mut d, 16, 16);
        assert!(p[0] > 20 && p[0] < 235, "the edge should be grey now: {:?}", p);
    }

    #[test]
    fn pixelate_makes_neighbours_identical() {
        let mut d = doc_with_paint();
        d.filter(3, &[8.0]);
        let a = px_at(&mut d, 8, 8);
        let b = px_at(&mut d, 9, 9);
        assert_eq!(a, b);
    }

    #[test]
    fn sharpen_keeps_the_image_readable() {
        let mut d = doc_with_paint();
        d.filter(1, &[80.0]);
        let p = px_at(&mut d, 10, 10);
        assert_eq!(p[3], 255, "alpha must survive a sharpen");
    }

    #[test]
    fn flip_horizontal_mirrors_pixels() {
        let mut d = Document::new(4, 1);
        {
            let l = &mut d.layers[0];
            let buf = l.pixels_mut();
            // Pixel 0 at the left edge: solid red.
            buf[0] = 255;
            buf[3] = 255;
        }
        d.invalidate_all();
        assert_eq!(d.composite()[0], 255, "the paint starts on the left");
        d.flip(true);
        let c = d.composite();
        assert_eq!(c[0], 0, "the left edge is empty after the flip");
        assert_eq!(c[12], 255, "the paint moved to the right edge");
    }

    #[test]
    fn rotate90_swaps_the_layer_dimensions() {
        let mut d = Document::new(8, 4);
        d.rotate90();
        assert_eq!((d.layers[0].w, d.layers[0].h), (4, 8));
    }

    #[test]
    fn crop_resizes_the_canvas_and_keeps_the_pixels() {
        let mut d = doc_with_paint();
        d.crop(Rect::new(10, 10, 30, 30));
        assert_eq!((d.w, d.h), (20, 20));
        let p = px_at(&mut d, 5, 5);
        assert_eq!(&p[..3], &[255, 0, 0]);
    }

    #[test]
    fn resize_canvas_scales_the_layer_buffers() {
        let mut d = doc_with_paint();
        d.resize_canvas(32, 32);
        assert_eq!((d.w, d.h), (32, 32));
        assert_eq!(d.layers[0].w, 32);
        let p = px_at(&mut d, 16, 16);
        assert_eq!(p[3], 255);
    }

    #[test]
    fn fill_selection_only_touches_selected_pixels() {
        let mut d = doc_with_paint();
        d.select_rect(0.0, 0.0, 16.0, 16.0, 0.0);
        d.fill_selection([0, 0, 255, 255]);
        assert_eq!(&px_at(&mut d, 8, 8)[..3], &[0, 0, 255]);
        assert_eq!(&px_at(&mut d, 40, 40)[..3], &[255, 0, 0]);
    }

    #[test]
    fn rgb_hsl_round_trip_is_lossless_enough() {
        for (r, g, b) in [(255u8, 0u8, 0u8), (12, 200, 90), (128, 128, 128), (3, 4, 5)] {
            let (h, s, l) = rgb_to_hsl(r, g, b);
            let (r2, g2, b2) = hsl_to_rgb(h, s, l);
            assert!(
                (r as i32 - r2 as i32).abs() <= 2,
                "red ({},{},{}) came back as ({},{},{})",
                r, g, b, r2, g2, b2
            );
            assert!((g as i32 - g2 as i32).abs() <= 2);
            assert!((b as i32 - b2 as i32).abs() <= 2);
        }
    }
}
