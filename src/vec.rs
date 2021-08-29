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

pub struct Vec2(glam::Vec2);
pub struct Vec3(glam::Vec3);
pub struct Vec3a(glam::Vec3A);
pub struct Vec4(glam::Vec4);

impl core::ops::Deref for Vec2 {
    type Target = glam::Vec2;
    fn deref(&self) -> &Self::Target {
        let Vec2(v) = self;
        v
    }
}

impl core::ops::Deref for Vec3 {
    type Target = glam::Vec3;
    fn deref(&self) -> &Self::Target {
        let Vec3(v) = self;
        v
    }
}

impl core::ops::Deref for Vec3a {
    type Target = glam::Vec3A;
    fn deref(&self) -> &Self::Target {
        let Vec3a(v) = self;
        v
    }
}

impl core::ops::Deref for Vec4 {
    type Target = glam::Vec4;
    fn deref(&self) -> &Self::Target {
        let Vec4(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec2(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec3(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec3a {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec3a(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec4(v) = self;
        v
    }
}

pub struct Vec2d(glam::DVec2);
pub struct Vec3d(glam::DVec3);
pub struct Vec4d(glam::DVec4);

impl core::ops::Deref for Vec2d {
    type Target = glam::DVec2;
    fn deref(&self) -> &Self::Target {
        let Vec2d(v) = self;
        v
    }
}

impl core::ops::Deref for Vec3d {
    type Target = glam::DVec3;
    fn deref(&self) -> &Self::Target {
        let Vec3d(v) = self;
        v
    }
}

impl core::ops::Deref for Vec4d {
    type Target = glam::DVec4;
    fn deref(&self) -> &Self::Target {
        let Vec4d(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec2d(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec3d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec3d(v) = self;
        v
    }
}

impl core::ops::DerefMut for Vec4d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Vec4d(v) = self;
        v
    }
}

pub struct Int2(glam::IVec2);
pub struct Int3(glam::IVec3);
pub struct Int4(glam::IVec4);

impl core::ops::Deref for Int2 {
    type Target = glam::IVec2;
    fn deref(&self) -> &Self::Target {
        let Int2(v) = self;
        v
    }
}

impl core::ops::Deref for Int3 {
    type Target = glam::IVec3;
    fn deref(&self) -> &Self::Target {
        let Int3(v) = self;
        v
    }
}

impl core::ops::Deref for Int4 {
    type Target = glam::IVec4;
    fn deref(&self) -> &Self::Target {
        let Int4(v) = self;
        v
    }
}

impl core::ops::DerefMut for Int2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Int2(v) = self;
        v
    }
}

impl core::ops::DerefMut for Int3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Int3(v) = self;
        v
    }
}

impl core::ops::DerefMut for Int4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Int4(v) = self;
        v
    }
}

