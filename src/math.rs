//! Math traits and functions.

use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

pub trait Num:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn new(x: i64) -> Self;
    fn from_u64(x: u64) -> Self;
    fn from_f64(x: f64) -> Self;
    fn from_f32(x: f32) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn pow(self, other: Self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
}

#[inline]
pub fn abs<T: Num>(x: T) -> T {
    x.abs()
}
#[inline]
pub fn signum<T: Num>(x: T) -> T {
    x.signum()
}
#[inline]
pub fn min<T: Num>(x: T, y: T) -> T {
    x.min(y)
}
#[inline]
pub fn max<T: Num>(x: T, y: T) -> T {
    x.max(y)
}
#[inline]
pub fn pow<T: Num>(x: T, power: T) -> T {
    x.pow(power)
}
#[inline]
pub fn floor<T: Num>(x: T) -> T {
    x.floor()
}
#[inline]
pub fn ceil<T: Num>(x: T) -> T {
    x.ceil()
}
#[inline]
pub fn round<T: Num>(x: T) -> T {
    x.round()
}

macro_rules! impl_signed_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { 0 }
        #[inline] fn one() -> Self { 1 }
        #[inline] fn new(x: i64) -> Self { x as Self }
        #[inline] fn from_u64(x: u64) -> Self { x as Self }
        #[inline] fn from_f64(x: f64) -> Self { x as Self }
        #[inline] fn from_f32(x: f32) -> Self { x as Self }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn signum(self) -> Self { self.signum() }
        #[inline] fn min(self, other: Self) -> Self { std::cmp::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { std::cmp::max(self, other) }
        #[inline] fn pow(self, other: Self) -> Self { <$t>::pow(self, other as u32) }
        #[inline] fn floor(self) -> Self { self }
        #[inline] fn ceil(self) -> Self { self }
        #[inline] fn round(self) -> Self { self }
    }) *
    }
}
impl_signed_num! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_unsigned_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { 0 }
        #[inline] fn one() -> Self { 1 }
        #[inline] fn new(x: i64) -> Self { x as Self }
        #[inline] fn from_u64(x: u64) -> Self { x as Self }
        #[inline] fn from_f64(x: f64) -> Self { x as Self }
        #[inline] fn from_f32(x: f32) -> Self { x as Self }
        #[inline] fn abs(self) -> Self { self }
        #[inline] fn signum(self) -> Self { 1 }
        #[inline] fn min(self, other: Self) -> Self { std::cmp::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { std::cmp::max(self, other) }
        #[inline] fn pow(self, other: Self) -> Self { <$t>::pow(self, other as u32) }
        #[inline] fn floor(self) -> Self { self }
        #[inline] fn ceil(self) -> Self { self }
        #[inline] fn round(self) -> Self { self }
    }) *
    }
}
impl_unsigned_num! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_float_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { 0.0 }
        #[inline] fn one() -> Self { 1.0 }
        #[inline] fn new(x: i64) -> Self { x as Self }
        #[inline] fn from_u64(x: u64) -> Self { x as Self }
        #[inline] fn from_f64(x: f64) -> Self { x as Self }
        #[inline] fn from_f32(x: f32) -> Self { x as Self }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn signum(self) -> Self { self.signum() }
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

pub trait Int:
    Num
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + PartialOrd
    + Ord
    + PartialEq
    + Eq
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
impl_int! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize }

pub trait Real:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
{
    const PI: Self;
    const TAU: Self;
    const SQRT_2: Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn log(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

#[inline]
pub fn sqrt<T: Real>(x: T) -> T {
    x.sqrt()
}
#[inline]
pub fn exp<T: Real>(x: T) -> T {
    x.exp()
}
#[inline]
pub fn log<T: Real>(x: T) -> T {
    x.log()
}
#[inline]
pub fn sin<T: Real>(x: T) -> T {
    x.sin()
}
#[inline]
pub fn cos<T: Real>(x: T) -> T {
    x.cos()
}
#[inline]
pub fn tan<T: Real>(x: T) -> T {
    x.tan()
}

macro_rules! impl_real {
    ( $($t:ty),* ) => {
    $( impl Real for $t {
        #[allow(clippy::excessive_precision)]
        const PI: Self = 3.14159265358979323846;
        #[allow(clippy::excessive_precision)]
        const TAU: Self = 6.28318530717958647692;
        #[allow(clippy::excessive_precision)]
        const SQRT_2: Self = 1.4142135623730950488;
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

/// Clamps x between x0 and x1.
#[inline]
pub fn clamp<T: Num>(x0: T, x1: T, x: T) -> T {
    x.max(x0).min(x1)
}

/// Clamps x between 0 and 1.
#[inline]
pub fn clamp01<T: Num>(x: T) -> T {
    x.max(T::zero()).min(T::one())
}

/// Clamps x between -1 and 1.
#[inline]
pub fn clamp11<T: Num>(x: T) -> T {
    x.max(T::new(-1)).min(T::one())
}

pub trait Lerp<T> {
    fn lerp(self, other: Self, t: T) -> Self;
}

impl<U, T> Lerp<T> for U
where
    U: Add<Output = U> + Mul<T, Output = U>,
    T: Num,
{
    #[inline]
    fn lerp(self, other: U, t: T) -> U {
        self * (T::one() - t) + other * t
    }
}

#[inline]
pub fn lerp<U: Lerp<T>, T>(a: U, b: U, t: T) -> U {
    a.lerp(b, t)
}

#[inline]
pub fn delerp<T: Num>(a: T, b: T, x: T) -> T {
    (x - a) / (b - a)
}

#[inline]
pub fn xerp<U: Lerp<T> + Real, T>(a: U, b: U, t: T) -> U {
    exp(lerp(log(a), log(b), t))
}

#[inline]
pub fn dexerp<T: Num + Real>(a: T, b: T, x: T) -> T {
    log(x / a) / log(b / a)
}

/// Square of x.
#[inline]
pub fn squared<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

/// Cube of x.
#[inline]
pub fn cubed<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x * x
}

/// Smooth cubic fade curve.
#[inline]
pub fn smooth3<T: Num>(x: T) -> T {
    (T::new(3) - T::new(2) * x) * x * x
}

/// Smooth quintic fade curve suggested by Ken Perlin.
#[inline]
pub fn smooth5<T: Num>(x: T) -> T {
    ((x * T::new(6) - T::new(15)) * x + T::new(10)) * x * x * x
}

/// Smooth septic fade curve.
#[inline]
pub fn smooth7<T: Num>(x: T) -> T {
    let x2 = x * x;
    x2 * x2 * (T::new(35) - T::new(84) * x + (T::new(70) - T::new(20) * x) * x2)
}

/// Smooth nonic fade curve.
#[inline]
pub fn smooth9<T: Num>(x: T) -> T {
    let x2 = x * x;
    ((((T::new(70) * x - T::new(315)) * x + T::new(540)) * x - T::new(420)) * x + T::new(126))
        * x2
        * x2
        * x
}

/// A quarter circle fade that slopes upwards. Inverse function of Fade.downarc.
#[inline]
pub fn uparc<T: Real + Num>(x: T) -> T {
    T::one() - sqrt(max(T::zero(), T::one() - x * x))
}

/// A quarter circle fade that slopes downwards. Inverse function of Fade.uparc.
#[inline]
pub fn downarc<T: Real + Num>(x: T) -> T {
    sqrt(max(T::new(0), (T::new(2) - x) * x))
}

/// Wave function stitched together from two symmetric pieces peaking at origin.
#[inline]
pub fn wave<T: Num, F: Fn(T) -> T>(f: F, x: T) -> T {
    let u = (x - T::one()) / T::new(4);
    let u = (u - u.floor()) * T::new(2);
    let w0 = u.min(T::one());
    let w1 = u - w0;
    T::one() - (f(w0) - f(w1)) * T::new(2)
}

/// Catmull-Rom cubic spline interpolation, which is a form of cubic Hermite spline. Interpolates between
/// y1 (returns y1 when x = 0) and y2 (returns y2 when x = 1) while using the previous (y0) and next (y3)
/// points to define slopes at the endpoints. The maximum overshoot is 1/8th of the range of the arguments.
#[inline]
pub fn spline<T: Num>(y0: T, y1: T, y2: T, y3: T, x: T) -> T {
    y1 + x / T::new(2)
        * (y2 - y0
            + x * (T::new(2) * y0 - T::new(5) * y1 + T::new(4) * y2 - y3
                + x * (T::new(3) * (y1 - y2) + y3 - y0)))
}

/// Monotonic cubic interpolation via Steffen's method. The result never overshoots.
/// It is first order continuous. Interpolates between y1 (at x = 0) and y2 (at x = 1)
/// while using the previous (y0) and next (y3) values to influence slopes.
pub fn spline_mono<T: Num>(y0: T, y1: T, y2: T, y3: T, x: T) -> T {
    let d0 = y1 - y0;
    let d1 = y2 - y1;
    let d2 = y3 - y2;
    let d1d = (signum(d0) + signum(d1)) * min(d0 + d1, min(abs(d0), abs(d1)));
    let d2d = (signum(d1) + signum(d2)) * min(d1 + d2, min(abs(d1), abs(d2)));
    cubed(x) * (T::new(2) * y1 - T::new(2) * y2 + d1d + d2d)
        + squared(x) * (T::new(-3) * y1 + T::new(3) * y2 - T::new(2) * d1d - d2d)
        + x * d1d
        + y1
}

/// Logistic sigmoid.
#[inline]
pub fn logistic<T: Num + Real>(x: T) -> T {
    T::one() / (T::one() + exp(T::zero() - x))
}

/// Derivative of the logistic sigmoid.
#[inline]
pub fn logistic_d<T: Num + Real>(x: T) -> T {
    let y = logistic(x);
    y * (T::one() - y)
}

/// Softsign function.
#[inline]
pub fn softsign<T: Num>(x: T) -> T {
    x / (T::one() + x.abs())
}

/// Derivative of the softsign function.
#[inline]
pub fn softsign_d<T: Num>(x: T) -> T {
    T::one() / squared(T::one() + x.abs())
}

/// This exp-like response function is second order continuous.
/// It has asymmetrical magnitude curves: (inverse) linear when x < 0 and quadratic when x > 0.
/// softexp(x) >= 0 for all x.
/// Like the exponential function, softexp(0) = softexp'(0) = 1.
#[inline]
pub fn softexp<T: Num>(x: T) -> T {
    // With a branch:
    // if x > 0 { x * x + x + 1 } else { 1 / (1 - x) }
    let p = max(x, T::zero());
    p * p + p + T::one() / (T::one() + p - x)
}

// Softmin function when amount < 0, softmax when amount > 0, and average when amount = 0.
#[inline]
pub fn softmix<T: Num>(amount: T, x: T, y: T) -> T {
    let xw = softexp(x * amount);
    let yw = softexp(y * amount);
    (x * xw + y * yw) / (xw + yw + T::from_f32(1.0e-10))
}

/// Encodes to binary reflected Gray code.
#[inline]
pub fn gray<T: Int>(x: T) -> T {
    x ^ (x >> 1)
}

/// Decodes from binary reflected Gray code.
#[inline]
pub fn degray<T: Int>(x: T) -> T {
    let x = x ^ (x >> 64);
    let x = x ^ (x >> 32);
    let x = x ^ (x >> 16);
    let x = x ^ (x >> 8);
    let x = x ^ (x >> 4);
    let x = x ^ (x >> 2);
    x ^ (x >> 1)
}

/// Sum of an arithmetic series with n terms: sum over i in [0, n[ of a0 + step * i.
#[inline]
pub fn arithmetic_sum<T: Num>(n: T, a0: T, step: T) -> T {
    n * (T::new(2) * a0 + step * (n - T::one())) / T::new(2)
}

/// Sum of a geometric series with n terms: sum over i in [0, n[ of a0 * ratio ** i.
#[inline]
pub fn geometric_sum<T: Num + PartialOrd>(n: T, a0: T, ratio: T) -> T {
    let denom = T::one() - ratio;
    if denom != T::zero() {
        a0 * (T::one() - ratio.pow(n)) / denom
    } else {
        a0 * n
    }
}
