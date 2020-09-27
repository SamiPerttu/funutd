use std::ops::{Add, Sub, Mul, Div};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use wrapping_arithmetic::wrappit;

pub trait Num: Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn new(x: i64) -> Self;
    fn new_u64(x: u64) -> Self;
    fn new_f64(x: f64) -> Self;
    fn new_f32(x: f32) -> Self;
    fn abs(self) -> Self;
    fn sign(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn pow(self, other: Self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
}

#[inline] pub fn abs<T: Num>(x: T) -> T { x.abs() }
#[inline] pub fn sign<T: Num>(x: T) -> T { x.sign() }
#[inline] pub fn min<T: Num>(x: T, y: T) -> T { x.min(y) }
#[inline] pub fn max<T: Num>(x: T, y: T) -> T { x.max(y) }
#[inline] pub fn pow<T: Num>(x: T, y: T) -> T { x.pow(y) }
#[inline] pub fn floor<T: Num>(x: T) -> T { x.floor() }
#[inline] pub fn ceil<T: Num>(x: T) -> T { x.ceil() }
#[inline] pub fn round<T: Num>(x: T) -> T { x.round() }

macro_rules! impl_signed_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { 0 }
        #[inline] fn one() -> Self { 1 }
        #[inline] fn new(x: i64) -> Self { x as Self }
        #[inline] fn new_u64(x: u64) -> Self { x as Self }
        #[inline] fn new_f64(x: f64) -> Self { x as Self }
        #[inline] fn new_f32(x: f32) -> Self { x as Self }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn sign(self) -> Self { self.signum() }
        #[inline] fn min(self, other: Self) -> Self { std::cmp::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { std::cmp::max(self, other) }
        #[inline] fn pow(self, other: Self) -> Self { <$t>::pow(self, other as u32) }
        #[inline] fn floor(self) -> Self { self }
        #[inline] fn ceil(self) -> Self { self }
        #[inline] fn round(self) -> Self { self }
    }) *
    }
}
impl_signed_num! { i8, i16, i32, i64, i128 }

macro_rules! impl_unsigned_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { 0 }
        #[inline] fn one() -> Self { 1 }
        #[inline] fn new(x: i64) -> Self { x as Self }
        #[inline] fn new_u64(x: u64) -> Self { x as Self }
        #[inline] fn new_f64(x: f64) -> Self { x as Self }
        #[inline] fn new_f32(x: f32) -> Self { x as Self }
        #[inline] fn abs(self) -> Self { self }
        #[inline] fn sign(self) -> Self { 1 }
        #[inline] fn min(self, other: Self) -> Self { std::cmp::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { std::cmp::max(self, other) }
        #[inline] fn pow(self, other: Self) -> Self { <$t>::pow(self, other as u32) }
        #[inline] fn floor(self) -> Self { self }
        #[inline] fn ceil(self) -> Self { self }
        #[inline] fn round(self) -> Self { self }
    }) *
    }
}
impl_unsigned_num! { u8, u16, u32, u64, u128 }

macro_rules! impl_float_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { 0.0 }
        #[inline] fn one() -> Self { 1.0 }
        #[inline] fn new(x: i64) -> Self { x as Self }
        #[inline] fn new_u64(x: u64) -> Self { x as Self }
        #[inline] fn new_f64(x: f64) -> Self { x as Self }
        #[inline] fn new_f32(x: f32) -> Self { x as Self }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn sign(self) -> Self { self.signum() }
        #[inline] fn min(self, other: Self) -> Self { <$t>::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { <$t>::max(self, other) }
        #[inline] fn pow(self, other: Self) -> Self { <$t>::powf(self, other) }
        #[inline] fn floor(self) -> Self { <$t>::floor(self) }
        #[inline] fn ceil(self) -> Self { <$t>::ceil(self) }
        #[inline] fn round(self) -> Self { <$t>::round(self) }
    }) *
    }
}
impl_float_num! { f32, f64 }

pub trait Int: Num
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn wrapping_mul(self, other: Self) -> Self;
}

macro_rules! impl_int {
    ( $($t:ty),* ) => {
    $( impl Int for $t {
        #[inline] fn wrapping_add(self, other: Self) -> Self { <$t>::wrapping_add(self, other) }
        #[inline] fn wrapping_sub(self, other: Self) -> Self { <$t>::wrapping_sub(self, other) }
        #[inline] fn wrapping_mul(self, other: Self) -> Self { <$t>::wrapping_mul(self, other) }
    }) *
    }
}
impl_int! { i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 }

pub trait Real {
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn log(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

#[inline] pub fn sqrt<T: Real>(x: T) -> T { x.sqrt() }
#[inline] pub fn exp<T: Real>(x: T) -> T { x.exp() }
#[inline] pub fn log<T: Real>(x: T) -> T { x.log() }
#[inline] pub fn sin<T: Real>(x: T) -> T { x.sin() }
#[inline] pub fn cos<T: Real>(x: T) -> T { x.cos() }
#[inline] pub fn tan<T: Real>(x: T) -> T { x.tan() }

macro_rules! impl_real {
    ( $($t:ty),* ) => {
    $( impl Real for $t {
        #[inline] fn sqrt(self) -> Self { self.sqrt() }    
        #[inline] fn exp(self) -> Self { self.exp() }
        #[inline] fn log(self) -> Self { self.ln() }
        #[inline] fn sin(self) -> Self { self.sin() }
        #[inline] fn cos(self) -> Self { self.cos() }
        #[inline] fn tan(self) -> Self { <$t>::tan(self) }
    }) *
    }
}
impl_real! { f32, f64 }

pub trait AsPrimitive<T: Copy>: Copy
{
    /// Convert a value using the as operator.
    fn as_(self) -> T;
}

macro_rules! impl_as_primitive {
    (@ $T: ty => $(#[$cfg:meta])* impl $U: ty ) => {
        $(#[$cfg])*
        impl AsPrimitive<$U> for $T {
            #[inline] fn as_(self) -> $U { self as $U }
        }
    };
    (@ $T: ty => { $( $U: ty ),* } ) => {$(
        impl_as_primitive!(@ $T => impl $U);
    )*};
    ($T: ty => { $( $U: ty ),* } ) => {
        impl_as_primitive!(@ $T => { $( $U ),* });
        impl_as_primitive!(@ $T => { u8, u16, u32, u64, u128, usize });
        impl_as_primitive!(@ $T => { i8, i16, i32, i64, i128, isize });
    };
}

impl_as_primitive!(u8 => { char, f32, f64 });
impl_as_primitive!(i8 => { f32, f64 });
impl_as_primitive!(u16 => { f32, f64 });
impl_as_primitive!(i16 => { f32, f64 });
impl_as_primitive!(u32 => { f32, f64 });
impl_as_primitive!(i32 => { f32, f64 });
impl_as_primitive!(u64 => { f32, f64 });
impl_as_primitive!(i64 => { f32, f64 });
impl_as_primitive!(u128 => { f32, f64 });
impl_as_primitive!(i128 => { f32, f64 });
impl_as_primitive!(usize => { f32, f64 });
impl_as_primitive!(isize => { f32, f64 });
impl_as_primitive!(f32 => { f32, f64 });
impl_as_primitive!(f64 => { f32, f64 });
impl_as_primitive!(char => { char });
impl_as_primitive!(bool => {});

/// Cast between primitive types.
#[inline] pub fn cast<T: AsPrimitive<U>, U: Copy>(t: T) -> U { t.as_() }

/// sqrt(2)
pub const SQRT_2: f64 = std::f64::consts::SQRT_2;
/// e (Euler's constant)
pub const E: f64 = std::f64::consts::E;
/// pi
pub const PI: f64 = std::f64::consts::PI;
/// tau = 2 * pi
pub const TAU: f64 = std::f64::consts::TAU;

/// Minimum of 3 items.
#[inline] pub fn min3<T: Num>(x: T, y: T, z: T) -> T { x.min(y).min(z) }

/// Maximum of 3 items.
#[inline] pub fn max3<T: Num>(x: T, y: T, z: T) -> T { x.max(y).max(z) }

/// Clamps x between x0 and x1.
#[inline] pub fn clamp<T: Num>(x0: T, x1: T, x: T) -> T { x.max(x0).min(x1) }

/// Clamps x between 0 and 1.
#[inline] pub fn clamp01<T: Num>(x: T) -> T { x.max(T::zero()).min(T::one()) }

/// Clamps x between -1 and 1.
#[inline] pub fn clamp11<T: Num>(x: T) -> T { x.max(T::new(-1)).min(T::one()) }

pub trait Lerp<T> {
    fn lerp(self, other: Self, t: T) -> Self;
}

impl<U, T> Lerp<T> for U where U: Add<Output = U> + Mul<T, Output = U>, T: Num {
    #[inline] fn lerp(self, other: U, t: T) -> U {
        self * (T::one() - t) + other * t
    }
}

#[inline] pub fn lerp<U: Lerp<T>, T>(a: U, b: U, t: T) -> U {
    a.lerp(b, t)
}

#[inline] pub fn delerp<T: Num>(a: T, b: T, x: T) -> T {
    (x - a) / (b - a)
}

#[inline] pub fn xerp<U: Lerp<T> + Real, T>(a: U, b: U, t: T) -> U {
    exp(lerp(log(a), log(b), t))
}

#[inline] pub fn dexerp<T: Num + Real>(a: T, b: T, x: T) -> T {
    log(x / a) / log(b / a)
}

/// Rounds to a multiple of step.
#[inline] pub fn discretize<T: Num>(x: T, step: T) -> T {
    (x / step).round() * step
}

/// Square of x.
#[inline] pub fn squared<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

/// Cube of x.
#[inline] pub fn cubed<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x * x
}

/// Smooth cubic fade curve.
#[inline] pub fn smooth3<T: Num>(x: T) -> T {
    (T::new(3) - T::new(2) * x) * x * x
}

/// Smooth quintic fade curve suggested by Ken Perlin.
#[inline] pub fn smooth5<T: Num>(x: T) -> T {
    ((x * T::new(6) - T::new(15)) * x + T::new(10)) * x * x * x
}

/// Smooth septic fade curve.
#[inline] pub fn smooth7<T: Num>(x: T) -> T {
    let x2 = x * x;
    x2 * x2 * (T::new(35) - T::new(84) * x + (T::new(70) - T::new(20) * x) * x2)
}

/// Smooth nonic fade curve.
#[inline] pub fn smooth9<T: Num>(x: T) -> T {
    let x2 = x * x;
    ((((T::new(70) * x - T::new(315)) * x + T::new(540)) * x - T::new(420)) * x + T::new(125)) * x2 * x2 * x
}

/// Fade that starts and ends at a slope but levels in the middle.
#[inline] pub fn shelf<T: Num>(x: T) -> T {
    ((T::new(4) * x - T::new(6)) * x + T::new(3)) * x
}

/// A quarter circle fade that slopes upwards. Inverse function of Fade.downarc.
#[inline] pub fn uparc<T: Real + Num>(x: T) -> T {
    T::one() - sqrt(max(T::zero(), T::one() - x * x))
}

/// A quarter circle fade that slopes downwards. Inverse function of Fade.uparc.
#[inline] pub fn downarc<T: Real + Num>(x: T) -> T {
    sqrt(max(T::new(0), (T::new(2) - x) * x))
}

/// Wave function stitched together from two symmetric pieces peaking at origin.
#[inline] pub fn wave<T: Num, F: Fn(T) -> T>(f: F, x: T) -> T {
    let u = (x - T::one()) / T::new(4);
    let u = (u - u.floor()) * T::new(2);
    let w0 = u.min(T::one());
    let w1 = u - w0;
    T::one() - (f(w0) - f(w1)) * T::new(2)
}

#[inline] pub fn wave3<T: Num>(x: T) -> T { wave(smooth3, x) }
#[inline] pub fn wave5<T: Num>(x: T) -> T { wave(smooth5, x) }

/// Catmull-Rom cubic spline interpolation, which is a form of cubic Hermite spline. Interpolates between
/// y1 (returns y1 when x = 0) and y2 (returns y2 when x = 1) while using the previous (y0) and next (y3)
/// points to define slopes at the endpoints. The maximum overshoot is 1/8th of the range of the arguments.
#[inline] pub fn spline<T: Num>(y0: T, y1: T, y2: T, y3: T, x: T) -> T {
    y1 + x / T::new(2) * (y2 - y0 + x * (T::new(2) * y0 - T::new(5) * y1 + T::new(4) * y2 - y3 + x * (T::new(3) * (y1 - y2) + y3 - y0)))
}

/// Monotonic cubic interpolation via Steffen's method. The result never overshoots.
/// It is first order continuous. Interpolates between y1 (at x = 0) and y2 (at x = 1)
/// while using the previous (y0) and next (y3) values to influence slopes.
pub fn spline_mono<T: Num>(y0: T, y1: T, y2: T, y3: T, x: T) -> T {
  let d0 = y1 - y0;
  let d1 = y2 - y1;
  let d2 = y3 - y2;
  let d1d = (sign(d0) + sign(d1)) * min3(d0 + d1, abs(d0), abs(d1));
  let d2d = (sign(d1) + sign(d2)) * min3(d1 + d2, abs(d1), abs(d2));
  cubed(x) * (T::new(2) * y1 - T::new(2) * y2 + d1d + d2d) + squared(x) * (T::new(-3) * y1 + T::new(3) * y2 - T::new(2) * d1d - d2d) + x * d1d + y1
}

/// Logistic sigmoid.
#[inline] pub fn logistic<T: Num + Real>(x: T) -> T {
    T::one() / (T::one() + exp(T::zero() - x))
}

/// Derivative of the logistic sigmoid.
#[inline] pub fn logistic_d<T: Num + Real>(x: T) -> T {
    let y = logistic(x);
    y * (T::one() - y)
}

/// Softsign function.
#[inline] pub fn softsign<T: Num>(x: T) -> T {
    x / (T::one() + x.abs())
}

/// Derivative of the softsign function.
#[inline] pub fn softsign_d<T: Num>(x: T) -> T {
    T::one() / squared(T::one() + x.abs())
}

/// This exp-like response function is second order continuous.
/// It has asymmetrical magnitude curves: (inverse) linear when x < 0 and quadratic when x > 0.
/// f(x) >= 0 for all x. Like the exponential function, f(0) = f'(0) = 1.
#[inline] pub fn exq<T: Num>(x: T) -> T {
    // With a branch:
    // if x > 0 { x * x + x + 1 } else { 1 / (1 - x) }
    let p = x.max(T::zero());
    p * p + p + T::one() / (T::one() + p - x)
}

// Softmin function when amount < 0, softmax when amount > 0, and average when amount = 0.
#[inline] pub fn softmix<T: Num>(amount: T, x: T, y: T) -> T {
    let xw = exq(x * amount);
    let yw = exq(y * amount);
    (x * xw + y * yw) / (xw + yw + T::new_f32(1.0e-10))
}

/// Linear congruential generator from Numerical Recipes. Cycles through all u32 values.
#[inline] pub fn lcg32(x: u32) -> u32 { x * 1664525 + 1013904223 }

/// Linear congruential generator by Donald Knuth. Cycles through all u64 values.
#[inline] pub fn lcg64(x: u64) -> u64 { x * 6364136223846793005 + 1442695040888963407 }

/// Encodes to binary reflected Gray code.
#[inline] pub fn gray<T: Int>(x: T) -> T {
    x ^ (x >> 1)
}

/// Decodes from binary reflected Gray code.
#[inline] pub fn degray<T: Int>(x: T) -> T {
    let x = x ^ (x >> 64);
    let x = x ^ (x >> 32);
    let x = x ^ (x >> 16);
    let x = x ^ (x >> 8);
    let x = x ^ (x >> 4);
    let x = x ^ (x >> 2);
    x ^ (x >> 1)
}

/// Quadratic probe. This is a bijective function in unsigned types.
#[wrappit] #[inline] pub fn quadp<T: Int>(x: T) -> T {
    let q = x >> 1;
    (x & T::one()) + q + q * q
}

/// Sum of an arithmetic series with n terms: sum over i in [0, n[ of a0 + step * i.
#[inline] pub fn arithmetic_sum<T: Num>(n: T, a0: T, step: T) -> T {
    n * (T::new(2) * a0 + step * (n - T::one())) / T::new(2)
}

/// Sum of a geometric series with n terms: sum over i in [0, n[ of a0 * ratio ** i.
#[inline] pub fn geometric_sum<T: Num + PartialOrd>(n: T, a0: T, ratio: T) -> T {
    if ratio != T::one() {
        a0 * (T::one() - ratio.pow(n)) / (T::one() - ratio)
    } else {
        a0 * n
    }
}

/*
/// Cubic Lagrange interpolation with x in range [0, 3] and the result intersecting all y.
let lagrange y0 y1 y2 y3 (x : float) =
  match x with
  | 0.0 -> y0
  | 1.0 -> y1
  | 2.0 -> y2
  | 3.0 -> y3
  | x ->
    let x = x * 2G/3G - 1G
    let d0 = 1G / (x + 1G)
    let d1 = -3G / (x + 1G/3G)
    let d2 = 3G / (x - 1G/3G)
    let d3 = -1G / (x - 1G)
    (y0 * d0 + y1 * d1 + y2 * d2 + y3 * d3) / (d0 + d1 + d2 + d3)
*/

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

/// 64-bit hash from SplitMix64 by Sebastiano Vigna.
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

/// 64-bit hash SplitMix64 by Sebastiano Vigna.
/// Extra high quality. Passes PractRand as an indexed RNG.
#[wrappit] #[inline] 
pub fn hashr(x: u64) -> u64 {
    let x = x * 0x9e3779b97f4a7c15;
    let x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
    let x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
    x ^ (x >> 31)
}
