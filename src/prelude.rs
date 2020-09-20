use num_traits::real::Real;
use num_traits::AsPrimitive;

pub fn tan<F: Real>(x : F) -> F { x.tan() }
pub fn exp<F: Real>(x : F) -> F { x.exp() }
pub fn sin<F: Real>(x : F) -> F { x.sin() }
pub fn cos<F: Real>(x : F) -> F { x.cos() }
pub fn sqrt<F: Real>(x : F) -> F { x.sqrt() }

pub use std::cmp::max;
pub use std::cmp::min;
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
