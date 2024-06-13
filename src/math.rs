//! Math traits and functions.

use core::cmp::PartialEq;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

/// Number abstraction.
pub trait Num:
    Copy
    + Default
    + Send
    + Sync
    + core::fmt::Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + MulAssign
    + SubAssign
    + DivAssign
    + PartialEq
{
    fn zero() -> Self;
    fn one() -> Self;
    fn new(x: i64) -> Self;
    fn from_f64(x: f64) -> Self;
    fn from_f32(x: f32) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    // Note that in numerical code we do not want to define min() and max() in terms of comparisons.
    // It is inadvisable in general to link traits like this; Min and Max traits would be preferable.
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
        #[inline(always)] fn zero() -> Self { 0 }
        #[inline(always)] fn one() -> Self { 1 }
        #[inline(always)] fn new(x: i64) -> Self { x as Self }
        #[inline(always)] fn from_f64(x: f64) -> Self { x as Self }
        #[inline(always)] fn from_f32(x: f32) -> Self { x as Self }
        #[inline(always)] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline(always)] fn signum(self) -> Self { <$t>::signum(self) }
        #[inline(always)] fn min(self, other: Self) -> Self { core::cmp::min(self, other) }
        #[inline(always)] fn max(self, other: Self) -> Self { core::cmp::max(self, other) }
        #[inline(always)] fn pow(self, other: Self) -> Self { <$t>::pow(self, other as u32) }
        #[inline(always)] fn floor(self) -> Self { self }
        #[inline(always)] fn ceil(self) -> Self { self }
        #[inline(always)] fn round(self) -> Self { self }
    }) *
    }
}
impl_signed_num! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_unsigned_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline(always)] fn zero() -> Self { 0 }
        #[inline(always)] fn one() -> Self { 1 }
        #[inline(always)] fn new(x: i64) -> Self { x as Self }
        #[inline(always)] fn from_f64(x: f64) -> Self { x as Self }
        #[inline(always)] fn from_f32(x: f32) -> Self { x as Self }
        #[inline(always)] fn abs(self) -> Self { self }
        #[inline(always)] fn signum(self) -> Self { 1 }
        #[inline(always)] fn min(self, other: Self) -> Self { core::cmp::min(self, other) }
        #[inline(always)] fn max(self, other: Self) -> Self { core::cmp::max(self, other) }
        #[inline(always)] fn pow(self, other: Self) -> Self { <$t>::pow(self, other as u32) }
        #[inline(always)] fn floor(self) -> Self { self }
        #[inline(always)] fn ceil(self) -> Self { self }
        #[inline(always)] fn round(self) -> Self { self }
    }) *
    }
}
impl_unsigned_num! { u8, u16, u32, u64, u128, usize }

impl Num for f32 {
    #[inline(always)]
    fn zero() -> Self {
        0.0
    }
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn new(x: i64) -> Self {
        x as Self
    }
    #[inline(always)]
    fn from_f64(x: f64) -> Self {
        x as Self
    }
    #[inline(always)]
    fn from_f32(x: f32) -> Self {
        x as Self
    }
    #[inline(always)]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }
    #[inline(always)]
    fn signum(self) -> Self {
        libm::copysignf(1.0, self)
    }
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    #[inline(always)]
    fn pow(self, other: Self) -> Self {
        libm::powf(self, other)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        libm::floorf(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        libm::ceilf(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        libm::roundf(self)
    }
}

impl Num for f64 {
    #[inline(always)]
    fn zero() -> Self {
        0.0
    }
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn new(x: i64) -> Self {
        x as Self
    }
    #[inline(always)]
    fn from_f64(x: f64) -> Self {
        x as Self
    }
    #[inline(always)]
    fn from_f32(x: f32) -> Self {
        x as Self
    }
    #[inline(always)]
    fn abs(self) -> Self {
        libm::fabs(self)
    }
    #[inline(always)]
    fn signum(self) -> Self {
        libm::copysign(1.0, self)
    }
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    #[inline(always)]
    fn pow(self, other: Self) -> Self {
        libm::pow(self, other)
    }
    #[inline(always)]
    fn floor(self) -> Self {
        libm::floor(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self {
        libm::ceil(self)
    }
    #[inline(always)]
    fn round(self) -> Self {
        libm::round(self)
    }
}

/// Integer abstraction.
pub trait Int:
    Num
    + PartialOrd
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
        #[inline(always)] fn wrapping_add(self, other: Self) -> Self { <$t>::wrapping_add(self, other) }
        #[inline(always)] fn wrapping_sub(self, other: Self) -> Self { <$t>::wrapping_sub(self, other) }
        #[inline(always)] fn wrapping_mul(self, other: Self) -> Self { <$t>::wrapping_mul(self, other) }
    }) *
    }
}
impl_int! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize }

/// Float abstraction.
pub trait Float: Num + PartialOrd + Neg<Output = Self> {
    const PI: Self;
    const TAU: Self;
    const SQRT_2: Self;
    fn from_float<T: Float>(x: T) -> Self;
    fn to_f64(self) -> f64;
    fn to_f32(self) -> f32;
    fn to_i64(self) -> i64;
}

impl Float for f32 {
    const PI: Self = core::f32::consts::PI;
    const TAU: Self = core::f32::consts::TAU;
    const SQRT_2: Self = core::f32::consts::SQRT_2;

    #[inline(always)]
    fn from_float<T: Float>(x: T) -> Self {
        x.to_f32()
    }

    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }

    #[inline(always)]
    fn to_f32(self) -> f32 {
        self
    }

    #[inline(always)]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

impl Float for f64 {
    const PI: Self = core::f64::consts::PI;
    const TAU: Self = core::f64::consts::TAU;
    const SQRT_2: Self = core::f64::consts::SQRT_2;

    #[inline(always)]
    fn from_float<T: Float>(x: T) -> Self {
        x.to_f64()
    }

    #[inline(always)]
    fn to_f64(self) -> f64 {
        self
    }

    #[inline(always)]
    fn to_f32(self) -> f32 {
        self as f32
    }

    #[inline(always)]
    fn to_i64(self) -> i64 {
        self as i64
    }
}

/// Generic floating point conversion function.
#[inline(always)]
pub fn convert<T: Float, U: Float>(x: T) -> U {
    U::from_float(x)
}

/// Refined float abstraction.
pub trait Real: Float {
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn log(self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn atan(self) -> Self;
}

impl Real for f32 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        libm::expf(self)
    }
    #[inline(always)]
    fn exp2(self) -> Self {
        libm::exp2f(self)
    }
    #[inline(always)]
    fn log(self) -> Self {
        libm::logf(self)
    }
    #[inline(always)]
    fn log2(self) -> Self {
        libm::log2f(self)
    }
    #[inline(always)]
    fn log10(self) -> Self {
        libm::log10f(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        libm::sinf(self)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cosf(self)
    }
    #[inline(always)]
    fn tan(self) -> Self {
        libm::tanf(self)
    }
    #[inline(always)]
    fn tanh(self) -> Self {
        libm::tanhf(self)
    }
    #[inline(always)]
    fn atan(self) -> Self {
        libm::atanf(self)
    }
}

impl Real for f64 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        libm::exp(self)
    }
    #[inline(always)]
    fn exp2(self) -> Self {
        libm::exp2(self)
    }
    #[inline(always)]
    fn log(self) -> Self {
        libm::log(self)
    }
    #[inline(always)]
    fn log2(self) -> Self {
        libm::log2(self)
    }
    #[inline(always)]
    fn log10(self) -> Self {
        libm::log10(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        libm::sin(self)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cos(self)
    }
    #[inline(always)]
    fn tan(self) -> Self {
        libm::tan(self)
    }
    #[inline(always)]
    fn tanh(self) -> Self {
        libm::tanh(self)
    }
    #[inline(always)]
    fn atan(self) -> Self {
        libm::atan(self)
    }
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
