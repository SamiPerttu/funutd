use crate::prelude::*;

pub use glam::*;

macro_rules! impl_vec_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { <$t>::zero() }
        #[inline] fn one() -> Self { <$t>::one() }
        #[inline] fn new(x: i64) -> Self { <$t>::splat(x as f32) }
        #[inline] fn new_u64(x: u64) -> Self { <$t>::splat(x as f32) }
        #[inline] fn new_f64(x: f64) -> Self { <$t>::splat(x as f32) }
        #[inline] fn new_f32(x: f32) -> Self { <$t>::splat(x) }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn sign(self) -> Self { <$t>::sign(self) }
        #[inline] fn min(self, other: Self) -> Self { <$t>::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { <$t>::max(self, other) }
        #[inline] fn pow(self, _other: Self) -> Self { panic!() }
        #[inline] fn floor(self) -> Self { <$t>::floor(self) }
        #[inline] fn ceil(self) -> Self { <$t>::ceil(self) }
        #[inline] fn round(self) -> Self { <$t>::round(self) }
    }) *
    }
}
impl_vec_num! { Vec2, Vec3, Vec3A, Vec4 }
