use glam::*;

// 3-D procedural texture library.

/// Smooth quintic easing function suggested by Ken Perlin.
pub fn smooth5(x: f32) -> f32 {
    ((x * 6.0 - 15.0) * x + 10.0) * x * x * x
}

/// Smooth cubic easing function.
pub fn smooth3(x: f32) -> f32 {
    (3.0 - 2.0 * x) * x * x
}

/// Exp-like function that uses basic arithmetic only.
pub fn exq(x: f32) -> f32 {
    let p = x.max(0.0);
    p * p + p + 1.0 / (1.0 + p - x)
}

/// Softsign function.
pub fn softsign(x: f32) -> f32 {
    x / (1.0 + x.abs())
}

/// 64-bit hash from SplitMix64.
pub fn hashc(x: u64) -> u64 {
    let x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

/// 64-bit hash by degski.
pub fn hashd(x: u64) -> u64 {
    let x = (x ^ (x >> 32)).wrapping_mul(0xd6e8feb86659fd93);
    let x = (x ^ (x >> 32)).wrapping_mul(0xd6e8feb86659fd93);
    x ^ (x >> 32)
}

/// 64-bit hash from MurmurHash3 by Austin Appleby.
pub fn hashq(x: u64) -> u64 {
    let x = (x ^ (x >> 33)).wrapping_mul(0xff51afd7ed558ccd);
    let x = (x ^ (x >> 33)).wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^ (x >> 33)
}

pub fn frc(x: u64) -> f32 {
    (x & 0xff) as f32
}
pub fn grc(x: u64) -> f32 {
    (x & 0xff) as f32
}
pub fn src(x: u64) -> f32 {
    (x & 0xff) as f32
}

pub struct Basis3 {
    // Cell.
    pub ix: u32,
    pub iy: u32,
    pub iz: u32,
    // Position inside cell in 0...1.
    pub d: Vec3A,
}
impl Basis3 {
    pub fn new(p: Vec3A) -> Basis3 {
        let i = p.floor();
        let a: [f32; 3] = i.into();
        Basis3 {
            ix: (a[0] as i32) as u32,
            iy: (a[1] as i32) as u32,
            iz: (a[2] as i32) as u32,
            d: p - i,
        }
    }
    pub fn hash_x(&self, current: u64, dx: i32) -> u64 {
        hashc(current ^ (self.ix.wrapping_add(dx as u32)) as u64)
    }
    pub fn hash_y(&self, current: u64, dy: i32) -> u64 {
        hashd(current ^ (self.iy.wrapping_add(dy as u32)) as u64)
    }
    pub fn hash_xy(&self, current: u64, dx: i32, dy: i32) -> u64 {
        hashd(
            current
                ^ ((self.ix.wrapping_add(dx as u32)) as u64)
                ^ (((self.iy.wrapping_add(dy as u32)) as u64) << 32),
        )
    }
    pub fn hash_z(&self, current: u64, dz: i32) -> u64 {
        hashc(current ^ (self.iz.wrapping_add(dz as u32)) as u64)
    }

    pub fn point(&self, h: u64) -> Vec3A {
        vec3a(frc(h) as f32, frc(h >> 8) as f32, frc(h >> 16) as f32) * (1.0 / 256.0)
    }
    pub fn gradient(&self, h: u64) -> Vec3A {
        vec3a(grc(h) as f32, grc(h >> 8) as f32, grc(h >> 16) as f32) * (2.0 / 255.0) - Vec3A::ONE
    }
    pub fn color(&self, h: u64) -> Vec3A {
        vec3a(src(h) as f32, src(h >> 8) as f32, src(h >> 16) as f32) * (2.0 / 255.0) - Vec3A::ONE
    }
}

/// Noise with a frequency of 1 unit.
pub fn noise3(v: Vec3A) -> Vec3A {
    let basis = Basis3::new(v);
    let mut result = Vec3A::ZERO;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let hxy = basis.hash_xy(0, dx, dy);
            let mut offset = Vec3A::new(dx as f32, dy as f32, 0.0) - basis.d;
            for dz in -1..=1 {
                let mut hash = basis.hash_z(hxy, dz);
                // Pick number of cells as a rough approximation to a Poisson distribution.
                let n = match hash & 7 {
                    0 | 1 | 2 | 3 => 1,
                    5 | 6 => 2,
                    _ => 3,
                };
                offset = Vec3A::new(offset.x, offset.y, dz as f32 - basis.d.z);
                for di in 0..n {
                    let p = basis.point(hash >> 8);
                    let distance2: f32 = (p + offset).length_squared();
                    let m: f32 = 1.0 - (((hash >> 3) & 31) as f32 / 31.0) * (15.0 / 31.0);
                    if distance2 < m * m {
                        let distance = distance2.sqrt() / m;
                        let color = basis.color(hash >> 32);
                        let blend = 1.0 - smooth5(distance);
                        result += color * blend;
                    }
                    if di + 1 < n {
                        hash = hashq(hash);
                    }
                }
            }
        }
    }
    result
}

/// Rotates v with u.
pub fn rotate(amount: f32, v: Vec3A, u: Vec3A) -> Vec3A {
    let length: f32 = u.length();
    if length > 1.0e-9 {
        let axis = u / length;
        Quat::from_axis_angle(Vec3::from(axis), amount * length) * v
    } else {
        Vec3A::ZERO
    }
}

pub fn softmix3(amount: f32, v: Vec3A, u: Vec3A) -> Vec3A {
    let vw: f32 = (v * amount).length_squared();
    let uw: f32 = (u * amount).length_squared();
    let epsilon: f32 = 1.0e-10;
    (v * vw + u * uw) / (vw + uw + epsilon)
}

pub fn reflect3(amount: f32, v: Vec3A) -> Vec3A {
    let m = v.length();
    if m > 0.0 {
        v * ((m * amount * std::f32::consts::PI * 0.5).sin() / m)
    } else {
        Vec3A::ZERO
    }
}

/// Saturates components (amount > 0).
pub fn overdrive(amount: f32, v: Vec3A) -> Vec3A {
    Vec3A::new(
        softsign(v.x * amount),
        softsign(v.y * amount),
        softsign(v.z * amount),
    )
}

/// Saturates the input while retaining component proportions (amount > 0).
pub fn overdrive3(amount: f32, v: Vec3A) -> Vec3A {
    // Use the 4-norm as a smooth proxy for the largest magnitude component.
    let v2 = v * v;
    let v4 = v2 * v2;
    let m = v4.length();

    if m > 0.0 {
        let m = m.sqrt().sqrt();
        v / m * softsign(m * amount)
    } else {
        Vec3A::ZERO
    }
}

pub fn posterize3(levels: f32, sharpness: f32, v: Vec3A) -> Vec3A {
    let magnitude = levels * v.length();
    if magnitude > 0.0 {
        let b = magnitude.floor();
        let t = magnitude - b;
        let sharpness2 = sharpness * sharpness;
        let power: f32 = 1.0 + 1000.0 * sharpness2 * sharpness2;
        let p = if t < 0.5 {
            0.5 * (2.0 * t).powf(power)
        } else {
            1.0 - 0.5 * (2.0 * (1.0 - t)).powf(power)
        };
        v * ((b + p) / magnitude)
    } else {
        Vec3A::ZERO
    }
}
