//! Color spaces and palette generation.

use super::map3base::*;
use super::math::*;
use super::*;
extern crate alloc;
use alloc::{boxed::Box, string::String, vec::Vec};

// Okhsv and Okhsl conversions in this file are based on code by Björn Ottosson.
// Here is the original copyright notice:
//
// Copyright(c) 2021 Björn Ottosson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files(the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and /or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub fn srgb_transfer_function(a: f32) -> f32 {
    let a = clamp01(a);
    if 0.0031308 >= a {
        12.92 * a
    } else {
        1.055 * pow(a, 0.4166666666666667) - 0.055
    }
}

fn compute_max_saturation(a: f32, b: f32) -> f32 {
    let (k0, k1, k2, k3, k4, wl, wm, ws) = if -1.88170328 * a - 0.80936493 * b > 1.0 {
        (
            1.19086277,
            1.76576728,
            0.59662641,
            0.75515197,
            0.56771245,
            4.076741662,
            -3.3077115913,
            0.2309699292,
        )
    } else if 1.81444104 * a - 1.19445276 * b > 1.0 {
        (
            0.73956515,
            -0.45954404,
            0.08285427,
            0.12541070,
            0.14503204,
            -1.2684380046,
            2.6097574011,
            -0.3413193965,
        )
    } else {
        (
            1.35733652,
            -0.00915799,
            -1.15130210,
            -0.50559606,
            0.00692167,
            -0.0041960863,
            -0.7034186147,
            1.7076147010,
        )
    };

    let S = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    let k_l = 0.3963377774 * a + 0.2158037573 * b;
    let k_m = -0.1055613458 * a - 0.0638541728 * b;
    let k_s = -0.0894841775 * a - 1.2914855480 * b;

    let l_ = 1.0 + S * k_l;
    let m_ = 1.0 + S * k_m;
    let s_ = 1.0 + S * k_s;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let l_ds = 3.0 * k_l * l_ * l_;
    let m_ds = 3.0 * k_m * m_ * m_;
    let s_ds = 3.0 * k_s * s_ * s_;

    let l_ds2 = 6.0 * k_l * k_l * l_;
    let m_ds2 = 6.0 * k_m * k_m * m_;
    let s_ds2 = 6.0 * k_s * k_s * s_;

    let f = wl * l + wm * m + ws * s;
    let f1 = wl * l_ds + wm * m_ds + ws * s_ds;
    let f2 = wl * l_ds2 + wm * m_ds2 + ws * s_ds2;

    S - f * f1 / (f1 * f1 - 0.5 * f * f2)
}

fn oklab_to_linear_srgb(L: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    (
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    )
}

// returns (L, C)
fn find_cusp(a: f32, b: f32) -> (f32, f32) {
    let S_cusp = compute_max_saturation(a, b);

    let (r, g, b) = oklab_to_linear_srgb(1.0, S_cusp * a, S_cusp * b);
    let L_cusp = pow(1.0 / max(max(r, g), b), 1.0 / 3.0);
    let C_cusp = L_cusp * S_cusp;

    (L_cusp, C_cusp)
}

fn find_gamut_intersection(
    a: f32,
    b: f32,
    L1: f32,
    C1: f32,
    L0: f32,
    cusp_L: f32,
    cusp_C: f32,
) -> f32 {
    if ((L1 - L0) * cusp_C - (cusp_L - L0) * C1) <= 0.0 {
        cusp_C * L0 / (C1 * cusp_L + cusp_C * (L0 - L1))
    } else {
        let t = cusp_C * (L0 - 1.0) / (C1 * (cusp_L - 1.0) + cusp_C * (L0 - L1));
        {
            let dL = L1 - L0;
            let dC = C1;

            let k_l = 0.3963377774 * a + 0.2158037573 * b;
            let k_m = -0.1055613458 * a - 0.0638541728 * b;
            let k_s = -0.0894841775 * a - 1.2914855480 * b;

            let l_dt = dL + dC * k_l;
            let m_dt = dL + dC * k_m;
            let s_dt = dL + dC * k_s;

            {
                let L = L0 * (1.0 - t) + t * L1;
                let C = t * C1;

                let l_ = L + C * k_l;
                let m_ = L + C * k_m;
                let s_ = L + C * k_s;

                let l = l_ * l_ * l_;
                let m = m_ * m_ * m_;
                let s = s_ * s_ * s_;

                let ldt = 3.0 * l_dt * l_ * l_;
                let mdt = 3.0 * m_dt * m_ * m_;
                let sdt = 3.0 * s_dt * s_ * s_;

                let ldt2 = 6.0 * l_dt * l_dt * l_;
                let mdt2 = 6.0 * m_dt * m_dt * m_;
                let sdt2 = 6.0 * s_dt * s_dt * s_;

                let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s - 1.0;
                let r1 = 4.0767416621 * ldt - 3.3077115913 * mdt + 0.2309699292 * sdt;
                let r2 = 4.0767416621 * ldt2 - 3.3077115913 * mdt2 + 0.2309699292 * sdt2;

                let u_r = r1 / (r1 * r1 - 0.5 * r * r2);
                let t_r = -r * u_r;

                let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s - 1.0;
                let g1 = -1.2684380046 * ldt + 2.6097574011 * mdt - 0.3413193965 * sdt;
                let g2 = -1.2684380046 * ldt2 + 2.6097574011 * mdt2 - 0.3413193965 * sdt2;

                let u_g = g1 / (g1 * g1 - 0.5 * g * g2);
                let t_g = -g * u_g;

                let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s - 1.0;
                let b1 = -0.0041960863 * ldt - 0.7034186147 * mdt + 1.7076147010 * sdt;
                let b2 = -0.0041960863 * ldt2 - 0.7034186147 * mdt2 + 1.7076147010 * sdt2;

                let u_b = b1 / (b1 * b1 - 0.5 * b * b2);
                let t_b = -b * u_b;

                let t_r = if u_r >= 0.0 { t_r } else { core::f32::INFINITY };
                let t_g = if u_g >= 0.0 { t_g } else { core::f32::INFINITY };
                let t_b = if u_b >= 0.0 { t_b } else { core::f32::INFINITY };

                t + min(t_r, min(t_g, t_b))
            }
        }
    }
}

// Returns (S, T)
fn to_st(cusp_L: f32, cusp_C: f32) -> (f32, f32) {
    (cusp_C / cusp_L, cusp_C / (1.0 - cusp_L))
}

// Returns (S, T)
fn get_st_mid(a_: f32, b_: f32) -> (f32, f32) {
    let S = 0.11516993
        + 1.0
            / (7.44778970
                + 4.15901240 * b_
                + a_ * (-2.19557347
                    + 1.75198401 * b_
                    + a_ * (-2.13704948 - 10.02301043 * b_
                        + a_ * (-4.24894561 + 5.38770819 * b_ + 4.69891013 * a_))));

    let T = 0.11239642
        + 1.0
            / (1.61320320 - 0.68124379 * b_
                + a_ * (0.40370612
                    + 0.90148123 * b_
                    + a_ * (-0.27087943
                        + 0.61223990 * b_
                        + a_ * (0.00299215 - 0.45399568 * b_ - 0.14661872 * a_))));

    (S, T)
}

fn get_cs(L: f32, a_: f32, b_: f32) -> (f32, f32, f32) {
    let (cusp_L, cusp_C) = find_cusp(a_, b_);

    let C_max = find_gamut_intersection(a_, b_, L, 1.0, L, cusp_L, cusp_C);
    let (ST_max_S, ST_max_T) = to_st(cusp_L, cusp_C);

    let k = C_max / min(L * ST_max_S, (1.0 - L) * ST_max_T);

    let C_mid = {
        let (ST_mid_S, ST_mid_T) = get_st_mid(a_, b_);

        let C_a = L * ST_mid_S;
        let C_b = (1.0 - L) * ST_mid_T;
        0.9 * k
            * sqrt(sqrt(
                1.0 / (1.0 / (C_a * C_a * C_a * C_a) + 1.0 / (C_b * C_b * C_b * C_b)),
            ))
    };

    let C_0 = {
        let C_a = L * 0.4;
        let C_b = (1.0 - L) * 0.8;

        sqrt(1.0 / (1.0 / (C_a * C_a) + 1.0 / (C_b * C_b)))
    };

    (C_0, C_mid, C_max)
}

fn toe_inv(x: f32) -> f32 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    (x * x + k_1 * x) / (k_3 * (x + k_2))
}

pub fn okhsl_to_srgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if l >= 1.0 {
        return (1.0, 1.0, 1.0);
    }
    if l <= 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let a_ = cos(2.0 * core::f32::consts::PI * h);
    let b_ = sin(2.0 * core::f32::consts::PI * h);

    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1.0 + k_1) / (1.0 + k_2);
    let L = (l * l + k_1 * l) / (k_3 * (l + k_2));

    let (C_0, C_mid, C_max) = get_cs(L, a_, b_);

    let mid = 0.8;
    let mid_inv = 1.25;

    let C = {
        if s < mid {
            let t = mid_inv * s;

            let k_1 = mid * C_0;
            let k_2 = 1.0 - k_1 / C_mid;

            t * k_1 / (1.0 - k_2 * t)
        } else {
            let t = (s - mid) / (1.0 - mid);

            let k_0 = C_mid;
            let k_1 = (1.0 - mid) * C_mid * C_mid * mid_inv * mid_inv / C_0;
            let k_2 = 1.0 - (k_1) / (C_max - C_mid);

            k_0 + t * k_1 / (1.0 - k_2 * t)
        }
    };

    let (r, g, b) = oklab_to_linear_srgb(L, C * a_, C * b_);
    (
        srgb_transfer_function(r),
        srgb_transfer_function(g),
        srgb_transfer_function(b),
    )
}

pub fn okhsv_to_srgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let a_ = cos(2.0 * core::f32::consts::PI * h);
    let b_ = sin(2.0 * core::f32::consts::PI * h);

    let (cusp_L, cusp_C) = find_cusp(a_, b_);
    let (S_max, T_max) = to_st(cusp_L, cusp_C);
    let S_0 = 0.5;
    let k = 1.0 - S_0 / S_max;

    let L_v = 1.0 - s * S_0 / (S_0 + T_max - T_max * k * s);
    let C_v = s * T_max * S_0 / (S_0 + T_max - T_max * k * s);

    let L = v * L_v;
    let C = v * C_v;

    let L_vt = toe_inv(L_v);
    let C_vt = C_v * L_vt / L_v;

    let L_new = toe_inv(L);
    let C = C * L_new / L;
    let L = L_new;

    let (r_scale, g_scale, b_scale) = oklab_to_linear_srgb(L_vt, a_ * C_vt, b_ * C_vt);
    let scale_L = pow(
        1.0 / max(max(r_scale, g_scale), max(b_scale, 0.0)),
        1.0 / 3.0,
    );

    let L = L * scale_L;
    let C = C * scale_L;

    let (r, g, b) = oklab_to_linear_srgb(L, C * a_, C * b_);
    (
        srgb_transfer_function(r),
        srgb_transfer_function(g),
        srgb_transfer_function(b),
    )
}

#[derive(Clone)]
pub enum Space {
    HSL,
    HSV,
}

/// Palette implemented as a 3-D LUT.
#[derive(Clone)]
pub struct Palette {
    lut: Vec<Vec3>,
    h1: f32,
    s1: f32,
    l1: f32,
    h2: f32,
    s2: f32,
    l2: f32,
    h3: f32,
    s3: f32,
    l3: f32,
    texture: Box<dyn Texture>,
}

/// Convert from Cartesian coordinates in [-1, 1] for each component
/// to cylindrical coordinates (angle, r, z) in [0, 1] for each component.
pub fn cartesian_to_cylindrical(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let angle = libm::atan2f(y, x);
    let r = sqrt(squared(x) + squared(y)).min(1.0);
    (
        angle / core::f32::consts::TAU + 0.5,
        r,
        clamp01(z * 0.5 + 0.5),
    )
}

/// Convert from HSL coordinates in [0, 1] for each component
/// to Cartesian coordinates in [-1, 1] for each component.
pub fn hsl_to_xyz(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    (
        cos(h * core::f32::consts::TAU) * l,
        sin(h * core::f32::consts::TAU) * l,
        s * 2.0 - 1.0,
    )
}

/// Generate a palette. The palette works by interpolating between 3 anchor points
/// `(h1, s1, l1)`, `(h2, s2, l2)` `(h3, s3, l3)`
/// placed inside a HSL color cylinder. All parameters are in 0...1.
pub fn palette(
    h1: f32,
    s1: f32,
    l1: f32,
    h2: f32,
    s2: f32,
    l2: f32,
    h3: f32,
    s3: f32,
    l3: f32,
    texture: Box<dyn Texture>,
) -> Box<dyn Texture> {
    let mut lut = vec![vec3(0.0, 0.0, 0.0); 32 * 32 * 32];
    let (x1, y1, z1) = hsl_to_xyz(h1, s1, l1);
    let (x2, y2, z2) = hsl_to_xyz(h2, s2, l2);
    let (x3, y3, z3) = hsl_to_xyz(h3, s3, l3);

    for h in 0..32 {
        let w1 = (h as f32 + 0.5) / 31.5;
        for s in 0..32 {
            let w2 = (s as f32 + 0.5) / 31.5;
            for v in 0..32 {
                let w3 = (v as f32 + 0.5) / 31.5;
                let w = w1 + w2 + w3;
                let w1 = w1 / w;
                let w2 = w2 / w;
                let w3 = w3 / w;
                let x = x1 * w1 + x2 * w2 + x3 * w3;
                let y = y1 * w1 + y2 * w2 + y3 * w3;
                let z = z1 * w1 + z2 * w2 + z3 * w3;
                let (hf, vf, sf) = cartesian_to_cylindrical(x, y, z);
                let (r, g, b) = okhsl_to_srgb(hf, sf, vf);
                lut[Palette::index_at(h, s, v)] = vec3(r, g, b);
            }
        }
    }

    Box::new(Palette {
        lut,
        h1,
        s1,
        l1,
        h2,
        s2,
        l2,
        h3,
        s3,
        l3,
        texture,
    })
}

impl Palette {
    #[inline]
    fn index_at(h: usize, s: usize, v: usize) -> usize {
        (h << 10) + (s << 5) + v
    }
}

impl Texture for Palette {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture.at_frequency(point, frequency);
        let x = clamp01(u.x * 0.5 + 0.5) * 30.9999;
        let y = clamp01(u.y * 0.5 + 0.5) * 30.9999;
        let z = clamp01(u.z * 0.5 + 0.5) * 30.9999;
        let xi = unsafe { f32::to_int_unchecked::<usize>(x) };
        let yi = unsafe { f32::to_int_unchecked::<usize>(y) };
        let zi = unsafe { f32::to_int_unchecked::<usize>(z) };
        let xf = x - xi as f32;
        let yf = y - yi as f32;
        let zf = z - zi as f32;
        let i000 = self.lut[Palette::index_at(xi, yi, zi)];
        let i001 = self.lut[Palette::index_at(xi, yi, zi + 1)];
        let i010 = self.lut[Palette::index_at(xi, yi + 1, zi)];
        let i011 = self.lut[Palette::index_at(xi, yi + 1, zi + 1)];
        let i100 = self.lut[Palette::index_at(xi + 1, yi, zi)];
        let i101 = self.lut[Palette::index_at(xi + 1, yi, zi + 1)];
        let i110 = self.lut[Palette::index_at(xi + 1, yi + 1, zi)];
        let i111 = self.lut[Palette::index_at(xi + 1, yi + 1, zi + 1)];
        let i00 = lerp(i000, i001, zf);
        let i01 = lerp(i010, i011, zf);
        let i10 = lerp(i100, i101, zf);
        let i11 = lerp(i110, i111, zf);
        let i0 = lerp(i00, i01, yf);
        let i1 = lerp(i10, i11, yf);
        let i = lerp(i0, i1, xf);
        // Rescale to -1...1.
        vec3a(i.x * 2.0 - 1.0, i.y * 2.0 - 1.0, i.z * 2.0 - 1.0)
    }

    fn get_code(&self) -> String {
        format!(
            "palette({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {})",
            self.h1,
            self.s1,
            self.l1,
            self.h2,
            self.s2,
            self.l2,
            self.h3,
            self.s3,
            self.l3,
            self.texture.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "palette({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {})",
            self.h1,
            self.s1,
            self.l1,
            self.h2,
            self.s2,
            self.l2,
            self.h3,
            self.s3,
            self.l3,
            self.texture.get_basis_code()
        )
    }
}
