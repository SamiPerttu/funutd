use super::math::*;

macro_rules! impl_vec32_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { <$t>::ZERO }
        #[inline] fn one() -> Self { <$t>::ONE }
        #[inline] fn new(x: i64) -> Self { <$t>::splat(x as f32) }
        #[inline] fn from_u64(x: u64) -> Self { <$t>::splat(x as f32) }
        #[inline] fn from_f64(x: f64) -> Self { <$t>::splat(x as f32) }
        #[inline] fn from_f32(x: f32) -> Self { <$t>::splat(x) }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn signum(self) -> Self { <$t>::signum(self) }
        #[inline] fn min(self, other: Self) -> Self { <$t>::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { <$t>::max(self, other) }
        #[inline] fn pow(self, _other: Self) -> Self { panic!() }
        #[inline] fn floor(self) -> Self { <$t>::floor(self) }
        #[inline] fn ceil(self) -> Self { <$t>::ceil(self) }
        #[inline] fn round(self) -> Self { <$t>::round(self) }
    }) *
    }
}
impl_vec32_num! { glam::Vec2, glam::Vec3, glam::Vec3A, glam::Vec4 }

macro_rules! impl_vec64_num {
    ( $($t:ty),* ) => {
    $( impl Num for $t {
        #[inline] fn zero() -> Self { <$t>::ZERO }
        #[inline] fn one() -> Self { <$t>::ONE }
        #[inline] fn new(x: i64) -> Self { <$t>::splat(x as f64) }
        #[inline] fn from_u64(x: u64) -> Self { <$t>::splat(x as f64) }
        #[inline] fn from_f64(x: f64) -> Self { <$t>::splat(x) }
        #[inline] fn from_f32(x: f32) -> Self { <$t>::splat(x as f64) }
        #[inline] fn abs(self) -> Self { <$t>::abs(self) }
        #[inline] fn signum(self) -> Self { <$t>::signum(self) }
        #[inline] fn min(self, other: Self) -> Self { <$t>::min(self, other) }
        #[inline] fn max(self, other: Self) -> Self { <$t>::max(self, other) }
        #[inline] fn pow(self, _other: Self) -> Self { panic!() }
        #[inline] fn floor(self) -> Self { <$t>::floor(self) }
        #[inline] fn ceil(self) -> Self { <$t>::ceil(self) }
        #[inline] fn round(self) -> Self { <$t>::round(self) }
    }) *
    }
}
impl_vec64_num! { glam::DVec2, glam::DVec3, glam::DVec4 }

pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;
pub type Vec3a = glam::Vec3A;
pub type Vec4 = glam::Vec4;

pub type Vec2d = glam::DVec2;
pub type Vec3d = glam::DVec3;
pub type Vec4d = glam::DVec4;

pub type Int2 = glam::IVec2;
pub type Int3 = glam::IVec3;
pub type Int4 = glam::IVec4;

/*
pub struct Vec2(glam::Vec2);
pub struct Vec3(glam::Vec3);
pub struct Vec3a(glam::Vec3A);
pub struct Vec4(glam::Vec4);

pub struct Vec2d(glam::DVec2);
pub struct Vec3d(glam::DVec3);
pub struct Vec4d(glam::DVec4);

pub struct Int2(glam::IVec2);
pub struct Int3(glam::IVec3);
pub struct Int4(glam::IVec4);
*/
