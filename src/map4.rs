use crate::prelude::*;
use glam::*;
use wrapping_arithmetic::wrappit;

#[inline] pub fn frc(x: u64) -> f32 { (x & 0xff) as f32 }
#[inline] pub fn src(x: u64) -> f32 { (x & 0xff) as f32 }

pub struct Basis3 {
    pub ix: u32,
    pub iy: u32,
    pub iz: u32,
    pub d: Vec3A,
}
impl Basis3 {
    #[inline] pub fn new(p: Vec3A) -> Basis3 {
        let i = p.floor();
        let a: [f32; 3] = i.into();
        Basis3 { ix: (a[0] as i32) as u32, iy: (a[1] as i32) as u32, iz: (a[2] as i32) as u32, d: p - i }
    }
    #[wrappit] pub fn hash_x(&self, current: u64, dx: i32) -> u64 { hashc(current ^ (self.ix + dx as u32) as u64) }
    #[wrappit] pub fn hash_y(&self, current: u64, dy: i32) -> u64 { hashd(current ^ (self.iy + dy as u32) as u64) }
    #[wrappit] pub fn hash_xy(&self, current: u64, dx: i32, dy: i32) -> u64 { hashd(current ^ ((self.ix + dx as u32) as u64) ^ (((self.iy + dy as u32) as u64) << 32)) }
    #[wrappit] pub fn hash_z(&self, current: u64, dz: i32) -> u64 { hashc(current ^ (self.iz + dz as u32) as u64) }
    #[inline] pub fn point(&self, h: u64, offset: Vec3A) -> Vec3A {
        offset + vec3a(frc(h) as f32, frc(h >> 8) as f32, frc(h >> 16) as f32) * (1.0 / 256.0)
    }
    #[inline] pub fn color(&self, h: u64) -> Vec4 {
        vec4(src(h) as f32, src(h >> 8) as f32, src(h >> 16) as f32, src(h >> 24) as f32) * (2.0 / 255.0) - Vec4::splat(1.0)
    }
}

pub fn noise3(v: Vec4) -> Vec4 {
    let basis = Basis3::new(v.truncate());
    let mut result = Vec4::zero();
    for dx in -1 ..= 1 {
        for dy in -1 ..= 1 {
            let hxy = basis.hash_xy(0, dx, dy);
            for dz in -1 ..= 1 {
                let mut h = basis.hash_z(hxy, dz);
                // Pick number of cells as a rough approximation to a Poisson distribution.
                //let n = match h & 7 { 0 | 1 | 2 | 3 | 4 => 1, 5 | 6 | 7 | _ => 2 };
                //let n = match h & 7 { 0 | 1 | 2 | 3 | 4 => 2, 5 | 6 | 7 | _ => 3 };
                let n = 1 + (h & 1);
                //let n = match h & 7 { 0 | 1 | 2 | 3 => 1, 5 | 6 => 2, 7 | _ => 3 };
                //let n = match h & 7 { 0 | 1 | 2 => 1, 3 | 4 | 5 => 2, 6 | 7 | _ => 3 };
                let offset = Vec3A::new(dx as f32, dy as f32, dz as f32);
                for di in 0 .. n {
                    let p = basis.point(h >> 8, offset);
                    let D: f32 = (basis.d - p).length();
                    //let M: f32 = 0.8 + ((h >> 3) & 31) as f32 * (0.2 / 31.0);
                    let M: f32 = 1.0 - squared(((h >> 3) & 31) as f32 / 31.0) * (15.0 / 31.0);
                    if D < M {
                        let D = D / M;
                        let C = basis.color(h >> 32);
                        //let blend = cos(D * PI as f32) * 0.5 + 0.5;
                        let blend = 1.0 - smooth3(D);
                        //let blend = 1.0 - smooth5(D);
                        result = result + C * blend;
                    }
                    if di + 1 < n { h = hashk(h); }
                }
            }
        }
    }
    result
}

/// Rotates v with u.
pub fn rotate(amount: f32, v: Vec4, u: Vec4) -> Vec4 {
    let w = v.truncate();
    let t = u.truncate();
    let length: f32 = t.length();
    if length > 1.0e-9 {
        let axis = t / length;
        let v3 = Quat::from_axis_angle(axis.into(), amount * length) * w;
        v3.extend(v.w())
    } else {
        Vec4::zero()
    }
}

pub fn softmix4(amount: f32, v: Vec4, u: Vec4) -> Vec4 {
    let vw: f32 = exq(v * amount).length_squared();
    let uw: f32 = exq(u * amount).length_squared();
    let epsilon: f32 = 1.0e-10;
    (v * vw + u * uw) / (vw + uw + epsilon)
}

pub fn reflect(amount: f32, v: Vec4) -> Vec4 {
    polywave(v * amount)
}

pub fn reflect4(amount: f32, v: Vec4) -> Vec4 {
    let m = v.length();
    if m > 0.0 {
        v * (sin(m * amount * (PI as f32) * 0.5) / m)
    } else {
        Vec4::zero()
    }
}

/// Saturates components (amount > 0).
pub fn overdrive(amount: f32, v: Vec4) -> Vec4 {
    softsign(v * amount)
}

/// Saturates the input while retaining component proportions (amount > 0).
pub fn overdrive4(amount: f32, v: Vec4) -> Vec4 {
    // Use the 8-norm as a smooth proxy for the largest magnitude component.
    let b = v * v;
    let b = b * b;
    let m = b.length();
    
    if m > 0.0 {
        let m = sqrt(sqrt(m));
        v / m * softsign(m * amount)
    } else {
        Vec4::zero()
    }
}

pub fn posterize(levels: f32, sharpness: f32, v: Vec4) -> Vec4 {
    let v = v * levels;
    let b = v.floor();
    let t = v - b;
    let p0 = smooth5(t);
    let p1 = p0 * p0;
    let p2 = p1 * p1;
    let p3 = p2 * p2;
    let p4 = p3 * p3;
    let p5 = p4 * p4;
    let p6 = p5 * p5; // t ** 320
    (b + lerp(p0, p6 * p6, sharpness)) / levels
}

pub fn posterize4(levels: f32, sharpness: f32, v: Vec4) -> Vec4 {
    //let m = levels * v.abs().max_element();
    let m = levels * v.length();
    if m > 0.0 {
        let b = m.floor();
        let t = m - b;
        let power: f32 = 1.0 + 1000.0 * squared(squared(sharpness));
        let p = if t < 0.5 {
            0.5 * pow(2.0 * t, power)
        } else {
            1.0 - 0.5 * pow(2.0 * (1.0 - t), power)
        };
        v * ((b + p) / m)
    } else {
        Vec4::zero()
    }
}
