use super::hash::*;
use super::math::*;
use glam::*;

/*
3-D procedural texture library: procedural self-maps in 3-space.
The preferred scale of texture values is [-1, 1] in each component.
*/

/// Bases attach a feature grid to a queried point.
/// Once attached, the grid is resolution independent:
/// scale of the grid is decided prior to obtaining a basis.
pub struct Basis {
    /// Texture specific seed value.
    pub seed: u64,
    /// X grid coordinate of point.
    pub ix: u32,
    /// Y grid coordinate of point.
    pub iy: u32,
    /// Z grid coordinate of point.
    pub iz: u32,
    /// For repeating hashers: total number of X tiles.
    pub sx: u32,
    /// For repeating hashers: total number of Y tiles.
    pub sy: u32,
    /// For repeating hashers: total number of Z tiles.
    pub sz: u32,
    /// Position inside cell with components in left-closed range [0, 1[.
    pub d: Vec3A,
}

/// Hashers supply data for grid cells and determine the topology of the procedural texture.
pub trait Hasher {
    /// Builds a grid around a point. The seed is texture specific.
    fn query(&self, seed: u64, frequency: f32, point: Vec3A) -> Basis;
    /// Hashes X coordinate. Supply any previous coordinate hashes in the previous argument.
    fn hash_x(&self, basis: &Basis, previous: u64, dx: i32) -> u64;
    /// Hashes Y coordinate. Supply any previous coordinate hashes in the previous argument.
    fn hash_y(&self, basis: &Basis, previous: u64, dy: i32) -> u64;
    /// Hashes Z coordinate. Supply any previous coordinate hashes in the previous argument.
    fn hash_z(&self, basis: &Basis, previous: u64, dz: i32) -> u64;
    /// Returns a code string that constructs this hasher.
    fn get_code(&self) -> String;
}

/// Returns a pseudorandom vector from seed with components in left-closed range [0, 1[.
pub fn hash_01(seed: u64) -> Vec3A {
    let h = hash64a(seed);
    let c: f32 = 1.0 / (1 << 21) as f32;
    vec3a(
        (h & 0x1fffff) as f32,
        ((h >> 21) & 0x1fffff) as f32,
        (h >> 43) as f32,
    ) * c
}

/// Returns a pseudorandom vector from seed with components in closed range [-1, 1].
pub fn hash_11(seed: u64) -> Vec3A {
    let h = hash64b(seed);
    let c: f32 = 2.0 / (1 << 21) as f32;
    vec3a(
        (h & 0x1fffff) as f32,
        ((h >> 21) & 0x1fffff) as f32,
        (h >> 43) as f32,
    ) * c
        - Vec3A::one()
}

/// This hasher does not tile on any axis. Frequencies are not rounded to nearest integer.
pub struct TileNone {}

pub fn tile_none() -> TileNone {
    TileNone {}
}

impl Hasher for TileNone {
    fn query(&self, seed: u64, frequency: f32, point: Vec3A) -> Basis {
        let p = frequency * point + hash_01(seed);
        let i = p.floor();
        Basis {
            seed,
            ix: (i.x as i32) as u32,
            iy: (i.y as i32) as u32,
            iz: (i.z as i32) as u32,
            sx: 0,
            sy: 0,
            sz: 0,
            d: p - i,
        }
    }
    fn hash_x(&self, basis: &Basis, current: u64, dx: i32) -> u64 {
        let x = basis.ix.wrapping_add(dx as u32);
        hash64a(current ^ x as u64 ^ basis.seed as u64)
    }
    fn hash_y(&self, basis: &Basis, current: u64, dy: i32) -> u64 {
        let y = basis.iy.wrapping_add(dy as u32);
        hash64b(current ^ y as u64)
    }
    fn hash_z(&self, basis: &Basis, current: u64, dz: i32) -> u64 {
        let z = basis.iz.wrapping_add(dz as u32);
        hash64c(current ^ z as u64)
    }
    fn get_code(&self) -> String {
        String::from("tile_none()")
    }
}

/// This hasher tiles all coordinate axes.
/// Frequencies are rounded to the nearest positive integer.
pub struct TileAll {
    sx: u32,
    sy: u32,
    sz: u32,
}

pub fn tile_all() -> TileAll {
    TileAll {
        sx: 1,
        sy: 1,
        sz: 1,
    }
}
pub fn tile_all_in(sx: u32, sy: u32, sz: u32) -> TileAll {
    TileAll { sx, sy, sz }
}

impl Hasher for TileAll {
    fn query(&self, seed: u64, frequency: f32, point: Vec3A) -> Basis {
        let fr = frequency.round().max(1.0);
        let fi = fr as u32;
        let p = fr * point + hash_01(seed);
        let i = p.floor();
        let sx = self.sx * fi;
        let sy = self.sy * fi;
        let sz = self.sz * fi;
        Basis {
            seed,
            ix: (i.x as i32).rem_euclid(sx as i32) as u32,
            iy: (i.y as i32).rem_euclid(sy as i32) as u32,
            iz: (i.z as i32).rem_euclid(sz as i32) as u32,
            sx,
            sy,
            sz,
            d: p - i,
        }
    }
    fn hash_x(&self, basis: &Basis, current: u64, dx: i32) -> u64 {
        let x = (basis.ix as i32)
            .wrapping_add(dx)
            .rem_euclid(basis.sx as i32);
        hash64a(current ^ x as u64 ^ basis.seed as u64)
    }
    fn hash_y(&self, basis: &Basis, current: u64, dy: i32) -> u64 {
        let y = (basis.iy as i32)
            .wrapping_add(dy)
            .rem_euclid(basis.sy as i32);
        hash64a(current ^ y as u64)
    }
    fn hash_z(&self, basis: &Basis, current: u64, dz: i32) -> u64 {
        let z = (basis.iz as i32)
            .wrapping_add(dz)
            .rem_euclid(basis.sz as i32);
        hash64a(current ^ z as u64)
    }
    fn get_code(&self) -> String {
        if self.sx == 1 && self.sy == 1 && self.sz == 1 {
            String::from("tile_all()")
        } else {
            format!("tile_all_in({}, {}, {})", self.sx, self.sy, self.sz)
        }
    }
}

/// This hasher tiles X and Y coordinate axes.
/// Frequencies are rounded to the nearest positive integer.
pub struct TileXY {
    sx: u32,
    sy: u32,
}

pub fn tile_xy() -> TileXY {
    TileXY { sx: 1, sy: 1 }
}
pub fn tile_xy_in(sx: u32, sy: u32) -> TileXY {
    TileXY { sx, sy }
}

impl Hasher for TileXY {
    fn query(&self, seed: u64, frequency: f32, point: Vec3A) -> Basis {
        let fr = frequency.round().max(1.0);
        let fi = fr as u32;
        let p = fr * point + hash_01(seed);
        let i = p.floor();
        let sx = self.sx * fi;
        let sy = self.sy * fi;
        Basis {
            seed,
            ix: (i.x as i32).rem_euclid(sx as i32) as u32,
            iy: (i.y as i32).rem_euclid(sy as i32) as u32,
            iz: (i.z as i32) as u32,
            sx,
            sy,
            sz: 0,
            d: p - i,
        }
    }
    fn hash_x(&self, basis: &Basis, current: u64, dx: i32) -> u64 {
        let x = (basis.ix as i32)
            .wrapping_add(dx)
            .rem_euclid(basis.sx as i32);
        hash64a(current ^ x as u64 ^ basis.seed as u64)
    }
    fn hash_y(&self, basis: &Basis, current: u64, dy: i32) -> u64 {
        let y = (basis.iy as i32)
            .wrapping_add(dy)
            .rem_euclid(basis.sy as i32);
        hash64a(current ^ y as u64)
    }
    fn hash_z(&self, basis: &Basis, current: u64, dz: i32) -> u64 {
        let z = basis.iz.wrapping_add(dz as u32);
        hash64c(current ^ z as u64)
    }
    fn get_code(&self) -> String {
        if self.sx == 1 && self.sy == 1 {
            String::from("tile_xy()")
        } else {
            format!("tile_xy_in({}, {})", self.sx, self.sy)
        }
    }
}

/// Textures are self-maps in 3-space.
pub trait Texture {
    fn at(&self, point: Vec3A) -> Vec3A;
    fn get_code(&self) -> String;
    
}

/// Basis textures accept an additional frequency argument.
pub trait BasisTexture {
    fn at_frequency(&self, frequency: f32, point: Vec3A) -> Vec3A;
    fn get_basis_code(&self) -> String;
}

/// Roughly isotropic value noise.
pub struct VNoise<H: Hasher> {
    seed: u64,
    frequency: f32,
    hasher: H,
}

pub fn vnoise<H: Hasher>(seed: u64, frequency: f32, hasher: H) -> VNoise<H> {
    VNoise {
        seed,
        frequency,
        hasher,
    }
}

pub fn vnoise_basis<H: Hasher>(seed: u64, hasher: H) -> VNoise<H> {
    VNoise { seed, frequency: 1.0, hasher }
}

impl<H: Hasher> BasisTexture for VNoise<H> {
    fn at_frequency(&self, frequency: f32, point: Vec3A) -> Vec3A {
        let basis = self.hasher.query(self.seed, frequency, point);
        let mut result = Vec3A::zero();

        for dx in -1..=1 {
            let hx = self.hasher.hash_x(&basis, 0, dx);
            for dy in -1..=1 {
                let hxy = self.hasher.hash_y(&basis, hx, dy);
                let mut offset = Vec3A::new(dx as f32, dy as f32, 0.0) - basis.d;
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

    fn get_basis_code(&self) -> String {
        format!(
            "vnoise_basis({}, {})",
            self.seed,
            self.hasher.get_code()
        )
    }
}

impl<H: Hasher> Texture for VNoise<H> {
    fn at(&self, point: Vec3A) -> Vec3A {
        self.at_frequency(self.frequency, point)
    }

    fn get_code(&self) -> String {
        format!(
            "vnoise({}, {}, {})",
            self.seed,
            self.frequency,
            self.hasher.get_code()
        )
    }
}

/// Saturates components.
pub struct Saturate {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

/// Saturates components.
impl Texture for Saturate {
    fn at(&self, point: Vec3A) -> Vec3A {
        softsign(self.texture.at(point) * self.amount)
    }
    fn get_code(&self) -> String {
        format!("saturate({}, {})", self.amount, self.texture.get_code())
    }
}

/// Saturates components (amount > 0).
/// Amount equals derivative at origin. Amounts greater than 1 result in overdrive.
pub fn saturate(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture>
{
    assert!(amount > 0.0);
    Box::new(Saturate { amount, texture })
}

/// Reflect: applies a wavy function to texture values with an offset, which can
/// spread and reflect components.
pub struct Reflect {
    amount: f32,
    offset: Vec3A,
    texture: Box<dyn Texture>,
}

impl Texture for Reflect {
    fn at(&self, point: Vec3A) -> Vec3A {
        wave(smooth3, self.offset + self.texture.at(point) * self.amount)
    }
    fn get_code(&self) -> String {
        format!("reflect({}, vec3a({}, {}, {}), {})", self.amount, self.offset.x, self.offset.y, self.offset.z, self.texture.get_code())
    }
}

/// Applies a wavy function to texture values with an offset, which can
/// spread and reflect components. Amount is the scale (amount > 0),
/// roughly corresponding to number of reflections.
pub fn reflect(amount: f32, offset: Vec3A, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    Box::new(Reflect { amount, offset, texture })
}

/// Posterize: applies a smooth step function in proportion to texture value magnitude.
pub struct Posterize {
    levels: f32,
    sharpness: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Posterize {
    fn at(&self, point: Vec3A) -> Vec3A {
        let v = self.texture.at(point);
        let magnitude = self.levels * v.length();
        if magnitude > 0.0 {
            let base = magnitude.floor();
            let t = magnitude - base;
            let power: f32 = 1.0 + 100.0 * squared(squared(self.sharpness));
            let p = if t < 0.5 {
                0.5 * pow(2.0 * t, power)
            } else {
                1.0 - 0.5 * pow(2.0 * (1.0 - t), power)
            };
            v * ((base + p) / magnitude)
        } else {
            Vec3A::zero()
        }
    }
    fn get_code(&self) -> String {
        format!("posterize({}, {}, {})", self.levels, self.sharpness, self.texture.get_code())
    }
}

/// Applies a wavy function to texture values with an offset, which can
/// spread and reflect components. Amount is the scale (amount > 0),
/// roughly corresponding to number of reflections.
pub fn posterize(levels: f32, sharpness: f32, texture: Box<dyn Texture>) -> Box<dyn Texture> {
    Box::new(Posterize { levels, sharpness, texture })
}

/// Saturates components while retaining component proportions.
pub struct Overdrive {
    /// Amount (amount > 0) equals derivative at origin.
    amount: f32,
    texture: Box<dyn Texture>,
}

impl Texture for Overdrive {
    fn at(&self, point: Vec3A) -> Vec3A {
        let v = self.texture.at(point);
        // Use the 4-norm as a smooth proxy for the largest magnitude component.
        let magnitude = squared(v).length_squared();
        if magnitude > 0.0 {
            let m = sqrt(sqrt(magnitude));
            v / m * softsign(m * self.amount)
        } else {
            Vec3A::zero()
        }
    }
    fn get_code(&self) -> String {
        format!("overdrive({}, {})", self.amount, self.texture.get_code())
    }
}

/// Saturates components (amount > 0).
/// Amount equals derivative at origin. Amounts greater than 1 result in overdrive.
pub fn overdrive(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture>
{
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
    fn at(&self, point: Vec3A) -> Vec3A {
        let v = self.texture.at(point);
        let m = v.length();
        if m > 0.0 {
            v * (sin(m * self.amount * f32::PI * 0.5) / m)
        } else {
            Vec3A::zero()
        }
    }
    fn get_code(&self) -> String {
        format!("vreflect({}, {})", self.amount, self.texture.get_code())
    }
}

/// Saturates components (amount > 0).
/// Amount equals derivative at origin. Amounts greater than 1 result in overdrive.
pub fn vreflect(amount: f32, texture: Box<dyn Texture>) -> Box<dyn Texture>
{
    assert!(amount > 0.0);
    Box::new(VReflect { amount, texture })
}







/// Mixes between two textures weighted with their vector magnitudes.
pub fn softmix3(amount: f32, v: Vec3A, u: Vec3A) -> Vec3A {
    let vw: f32 = softexp(v * amount).length_squared();
    let uw: f32 = softexp(u * amount).length_squared();
    let epsilon: f32 = 1.0e-9;
    (v * vw + u * uw) / (vw + uw + epsilon)
}

/// Rotates v by u.
pub fn rotate(amount: f32, v: Vec3A, u: Vec3A) -> Vec3A {
    let length: f32 = u.length();
    if length > 1.0e-9 {
        let axis = u / length;
        Quat::from_axis_angle(Vec3::from(axis), amount * length) * v
    } else {
        Vec3A::zero()
    }
}
