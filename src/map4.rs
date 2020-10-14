use super::math::*;
use super::hash::*;
use glam::*;
use wrapping_arithmetic::wrappit;

#[inline] pub fn frc(x: u64) -> f32 { (x & 0xff) as f32 }
#[inline] pub fn grc(x: u64) -> f32 { (x & 0xff) as f32 }
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

    #[inline] pub fn point(&self, h: u64) -> Vec3A {
        vec3a(frc(h) as f32, frc(h >> 8) as f32, frc(h >> 16) as f32) * (1.0 / 256.0)
    }
    #[inline] pub fn gradient(&self, h: u64) -> Vec3A {
        vec3a(grc(h) as f32, grc(h >> 8) as f32, grc(h >> 16) as f32) * (2.0 / 255.0) - Vec3A::one()
    }
    #[inline] pub fn color(&self, h: u64) -> Vec4 {
        vec4(src(h) as f32, src(h >> 8) as f32, src(h >> 16) as f32, src(h >> 24) as f32) * (2.0 / 255.0) - Vec4::one()
    }
}


pub fn filter_point(origin: u64, cell: u64, cell_hash: u64) -> bool {
    let two = 0x10 | (0x10 << 10) | (0x10 << 20);
    let _one = 0x08 | (0x08 << 10) | (0x08 << 20);
    let fraction = 0x07 | (0x07 << 10) | (0x07 << 20);

    let point = cell + (cell_hash & fraction);
    let delta = origin + two - point;
    let sign = delta & two;
    let dup_sign = sign | (sign >> 1) | (sign >> 2);
    let dup_sign = dup_sign | (dup_sign >> 2);
    let distance = delta ^ dup_sign;
    let squared = distance * distance;
    let sum = (squared & 0x3ff) + ((squared >> 10) & 0x3ff) + ((squared >> 20) & 0x3ff);
    sum < 0x40
}

/*
Let's use bit-parallelism to accelerate noise computations.
We can't escape having to compute 27 cell hashes but maybe we can improve point filtering.
After we hit something, then we can afford to do more work.

We have 64 bits. From the bits, we want to rejection sample all points in 1 go.
-Did I already determine I need more than 2 points/cell?
-Anyway, try with 2 points/cell first, 3 points/cell is more difficult.

We have 32 bits available per 3 components. That's 10 bits per component.
We're going to use 5 fractional bits so we have enough room to multiply fractions.
Our point location error will be 1/32 + 1/32 = 0.0625 at a maximum, which is OK.
The bit arrangement for component:

origin:  00001fffff
offset: +0001000000
point:  -000uufffff (uu = 00, 01 or 10)

We have added an offset of 2.0 to origin to make sure it doesn't underflow.
If a point is missing, we set its position to all zeros and it will be rejected.
Now we have these 10-bit patterns.

delta: 000ssfffff

Examine the first three bits. (000 is impossible)

100 = distance 0.0 .. 0.5
011 = distance 0.0 .. 0.5
101 = distance 0.5 .. 1.0
010 = distance 0.5 .. 1.0
110 = distance 1.0 .. 1.5
001 = distance 1.0 .. 1.5
111 = distance 1.5 .. 2.0

Okay, we can see that if the highest bit is set, we can flip the lower bits and
get the same value as positive. We can do this quite fast by duplicating the sign bit.
Let's see... three shifts are needed, that's fine.
So now we have absolute axial distances as 6-bit quantities:

distance: 0000ufffff

If any of the 'u' bits are 1, then we can discard the point immediately.
(We can do this earlier already, actually.)
otherwise, we have the remaining fractional axis distances:

distance: 00000fffff

Now we can multiply to get the distance squared.

squared:  ffffffffff

Finally, we add the results together by shifting and masking and get a 12-bit quantity
with 10 fractional bits representing the distance squared. If the first two bits are
zero, then we are close to the point. Next we need to build the exact vectors as floats
and compute the distance again using floating point.
*/


pub fn noise3(v: Vec4) -> Vec4 {
    let basis = Basis3::new(v.truncate());
    let mut result = Vec4::zero();
    for dx in -1 ..= 1 {
        for dy in -1 ..= 1 {
            let hxy = basis.hash_xy(0, dx, dy);
            let mut offset = Vec3A::new(dx as f32, dy as f32, 0.0) - basis.d;
            for dz in -1 ..= 1 {
                let mut h = basis.hash_z(hxy, dz);
                // Pick number of cells as a rough approximation to a Poisson distribution.
                //let n = match h & 7 { 0 | 1 | 2 | 3 | 4 => 1, 5 | 6 | 7 | _ => 2 };
                //let n = match h & 7 { 0 | 1 | 2 | 3 | 4 => 2, 5 | 6 | 7 | _ => 3 };
                let n = 1 + (h & 1);
                //let n = match h & 7 { 0 | 1 | 2 | 3 => 1, 5 | 6 => 2, 7 | _ => 3 };
                //let n = match h & 7 { 0 | 1 | 2 => 1, 3 | 4 | 5 => 2, 6 | 7 | _ => 3 };
                offset.set_z(dz as f32 - basis.d.z());
                //let offset = Vec3A::new(dx as f32, dy as f32, dz as f32) - basis.d;
                for di in 0 .. n {
                    let p = basis.point(h >> 8);
                    let D: f32 = (p + offset).length_squared();
                    //let M: f32 = 0.8 + ((h >> 3) & 31) as f32 * (0.2 / 31.0);
                    let M: f32 = 1.0 - (((h >> 3) & 31) as f32 / 31.0) * (15.0 / 31.0);
                    if D < M * M {
                        let D = sqrt(D) / M;
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
        Vec3A::zero().extend(v.w())
    }
}

pub fn softmix4(amount: f32, v: Vec4, u: Vec4) -> Vec4 {
    let vw: f32 = exq(v * amount).length_squared();
    let uw: f32 = exq(u * amount).length_squared();
    let epsilon: f32 = 1.0e-10;
    (v * vw + u * uw) / (vw + uw + epsilon)
}

pub fn reflect(amount: f32, v: Vec4) -> Vec4 {
    wave(smooth3, v * amount)
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
    let m = squared(squared(v)).length();
    
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
