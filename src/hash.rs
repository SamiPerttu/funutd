//! Hash functions. All of the hashes here are permutations.

/// 32-bit hash by degski. Inverse of `hash32a_inverse`.
#[inline]
pub fn hash32a(x: u32) -> u32 {
    let x = (x ^ (x >> 16)).wrapping_mul(0x45d9f3b);
    let x = (x ^ (x >> 16)).wrapping_mul(0x45d9f3b);
    x ^ (x >> 16)
}

/// 32-bit hash by degski. Inverse of `hash32a`.
#[inline]
pub fn hash32a_inverse(x: u32) -> u32 {
    let x = (x ^ (x >> 16)).wrapping_mul(0x119de1f3);
    let x = (x ^ (x >> 16)).wrapping_mul(0x119de1f3);
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Inverse of `hash32b_inverse`.
#[inline]
pub fn hash32b(x: u32) -> u32 {
    let x = (x ^ (x >> 16)).wrapping_mul(0x7feb352d);
    let x = (x ^ (x >> 15)).wrapping_mul(0x846ca68b);
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Inverse of `hash32b`.
#[inline]
pub fn hash32b_inverse(x: u32) -> u32 {
    let x = (x ^ (x >> 16)).wrapping_mul(0x43021123);
    let x = (x ^ (x >> 15) ^ (x >> 30)).wrapping_mul(0x1d69e2a5);
    x ^ (x >> 16)
}

/// 32-bit hash from MurmurHash3 by Austin Appleby.
#[inline]
pub fn hash32c(x: u32) -> u32 {
    let x = (x ^ (x >> 16)).wrapping_mul(0x85ebca6b);
    let x = (x ^ (x >> 13)).wrapping_mul(0xc2b2ae35);
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellons.
#[inline]
pub fn hash32d(x: u32) -> u32 {
    let x = (x ^ (x >> 15)).wrapping_mul(0x2c1b3c6d);
    let x = (x ^ (x >> 12)).wrapping_mul(0x297a2d39);
    x ^ (x >> 15)
}

/// 32-bit hash by Chris Wellon. Extra high quality.
#[inline]
pub fn hash32e(x: u32) -> u32 {
    let x = (x ^ (x >> 17)).wrapping_mul(0xed5ad4bb);
    let x = (x ^ (x >> 11)).wrapping_mul(0xac4c1b51);
    let x = (x ^ (x >> 15)).wrapping_mul(0x31848bab);
    x ^ (x >> 14)
}

/// 64-bit hash by degski. Inverse of `hash64a_inverse`.
#[inline]
pub fn hash64a(x: u64) -> u64 {
    let x = (x ^ (x >> 32)).wrapping_mul(0xd6e8feb86659fd93);
    let x = (x ^ (x >> 32)).wrapping_mul(0xd6e8feb86659fd93);
    x ^ (x >> 32)
}

/// 64-bit hash by degski. Inverse of `hash64a`.
#[inline]
pub fn hash64a_inverse(x: u64) -> u64 {
    let x = (x ^ (x >> 32)).wrapping_mul(0xcfee444d8b59a89b);
    let x = (x ^ (x >> 32)).wrapping_mul(0xcfee444d8b59a89b);
    x ^ (x >> 32)
}

/// 64-bit hash from MurmurHash3 by Austin Appleby.
#[inline]
pub fn hash64b(x: u64) -> u64 {
    let x = (x ^ (x >> 33)).wrapping_mul(0xff51afd7ed558ccd);
    let x = (x ^ (x >> 33)).wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^ (x >> 33)
}

/// 64-bit hash by Thomas Wang.
#[inline]
pub fn hash64c(x: u64) -> u64 {
    let x = !x.wrapping_add(x << 21);
    let x = x ^ (x >> 24);
    let x = x.wrapping_add(x << 3).wrapping_add(x << 8);
    let x = x ^ (x >> 14);
    let x = x.wrapping_add(x << 2).wrapping_add(x << 4);
    let x = x ^ (x >> 28);
    x.wrapping_add(x << 31)
}

/// 64-bit hash from FarmHash by Geoff Pike and Jyrki Alakuijala.
#[inline]
pub fn hash64d(x: u64) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let x = x.wrapping_mul(C);
    let x = (x ^ (x >> 44)).wrapping_mul(C);
    (x ^ (x >> 41)).wrapping_mul(C)
}

/// 64-bit hash SplitMix64. Extra high quality.
/// Passes PractRand as an indexed RNG.
#[inline]
pub fn hash64e(x: u64) -> u64 {
    let x = x.wrapping_mul(0x9e3779b97f4a7c15);
    let x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

/// Fast 64-bit hash from FXHasher.
#[inline]
pub fn hash64f(x: u64) -> u64 {
    (x.rotate_left(5) ^ x).wrapping_mul(0x517cc1b727220a95)
}

/// 64-bit hash from Krull64 RNG output stage.
/// Extra high quality - can be used as an indexed RNG.
pub fn hash64g(x: u64) -> u64 {
    let x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    let x = (x ^ (x >> 31)).wrapping_mul(0xd6e8feb86659fd93);
    x ^ (x >> 32)
}

/// 128-to-64-bit hash from CityHash by Geoff Pike and Jyrki Alakuijala.
#[inline]
pub fn hash128a(x: u128) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let y = (x >> 64) as u64;
    let a = (y ^ x as u64).wrapping_mul(C);
    let a = a ^ (a >> 47);
    let a = (a ^ x as u64).wrapping_mul(C);
    (a ^ (a >> 47)).wrapping_mul(C)
}

/// 128-to-64-bit hash from FarmHash by Geoff Pike and Jyrki Alakuijala.
#[inline]
pub fn hash128b(x: u128) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let y = (x >> 64) as u64;
    let a = (x as u64 ^ y).wrapping_mul(C);
    let a = (y ^ a ^ (a >> 47)).wrapping_mul(C);
    let a = (a ^ (a >> 44)).wrapping_mul(C);
    (a ^ (a >> 41)).wrapping_mul(C)
}
