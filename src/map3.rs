//! Texture unary and binary operators.

use super::ease::*;
use super::hash::*;
use super::map3base::*;
use super::math::*;
use super::*;
extern crate alloc;
use alloc::{boxed::Box, string::String, string::ToString};

/// Zero texture.
#[derive(Clone)]
pub struct Zero {}

impl Texture for Zero {
    fn at_frequency(&self, _point: Vec3a, _frequency: Option<f32>) -> Vec3a {
        Vec3a::zero()
    }
    fn get_code(&self) -> String {
        "zero()".to_string()
    }
    fn get_basis_code(&self) -> String {
        "zero()".to_string()
    }
}

/// Zero texture.
pub fn zero() -> Zero {
    Zero {}
}

/// Saturates components.
#[derive(Clone)]
pub struct Saturate {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Saturate {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        softsign(self.texture.at_frequency(point, frequency) * self.amount)
    }
    fn get_code(&self) -> String {
        format!("saturate({}, {})", self.amount, self.texture.get_code())
    }
    fn get_basis_code(&self) -> String {
        format!(
            "saturate({}, {})",
            self.amount,
            self.texture.get_basis_code()
        )
    }
}

/// Saturates components (amount > 0).
/// Amount equals derivative at origin. Amounts greater than 1 result in overdrive.
pub fn saturate(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(Saturate { amount, texture })
}

/// Reflect: applies a wavy function to texture values with an offset, which can
/// spread and reflect components.
#[derive(Clone)]
pub struct Reflect {
    amount: f32,
    offset: Vec3a,
    texture: Box<dyn Texture>,
}

impl Texture for Reflect {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        wave(
            smooth3,
            self.offset + self.texture.at_frequency(point, frequency) * self.amount,
        )
    }
    fn get_code(&self) -> String {
        format!(
            "reflect({}, vec3({:?}, {:?}, {:?}), {})",
            self.amount,
            self.offset.x,
            self.offset.y,
            self.offset.z,
            self.texture.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "reflect({}, vec3({:?}, {:?}, {:?}), {})",
            self.amount,
            self.offset.x,
            self.offset.y,
            self.offset.z,
            self.texture.get_basis_code()
        )
    }
}

/// Applies a wavy function to texture values with an offset, which can
/// spread and reflect components. Amount is the scale (amount > 0),
/// roughly corresponding to number of reflections.
pub fn reflect(amount: f32, offset: Vec3, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    Box::new(Reflect {
        amount,
        offset: offset.into(),
        texture,
    })
}

/// Posterize: applies a smooth step function in proportion to texture value magnitude.
#[derive(Clone)]
pub struct Posterize {
    levels: f32,
    sharpness: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Posterize {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at_frequency(point, frequency);
        let magnitude = self.levels * v.length();
        if magnitude > 0.0 {
            let base = magnitude.floor();
            let t = magnitude - base;
            let power: f32 = 1.0 + 50.0 * squared(self.sharpness);
            let p = if t < 0.5 {
                0.5 * pow(2.0 * t, power)
            } else {
                1.0 - 0.5 * pow(2.0 * (1.0 - t), power)
            };
            v * ((base + p) / magnitude)
        } else {
            Vec3a::zero()
        }
    }
    fn get_code(&self) -> String {
        format!(
            "posterize({}, {}, {})",
            self.levels,
            self.sharpness,
            self.texture.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "posterize({}, {}, {})",
            self.levels,
            self.sharpness,
            self.texture.get_basis_code()
        )
    }
}

/// Posterize: applies a smooth step function in proportion to texture value magnitude.
pub fn posterize(levels: f32, sharpness: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    Box::new(Posterize {
        levels,
        sharpness,
        texture,
    })
}

/// Saturates components while retaining component proportions.
#[derive(Clone)]
pub struct Overdrive {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Overdrive {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at_frequency(point, frequency);
        // Use the 4-norm as a smooth proxy for the largest magnitude component.
        let magnitude = squared(v).length_squared();
        if magnitude > 0.0 {
            let m = sqrt(sqrt(magnitude));
            v / m * softsign(m * self.amount)
        } else {
            Vec3a::zero()
        }
    }
    fn get_code(&self) -> String {
        format!("overdrive({}, {})", self.amount, self.texture.get_code())
    }
    fn get_basis_code(&self) -> String {
        format!(
            "overdrive({}, {})",
            self.amount,
            self.texture.get_basis_code()
        )
    }
}

/// Saturates components (amount > 0).
/// Amount equals derivative at origin. Amounts greater than 1 result in overdrive.
pub fn overdrive(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(Overdrive { amount, texture })
}

/// Applies a wavy function in proportion to vector magnitude.
#[derive(Clone)]
pub struct VReflect {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

impl Texture for VReflect {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at_frequency(point, frequency);
        let m = v.length();
        if m > 0.0 {
            v * (sin(m * self.amount * f32::PI * 0.5) / m)
        } else {
            Vec3a::zero()
        }
    }
    fn get_code(&self) -> String {
        format!("vreflect({}, {})", self.amount, self.texture.get_code())
    }
    fn get_basis_code(&self) -> String {
        format!(
            "vreflect({}, {})",
            self.amount,
            self.texture.get_basis_code()
        )
    }
}

/// Applies a wavy function in proportion to vector magnitude.
pub fn vreflect(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(VReflect { amount, texture })
}

/// Rotates values of one texture with values from another texture.
#[derive(Clone)]
pub struct Rotate {
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Rotate {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at_frequency(point, frequency);
        let v = self.texture_b.at_frequency(point, frequency);
        let length: f32 = u.length();
        if length > 1.0e-9 {
            let axis = u / length;
            Quat::from_axis_angle(Vec3::from(axis), self.amount * length) * v
        } else {
            Vec3a::zero()
        }
    }
    fn get_code(&self) -> String {
        format!(
            "rotate({:?}, {}, {})",
            self.amount,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "rotate({:?}, {}, {})",
            self.amount,
            self.texture_a.get_basis_code(),
            self.texture_b.get_basis_code()
        )
    }
}

/// Amount is the maximum rotation in radians.
pub fn rotate(
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(Rotate {
        amount,
        texture_a,
        texture_b,
    })
}

/// Mixes between two textures weighted with their vector magnitudes.
#[derive(Clone)]
pub struct Softmix3 {
    amount: f32,
    displacement: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Softmix3 {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at_frequency(point, frequency);
        let v = self.texture_b.at_frequency(
            point + u * self.displacement / frequency.unwrap_or(2.0),
            frequency,
        );
        let vw: f32 = softexp(v * self.amount).length();
        let uw: f32 = softexp(u * self.amount).length();
        let epsilon: f32 = 1.0e-9;
        (v * vw + u * uw) / (vw + uw + epsilon)
    }
    fn get_code(&self) -> String {
        format!(
            "softmix3({:?}, {:?}, {}, {})",
            self.amount,
            self.displacement,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "softmix3({:?}, {:?}, {}, {})",
            self.amount,
            self.displacement,
            self.texture_a.get_basis_code(),
            self.texture_b.get_basis_code()
        )
    }
}

pub fn softmix3(
    amount: f32,
    displacement: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(Softmix3 {
        amount,
        displacement,
        texture_a,
        texture_b,
    })
}

/// Layers one texture on another with weight depending on distance between texture values.
#[derive(Clone)]
pub struct Layer {
    width: f32,
    ease: Ease,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Layer {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at_frequency(point, frequency);
        let v = self.texture_b.at_frequency(point, frequency);
        let d = u - v;
        let distance = d.length();
        if distance < self.width {
            u + v * self.ease.at(1.0 - distance / self.width)
        } else {
            u
        }
    }
    fn get_code(&self) -> String {
        format!(
            "layer({:?}, Ease::{:?}, {}, {})",
            self.width,
            self.ease,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "layer({:?}, Ease::{:?}, {}, {})",
            self.width,
            self.ease,
            self.texture_a.get_basis_code(),
            self.texture_b.get_basis_code()
        )
    }
}

pub fn layer(
    width: f32,
    ease: Ease,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
) -> Box<dyn Texture> {
    assert!(width > 0.0);
    Box::new(Layer {
        width,
        ease,
        texture_a,
        texture_b,
    })
}

/// Displaces lookup of one texture by values from another texture.
#[derive(Clone)]
pub struct Displace {
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Displace {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at_frequency(point, frequency);
        self.texture_b.at_frequency(
            point + u * self.amount / frequency.unwrap_or(2.0),
            frequency,
        )
    }
    fn get_code(&self) -> String {
        format!(
            "displace({:?}, {}, {})",
            self.amount,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "displace({:?}, {}, {})",
            self.amount,
            self.texture_a.get_basis_code(),
            self.texture_b.get_basis_code()
        )
    }
}

pub fn displace(
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
) -> Box<dyn Texture> {
    Box::new(Displace {
        amount,
        texture_a,
        texture_b,
    })
}

#[derive(Clone)]
pub struct Fractal {
    base_f: f32,
    octaves: usize,
    first_octave: usize,
    roughness: f32,
    lacunarity: f32,
    displace: f32,
    layer: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Fractal {
    fn at_frequency(&self, point: Vec3a, _frequency: Option<f32>) -> Vec3a {
        let mut result = Vec3a::zero();
        let mut p = point;
        let mut total_w = 0.0;
        let mut octave = self.first_octave;
        for _ in 0..self.octaves {
            let f = self.base_f * pow(self.lacunarity, octave as f32);
            let w = pow(self.roughness, octave as f32);

            let v = self.texture.at_frequency(p, Some(f));

            let weight = if octave <= self.first_octave || self.layer == 0.0 {
                1.0
            } else {
                let layer_diff = result / total_w - v;
                let layer_distance = layer_diff.length();
                if layer_distance < self.layer {
                    smooth3(1.0 - layer_distance / self.layer)
                } else {
                    0.0
                }
            };
            result += v * w * weight;
            total_w += w * weight;

            if octave == 0 {
                octave = self.first_octave + 1;
                p += v * self.displace * weight / f;
            } else if octave <= self.first_octave {
                octave -= 1;
                p += v * self.displace * weight / f * self.lacunarity;
            } else {
                octave += 1;
                p += v * self.displace * weight / f / self.lacunarity;
            }
        }
        result / sqrt(total_w)
    }
    fn get_code(&self) -> String {
        format!(
            "fractal({:?}, {}, {}, {:?}, {:?}, {:?}, {:?}, {})",
            self.base_f,
            self.octaves,
            self.first_octave,
            self.roughness,
            self.lacunarity,
            self.displace,
            self.layer,
            self.texture.get_basis_code()
        )
    }
    fn get_basis_code(&self) -> String {
        self.get_code()
    }
}

pub fn fractal(
    base_f: f32,
    octaves: usize,
    first_octave: usize,
    roughness: f32,
    lacunarity: f32,
    displace: f32,
    layer: f32,
    texture: Box<dyn Texture>,
) -> Box<dyn Texture> {
    Box::new(Fractal {
        texture,
        base_f,
        octaves,
        first_octave,
        roughness,
        lacunarity,
        displace,
        layer,
    })
}

/// Shifts components around.
/// Intended to induce more dependencies between components.
#[derive(Clone)]
pub struct Shift {
    seed: u32,
    rotation: Quat,
    origin: Vec3a,
    texture: Box<dyn Texture>,
}

impl Texture for Shift {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at_frequency(point, frequency);
        let p = (self.rotation * (v - self.origin)) + self.origin;
        vec3a(sin(p.x), sin(p.y), sin(p.z))
    }
    fn get_code(&self) -> String {
        format!("shift({}, {})", self.seed, self.texture.get_code())
    }
    fn get_basis_code(&self) -> String {
        format!("shift({}, {})", self.seed, self.texture.get_basis_code())
    }
}

/// Shifts components around.
pub fn shift(seed: u32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    let axis = hash_unit(seed as u64);
    let angle = hash_01(hash64c(seed as u64));
    let rotation = Quat::from_axis_angle(
        axis.into(),
        lerp(f32::TAU * 1.0 / 8.0, f32::TAU * 7.0 / 8.0, angle.x),
    );
    let origin = hash_11(hash64d(seed as u64));
    Box::new(Shift {
        seed,
        rotation,
        origin,
        texture,
    })
}
