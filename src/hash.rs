use super::math::Int;
use wrapping_arithmetic::wrappit;

/// Quadratic probe. This is a bijective function in unsigned types.
#[wrappit] #[inline] pub fn quadp<T: Int>(x: T) -> T {
    let q = x >> 1;
    (x & T::one()) + q + q * q
}

/// 32-bit hash by Chris Wellons.
#[wrappit] #[inline] 
pub fn hasha(x: u32) -> u32 {
    let x = (x ^ (x >> 15)) * 0x2c1b3c6d;
    let x = (x ^ (x >> 12)) * 0x297a2d39;
    x ^ (x >> 15)
}

/// 32-bit hash from MurmurHash3 by Austin Appleby.
#[wrappit] #[inline] 
pub fn hashb(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x85ebca6b;
    let x = (x ^ (x >> 13)) * 0xc2b2ae35;
    x ^ (x >> 16)
}

/// 64-bit hash from SplitMix64.
#[wrappit] #[inline] 
pub fn hashc(x: u64) -> u64 {
    let x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
    let x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
    x ^ (x >> 31)
}

/// 64-bit hash by degski. Inverse of hashe.
#[wrappit] #[inline] 
pub fn hashd(x: u64) -> u64 {
    let x = (x ^ (x >> 32)) * 0xd6e8feb86659fd93;
    let x = (x ^ (x >> 32)) * 0xd6e8feb86659fd93;
    x ^ (x >> 32)
}

/// 64-bit hash by degski. Inverse of hashd.
#[wrappit] #[inline] 
pub fn hashe(x: u64) -> u64 {
    let x = (x ^ (x >> 32)) * 0xcfee444d8b59a89b;
    let x = (x ^ (x >> 32)) * 0xcfee444d8b59a89b;
    x ^ (x >> 32)
}

/// 32-bit hash by degski. Inverse of hashg.
#[wrappit] #[inline] 
pub fn hashf(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x45d9f3b;
    let x = (x ^ (x >> 16)) * 0x45d9f3b;
    x ^ (x >> 16)
}

/// 32-bit hash by degski. Inverse of hashf.
#[wrappit] #[inline] 
pub fn hashg(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x119de1f3;
    let x = (x ^ (x >> 16)) * 0x119de1f3;
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Inverse of hashi.
#[wrappit] #[inline] 
pub fn hashh(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x7feb352d;
    let x = (x ^ (x >> 15)) * 0x846ca68b;
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Inverse of hashh.
#[wrappit] #[inline] 
pub fn hashi(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x43021123;
    let x = (x ^ (x >> 15) ^ (x >> 30)) * 0x1d69e2a5;
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Extra high quality.
#[wrappit] #[inline] 
pub fn hashj(x: u32) -> u32 {
    let x = (x ^ (x >> 17)) * 0xed5ad4bb;
    let x = (x ^ (x >> 11)) * 0xac4c1b51;
    let x = (x ^ (x >> 15)) * 0x31848bab;
    x ^ (x >> 14)
}

/// 64-bit hash by Thomas Wang.
#[wrappit] #[inline] 
pub fn hashk(x: u64) -> u64 {
    let x = !x + (x << 21);
    let x = x ^ (x >> 24);
    let x = x + (x << 3) + (x << 8);
    let x = x ^ (x >> 14);
    let x = x + (x << 2) + (x << 4);
    let x = x ^ (x >> 28);
    x + (x << 31)
}

/// 128-to-64-bit hash from CityHash by Geoff Pike and Jyrki Alakuijala.
#[wrappit] #[inline] 
pub fn hashm(x: u128) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let y = (x >> 64) as u64;
    let a = (y ^ x as u64) * C;
    let a = a ^ (a >> 47);
    let a = (a ^ x as u64) * C;
    (a ^ (a >> 47)) * C
}

/// 64-bit hash from FarmHash by Geoff Pike and Jyrki Alakuijala.
#[wrappit] #[inline] 
pub fn hashn(x: u64) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let x = x * C;
    let x = (x ^ (x >> 44)) * C;
    (x ^ (x >> 41)) * C
}

/// 128-to-64-bit hash from FarmHash by Geoff Pike and Jyrki Alakuijala.
#[wrappit] #[inline] 
pub fn hashp(x: u128) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let y = (x >> 64) as u64;
    let a = (x as u64 ^ y) * C;
    let a = (y ^ a ^ (a >> 47)) * C;
    let a = (a ^ (a >> 44)) * C;
    let a = (a ^ (a >> 41)) * C;
    a
}

/// 64-bit hash from MurmurHash3 by Austin Appleby.
#[wrappit] #[inline] 
pub fn hashq(x: u64) -> u64 {
    let x = (x ^ (x >> 33)) * 0xff51afd7ed558ccd;
    let x = (x ^ (x >> 33)) * 0xc4ceb9fe1a85ec53;
    x ^ (x >> 33)
}

/// 64-bit hash SplitMix64. Extra high quality.
/// Passes PractRand as an indexed RNG.
#[wrappit] #[inline] 
pub fn hashr(x: u64) -> u64 {
    let x = x * 0x9e3779b97f4a7c15;
    let x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
    let x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
    x ^ (x >> 31)
}
