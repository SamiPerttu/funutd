//! Isotropic value and gradient noises.

use super::hash::*;
use super::map3base::*;
use super::math::*;
use super::*;

/// Roughly isotropic value noise.
pub struct VNoise<H: Hasher> {
    seed: u64,
    frequency: f32,
    hasher: H,
}

pub fn vnoise<H: 'static + Hasher>(seed: u64, frequency: f32, hasher: H) -> Box<dyn Texture> {
    Box::new(VNoise {
        seed,
        frequency,
        hasher,
    })
}

pub fn vnoise_basis<H: 'static + Hasher>(seed: u64, hasher: H) -> Box<dyn Texture> {
    Box::new(VNoise {
        seed,
        frequency: 1.0,
        hasher,
    })
}

impl<H: Hasher> Texture for VNoise<H> {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let frequency = frequency.unwrap_or(self.frequency);
        let basis = self.hasher.query(self.seed, frequency, point);
        let mut result = Vec3a::zero();

        for dx in -1..=1 {
            let hx = self.hasher.hash_x(&basis, 0, dx);
            for dy in -1..=1 {
                let hxy = self.hasher.hash_y(&basis, hx, dy);
                let mut offset = Vec3a::new(dx as f32, dy as f32, 0.0) - basis.d;
                for dz in -1..=1 {
                    let mut hash = self.hasher.hash_z(&basis, hxy, dz);
                    // Pick number of cells as a rough approximation to a Poisson distribution.
                    let n = match hash & 7 {
                        0 | 1 | 2 | 3 => 1,
                        5 | 6 => 2,
                        _ => 3,
                    };
                    // Offset points from cell corner to queried point.
                    offset = vec3a(offset.x, offset.y, dz as f32 - basis.d.z);
                    for i in 0..n {
                        // Feature location.
                        let p = hash_01(hash);
                        let distance2: f32 = (p + offset).length_squared();
                        // Feature radius is always 1 here, which is the maximum.
                        let radius: f32 = 1.0;
                        if distance2 < radius * radius {
                            let distance = sqrt(distance2) / radius;
                            let color = hash_11(hash);
                            let blend = 1.0 - smooth5(distance);
                            result += color * blend;
                        }
                        if i + 1 < n {
                            hash = hash64c(hash);
                        }
                    }
                }
            }
        }
        result
    }

    fn get_code(&self) -> String {
        format!(
            "vnoise({}, {}, {})",
            self.seed,
            self.frequency,
            self.hasher.get_code()
        )
    }

    fn get_basis_code(&self) -> String {
        format!("vnoise_basis({}, {})", self.seed, self.hasher.get_code())
    }
}

/// Roughly isotropic gradient noise.
pub struct Noise<H: Hasher> {
    seed: u64,
    frequency: f32,
    hasher: H,
}

pub fn noise<H: 'static + Hasher>(seed: u64, frequency: f32, hasher: H) -> Box<dyn Texture> {
    Box::new(Noise {
        seed,
        frequency,
        hasher,
    })
}

pub fn noise_basis<H: 'static + Hasher>(seed: u64, hasher: H) -> Box<dyn Texture> {
    Box::new(Noise {
        seed,
        frequency: 1.0,
        hasher,
    })
}

impl<H: Hasher> Texture for Noise<H> {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let frequency = frequency.unwrap_or(self.frequency);
        let basis = self.hasher.query(self.seed, frequency, point);
        let mut result = Vec3a::zero();

        for dx in -1..=1 {
            let hx = self.hasher.hash_x(&basis, 0, dx);
            for dy in -1..=1 {
                let hxy = self.hasher.hash_y(&basis, hx, dy);
                let mut offset = Vec3a::new(dx as f32, dy as f32, 0.0) - basis.d;
                for dz in -1..=1 {
                    let mut hash = self.hasher.hash_z(&basis, hxy, dz);
                    // Pick number of cells as a rough approximation to a Poisson distribution.
                    let n = match hash & 7 {
                        0 | 1 | 2 | 3 => 1,
                        5 | 6 => 2,
                        _ => 3,
                    };
                    // Offset points from cell corner to queried point.
                    offset = vec3a(offset.x, offset.y, dz as f32 - basis.d.z);
                    for i in 0..n {
                        // Feature location.
                        let p = hash_01(hash);
                        let distance2: f32 = (p + offset).length_squared();
                        // Feature radius is always 1 here, which is the maximum.
                        let radius: f32 = 1.0;
                        if distance2 < radius * radius {
                            let distance = sqrt(distance2) / radius;
                            let color = hash_11(hash);
                            let gradient = hash_unit(hash64b(hash));
                            let blend = 1.0 - smooth5(distance);
                            result += color * blend * gradient.dot(p + offset);
                        }
                        if i + 1 < n {
                            hash = hash64c(hash);
                        }
                    }
                }
            }
        }
        result * 2.0
    }

    fn get_code(&self) -> String {
        format!(
            "noise({}, {}, {})",
            self.seed,
            self.frequency,
            self.hasher.get_code()
        )
    }

    fn get_basis_code(&self) -> String {
        format!("noise_basis({}, {})", self.seed, self.hasher.get_code())
    }
}
