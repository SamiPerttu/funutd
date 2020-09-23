use num_traits::{AsPrimitive, float::FloatCore};
use std::ops::{Add, Sub, Mul, Div};
use wrapping_arithmetic::wrappit;

pub trait Real {
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn log(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

pub fn sqrt<F: Real>(x: F) -> F { x.sqrt() }
pub fn exp<F: Real>(x: F) -> F { x.exp() }
pub fn log<F: Real>(x: F) -> F { x.log() }
pub fn sin<F: Real>(x: F) -> F { x.sin() }
pub fn cos<F: Real>(x: F) -> F { x.cos() }
pub fn tan<F: Real>(x: F) -> F { x.tan() }

impl<F: num_traits::real::Real> Real for F {
    fn sqrt(self) -> F { self.sqrt() }    
    fn exp(self) -> F { self.exp() }
    fn log(self) -> F { self.ln() }
    fn sin(self) -> F { self.sin() }
    fn cos(self) -> F { self.cos() }
    fn tan(self) -> F { self.tan() }
}

pub use num_traits::sign::abs;

pub const SQRT_2: f64 = std::f64::consts::SQRT_2;
pub const E: f64 = std::f64::consts::E;
pub const PI: f64 = std::f64::consts::PI;
pub const TAU: f64 = std::f64::consts::TAU;

/// Cast between primitive types.
pub fn cast<T : AsPrimitive<U>, U : Copy + 'static>(t: T) -> U { t.as_() }

pub trait At<T : Copy> {
    fn at<I: AsPrimitive<usize> >(&self, i: I) -> T;
}
impl<T : Copy> At<T> for Vec<T> {
    fn at<I: AsPrimitive<usize> >(&self, i: I) -> T { self[i.as_()] }
}

pub trait Min {
    fn min(self, other: Self) -> Self;
}

/// Minimum of x and y.
pub fn min<T: Min>(x: T, y: T) -> T { x.min(y) }

impl Min for f32 {
    fn min(self, other: Self) -> Self { self.min(other) }
}
impl Min for f64 {
    fn min(self, other: Self) -> Self { self.min(other) }
}

macro_rules! impl_min {
    ( $($t:ty),* ) => {
    $( impl Min for $t {
        fn min(self, other: Self) -> Self { std::cmp::min(self, other) }
    }) *
    }
}
impl_min! { i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 }

pub trait Max {
    fn max(self, other: Self) -> Self;
}

/// Maximum of x and y.
pub fn max<T: Max>(x: T, y: T) -> T { x.max(y) }

impl Max for f32 {
    fn max(self, other: Self) -> Self { self.max(other) }
}
impl Max for f64 {
    fn max(self, other: Self) -> Self { self.max(other) }
}

macro_rules! impl_max {
    ( $($t:ty),* ) => {
    $( impl Max for $t {
        fn max(self, other: Self) -> Self { std::cmp::max(self, other) }
    }) *
    }
}
impl_max! { i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 }

pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}

impl<T: num_traits::Zero> Zero for T { fn zero() -> Self { T::zero() } }
impl<T: num_traits::One> One for T { fn one() -> Self { T::one() } }

/// Clamps x between min and max.
pub fn clamp<T: Max + Min>(min: T, max: T, x: T) -> T { x.max(min).min(max) }

/// Clamps x between 0 and 1.
pub fn clamp01<T: Min + Max + Zero + One>(x: T) -> T { x.max(T::zero()).min(T::one()) }

pub trait Lerp<T> {
    fn lerp(self, other: Self, t: T) -> Self;
}

impl<U, T> Lerp<T> for U where U: Add<Output = U> + Mul<T, Output = U>, T: Copy + One + Sub<Output = T> {
    fn lerp(self, other: U, t: T) -> U {
        self * (T::one() - t) + other * t
    }
}

pub fn lerp<U: Lerp<T>, T>(a: U, b: U, t: T) -> U { a.lerp(b, t) }

pub fn lerp01<U: Lerp<T>, T: Min + Max + Zero + One>(a: U, b: U, t: T) -> U { lerp(a, b, clamp01(t)) }

pub fn delerp<T: Copy + Sub<Output = T> + Div<Output = T>>(a: T, b: T, x: T) -> T { (x - a) / (b - a) }

pub fn xerp<U: Lerp<T> + Real, T>(a: U, b: U, t: T) -> U { exp(lerp(log(a), log(b), t)) }

pub fn xerp01<U: Lerp<T> + Real, T: Min + Max + Zero + One>(a: U, b: U, t: T) -> U { exp(lerp(log(a), log(b), clamp01(t))) }

pub fn dexerp<T: Copy + Div<Output = T> + Real>(a: T, b: T, x: T) -> T { log(x / a) / log(b / a) }

/// Linear congruential generator from Numerical Recipes. Cycles through all u32 values.
pub fn lcg32(x: u32) -> u32 { x * 1664525 + 1013904223 }

/// Linear congruential generator by Donald Knuth. Cycles through all u64 values.
pub fn lcg64(x: u64) -> u64 { x * 6364136223846793005 + 1442695040888963407 }

/// Encodes to binary reflected Gray code.
pub fn gray(x: u32) -> u32 {
    x ^ (x >> 1)
}

/// Decodes from binary reflected Gray code.
pub fn degray(x: u32) -> u32 {
    let x = x ^ (x >> 16);
    let x = x ^ (x >> 8);
    let x = x ^ (x >> 4);
    let x = x ^ (x >> 2);
    x ^ (x >> 1)
}

/// Rounds to a multiple of step.
pub fn discretize<F: FloatCore>(x: F, step: F) -> F { (x / step).round() * step }

/// Square of x.
pub fn squared<T: Mul<Output = T> + Copy>(x: T) -> T { x * x }

/// x cubed.
pub fn cubed<T: Mul<Output = T> + Copy>(x: T) -> T { x * x * x }

/// Smooth cubic fade curve.
pub fn smooth3<F: FloatCore + 'static>(x: F) -> F where i8: AsPrimitive<F> {
    (cast(3) - cast(2) * x) * x * x
}

/// Smooth quintic fade curve suggested by Ken Perlin.
pub fn smooth5<F: FloatCore + 'static>(x: F) -> F where i8: AsPrimitive<F> {
    ((x * cast(6) - cast(15)) * x + cast(10)) * x * x * x
}

/// Smooth septic fade curve suggested by Ken Perlin.
pub fn smooth7<F: FloatCore + 'static>(x: F) -> F where i8: AsPrimitive<F> {
    let x2 = x * x;
    x2 * x2 * (cast(35) - cast(84) * x + (cast(70) - cast(20) * x) * x2)
}

/// A quarter circle fade that slopes upwards. Inverse function of Fade.downarc.
pub fn uparc<F: Real + FloatCore + Max>(x: F) -> F {
    F::one() - sqrt(max(F::zero(), F::one() - x * x))
}

/// A quarter circle fade that slopes downwards. Inverse function of Fade.uparc.
pub fn downarc<F: Real + FloatCore + Max + 'static>(x: F) -> F where i8: AsPrimitive<F> {
    sqrt(max(cast(0), (cast(2) - x) * x))
}

/// Catmull-Rom cubic spline interpolation, which is a form of cubic Hermite spline. Interpolates between
/// y1 (returns y1 when x = 0) and y2 (returns y2 when x = 1) while using the previous (y0) and next (y3)
/// points to define slopes at the endpoints. The maximum overshoot is 1/8th of the range of the arguments.
pub fn spline<F: FloatCore + 'static>(y0: F, y1: F, y2: F, y3: F, x: F) -> F where i8: AsPrimitive<F> {
    y1 + x / cast(2) * (y2 - y0 + x * (cast(2) * y0 - cast(5) * y1 + cast(4) * y2 - y3 + x * (cast(3) * (y1 - y2) + y3 - y0)))
}

/// Monotonic cubic interpolation via Steffen's method. The result never overshoots.
/// It is first order continuous. Interpolates between y1 (at x = 0) and y2 (at x = 1)
/// while using the previous (y0) and next (y3) values to influence slopes.
pub fn spline_mono<F: FloatCore + 'static>(y0: F, y1: F, y2: F, y3: F, x: F) -> F where i8: AsPrimitive<F> {
  let d0 = y1 - y0;
  let d1 = y2 - y1;
  let d2 = y3 - y2;
  let d1d = (d0.signum() + d1.signum()) * (d0 + d1).min(d0.abs().min(d1.abs()));
  let d2d = (d1.signum() + d2.signum()) * (d1 + d2).min(d1.abs().min(d2.abs()));
  cubed(x) * (cast(2) * y1 - cast(2) * y2 + d1d + d2d) + squared(x) * (cast(-3) * y1 + cast(3) * y2 - cast(2) * d1d - d2d) + x * d1d + y1
}

/// Logistic sigmoid.
pub fn logistic<F: Real + FloatCore>(x: F) -> F {
    F::one() / (F::one() + exp(-x))
}

/// Derivative of the logistic sigmoid.
pub fn logistic_d<F: Real + FloatCore>(x: F) -> F {
    let y = logistic(x);
    y * (F::one() - y)
}

/// Softsign function.
pub fn softsign<F: FloatCore>(x: F) -> F {
    x / (F::one() + x.abs())
}

/// Derivative of the softsign function.
pub fn softsign_d<F: FloatCore>(x: F) -> F {
    F::one() / squared(F::one() + x.abs())
}

/// 32-bit hash by Chris Wellons.
#[wrappit]
pub fn hasha(x: u32) -> u32 {
    let x = (x ^ (x >> 15)) * 0x2c1b3c6d;
    let x = (x ^ (x >> 12)) * 0x297a2d39;
    x ^ (x >> 15)
}

/// 32-bit hash from MurmurHash3 by Austin Appleby.
#[wrappit]
pub fn hashb(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x85ebca6b;
    let x = (x ^ (x >> 13)) * 0xc2b2ae35;
    x ^ (x >> 16)
}

/// 64-bit hash SplitMix64 by Sebastiano Vigna.
#[wrappit]
pub fn hashc(x: u64) -> u64 {
    let x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
    let x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
    x ^ (x >> 31)
}

/// 64-bit hash by degski. Inverse of hashe.
#[wrappit]
pub fn hashd(x: u64) -> u64 {
    let x = (x ^ (x >> 32)) * 0xd6e8feb86659fd93;
    let x = (x ^ (x >> 32)) * 0xd6e8feb86659fd93;
    x ^ (x >> 32)
}

/// 64-bit hash by degski. Inverse of hashd.
#[wrappit]
pub fn hashe(x: u64) -> u64 {
    let x = (x ^ (x >> 32)) * 0xcfee444d8b59a89b;
    let x = (x ^ (x >> 32)) * 0xcfee444d8b59a89b;
    x ^ (x >> 32)
}

/// 32-bit hash by degski. Inverse of hashg.
#[wrappit]
pub fn hashf(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x45d9f3b;
    let x = (x ^ (x >> 16)) * 0x45d9f3b;
    x ^ (x >> 16)
}

/// 32-bit hash by degski. Inverse of hashf.
#[wrappit]
pub fn hashg(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x119de1f3;
    let x = (x ^ (x >> 16)) * 0x119de1f3;
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Inverse of hashi.
#[wrappit]
pub fn hashh(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x7feb352d;
    let x = (x ^ (x >> 15)) * 0x846ca68b;
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Inverse of hashh.
#[wrappit]
pub fn hashi(x: u32) -> u32 {
    let x = (x ^ (x >> 16)) * 0x43021123;
    let x = (x ^ (x >> 15) ^ (x >> 30)) * 0x1d69e2a5;
    x ^ (x >> 16)
}

/// 32-bit hash by Chris Wellon. Extra high quality.
#[wrappit]
pub fn hashj(x: u32) -> u32 {
    let x = (x ^ (x >> 17)) * 0xed5ad4bb;
    let x = (x ^ (x >> 11)) * 0xac4c1b51;
    let x = (x ^ (x >> 15)) * 0x31848bab;
    x ^ (x >> 14)
}

/// 64-bit hash by Thomas Wang.
#[wrappit]
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
#[wrappit]
pub fn hashm(x: u128) -> u64 {
    const C: u64 = 0x9ddfea08eb382d69;
    let y = (x >> 64) as u64;
    let a = (y ^ x as u64) * C;
    let a = a ^ (a >> 47);
    let a = (a ^ x as u64) * C;
    (a ^ (a >> 47)) * C
}
