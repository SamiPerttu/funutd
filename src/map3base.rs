//! Base definitions for textures.

use super::hash::*;
use super::math::*;
use super::*;

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
    pub d: Vec3a,
}

/// Hashers supply data for grid cells and determine the topology of the procedural texture.
pub trait Hasher: Clone + Sync + Send {
    /// Builds a grid around a point. The seed is texture specific.
    fn query(&self, seed: u64, frequency: f32, point: Vec3a) -> Basis;

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
pub fn hash_01(seed: u64) -> Vec3a {
    let h = hash64a(seed);
    let c: f32 = 1.0 / (1 << 21) as f32;
    vec3a(
        (h & 0x1fffff) as f32,
        ((h >> 21) & 0x1fffff) as f32,
        (h >> 43) as f32,
    ) * c
}

/// Returns a pseudorandom vector from seed with components in closed range [-1, 1].
pub fn hash_11(seed: u64) -> Vec3a {
    let h = hash64b(seed);
    let c: f32 = 2.0 / (1 << 21) as f32;
    vec3a(
        (h & 0x1fffff) as f32,
        ((h >> 21) & 0x1fffff) as f32,
        (h >> 43) as f32,
    ) * c
        - Vec3a::one()
}

/// Returns a pseudorandom unit length vector.
pub fn hash_unit(mut seed: u64) -> Vec3a {
    loop {
        let v = hash_11(seed);
        let length2 = v.length_squared();
        if length2 <= 1.0 {
            return if length2 > 0.0 { v / sqrt(length2) } else { v };
        }
        seed = hash64d(seed);
    }
}

/// This hasher does not tile on any axis. Frequencies are not rounded to nearest integer.
#[derive(Clone)]
pub struct TileNone {}

pub fn tile_none() -> TileNone {
    TileNone {}
}

impl Hasher for TileNone {
    fn query(&self, seed: u64, frequency: f32, point: Vec3a) -> Basis {
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
#[derive(Clone)]
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
    fn query(&self, seed: u64, frequency: f32, point: Vec3a) -> Basis {
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
#[derive(Clone)]
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
    fn query(&self, seed: u64, frequency: f32, point: Vec3a) -> Basis {
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
pub trait Texture: Sync + Send {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a;
    fn at(&self, point: Vec3a) -> Vec3a {
        self.at_frequency(point, None)
    }
    fn get_code(&self) -> String;
    fn get_basis_code(&self) -> String;
}
