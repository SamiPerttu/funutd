use super::map3base::*;
use super::math::*;
use super::*;

/// Saturates components.
pub struct Saturate {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

/// Saturates components.
impl Texture for Saturate {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        softsign(self.texture.at(point, frequency) * self.amount)
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
pub struct Reflect {
    amount: f32,
    offset: Vec3a,
    texture: Box<dyn Texture>,
}

impl Texture for Reflect {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        wave(
            smooth3,
            self.offset + self.texture.at(point, frequency) * self.amount,
        )
    }
    fn get_code(&self) -> String {
        format!(
            "reflect({}, Vec3a({}, {}, {}), {})",
            self.amount,
            self.offset.x,
            self.offset.y,
            self.offset.z,
            self.texture.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "reflect({}, Vec3a({}, {}, {}), {})",
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
pub fn reflect(amount: f32, offset: Vec3a, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    Box::new(Reflect {
        amount,
        offset,
        texture,
    })
}

/// Posterize: applies a smooth step function in proportion to texture value magnitude.
pub struct Posterize {
    levels: f32,
    sharpness: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Posterize {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at(point, frequency);
        let magnitude = self.levels * v.length();
        if magnitude > 0.0 {
            let base = magnitude.floor();
            let t = magnitude - base;
            let power: f32 = 1.0 + 50.0 * squared(squared(self.sharpness));
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

/// Applies a wavy function to texture values with an offset, which can
/// spread and reflect components. Amount is the scale (amount > 0),
/// roughly corresponding to number of reflections.
pub fn posterize(levels: f32, sharpness: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    Box::new(Posterize {
        levels,
        sharpness,
        texture,
    })
}

/// Saturates components while retaining component proportions.
pub struct Overdrive {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Overdrive {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at(point, frequency);
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

/// Saturates components while retaining component proportions.
pub struct VReflect {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

impl Texture for VReflect {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let v = self.texture.at(point, frequency);
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

/// Saturates components (amount > 0).
/// Amount equals derivative at origin. Amounts greater than 1 result in overdrive.
pub fn vreflect(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(VReflect { amount, texture })
}

/// Rotates values of one texture with values from another texture.
pub struct Rotate {
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Rotate {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at(point, frequency);
        let v = self.texture_b.at(point, frequency);
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
            "rotate({}, {}, {})",
            self.amount,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "rotate({}, {}, {})",
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
pub struct Softmix3 {
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Softmix3 {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at(point, frequency);
        let v = self.texture_b.at(point, frequency);
        let vw: f32 = softexp(v * self.amount).length_squared();
        let uw: f32 = softexp(u * self.amount).length_squared();
        let epsilon: f32 = 1.0e-9;
        (v * vw + u * uw) / (vw + uw + epsilon)
    }
    fn get_code(&self) -> String {
        format!(
            "softmix3({}, {}, {})",
            self.amount,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "softmix3({}, {}, {})",
            self.amount,
            self.texture_a.get_basis_code(),
            self.texture_b.get_basis_code()
        )
    }
}

pub fn softmix3(
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
) -> Box<dyn Texture> {
    assert!(amount > 0.0);
    Box::new(Softmix3 {
        amount,
        texture_a,
        texture_b,
    })
}

/*
        let v0 = self.texture.at(point);
        let mut v = Vec3a(v0.x, v0.y, v0.z);
        let mut v1: Vec3a = v;
        let mut v2: Vec3a = v;
        let mut v3: Vec3a = v;
        for _i in 0 .. 12 {
            let u = Vec3a(v.x * v.x - v.y * v.y - v.z * v.z + v0.x, 2.0 * v.x * v.y + v0.y, 2.0 * (v.x - v.y) * v.z + v0.z);
            //u = Vec3a(cubed(v.x) - 3.0 * v.x * (squared(v.y) + squared(v.z)) + v0.x, -cubed(v.y) + 3.0 * v.y * squared(v.x) - v.y * squared(v.z) + v0.y, cubed(v.z) - 3.0 * v.z * squared(v.x) + v.z * squared(v.y) + v0.z);
            //u = Vec3a(v.x * v.x - v.y * v.y + v0.x, 2.0 * v.x * v.y + v0.y, 0.0);
            if u.length_squared() > 4.0 {
                let w = (u.length_squared() - 4.0).tanh();
                //return v0; // Vec3a(-1.0, -1.0, -1.0);
                return v3 * (1.0 - w) + v2 * w;
                //return v * (1.0 - w) * 0.5 + u * w * 0.5;
            }
            v3 = v2;
            v2 = v1;
            v1 = v;
            v = u;
        }
        //u * 0.5
        v3

*/

/// Displaces lookup of one texture by values from another texture.
pub struct Displace {
    amount: f32,
    texture_a: Box<dyn Texture>,
    texture_b: Box<dyn Texture>,
}

impl Texture for Displace {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let u = self.texture_a.at(point, frequency);
        self.texture_b.at(point + u * self.amount, frequency)
    }
    fn get_code(&self) -> String {
        format!(
            "displace({}, {}, {})",
            self.amount,
            self.texture_a.get_code(),
            self.texture_b.get_code()
        )
    }
    fn get_basis_code(&self) -> String {
        format!(
            "displace({}, {}, {})",
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
    assert!(amount > 0.0);
    Box::new(Displace {
        amount,
        texture_a,
        texture_b,
    })
}

pub struct Fractal {
    base_f: f32,
    octaves: usize,
    roughness: f32,
    displace: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Fractal {
    fn at(&self, point: Vec3a, _frequency: Option<f32>) -> Vec3a {
        let mut f = self.base_f;
        let mut result = Vec3a::zero();
        let mut p = point;
        let mut w = 1.0;
        let mut total_w = 0.0;
        for _ in 0..self.octaves {
            let v = self.texture.at(p, Some(f));
            result += v * w;
            total_w += w;
            w *= self.roughness;
            p += v * self.displace / f;
            f *= 2.0;
        }
        result / total_w
    }
    fn get_code(&self) -> String {
        format!(
            "fractal({}, {}, {}, {3:.7}, {4:})",
            self.base_f,
            self.octaves,
            self.roughness,
            self.displace,
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
    roughness: f32,
    displace: f32,
    texture: Box<dyn Texture>,
) -> Box<dyn Texture> {
    Box::new(Fractal {
        texture,
        base_f,
        octaves,
        roughness,
        displace,
    })
}
