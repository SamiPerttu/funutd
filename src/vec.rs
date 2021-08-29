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

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}
#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}
#[inline]
pub fn vec3a(x: f32, y: f32, z: f32) -> Vec3a {
    Vec3a::new(x, y, z)
}
#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

#[inline]
pub fn vec2d(x: f64, y: f64) -> Vec2d {
    Vec2d::new(x, y)
}
#[inline]
pub fn vec3d(x: f64, y: f64, z: f64) -> Vec3d {
    Vec3d::new(x, y, z)
}
#[inline]
pub fn vec4d(x: f64, y: f64, z: f64, w: f64) -> Vec4d {
    Vec4d::new(x, y, z, w)
}

#[inline]
pub fn int2(x: i32, y: i32) -> Int2 {
    Int2::new(x, y)
}
#[inline]
pub fn int3(x: i32, y: i32, z: i32) -> Int3 {
    Int3::new(x, y, z)
}
#[inline]
pub fn int4(x: i32, y: i32, z: i32, w: i32) -> Int4 {
    Int4::new(x, y, z, w)
}

pub trait Vec2Ext {
    type Scalar;
    fn from_angle(radians: Self::Scalar) -> Self;
    fn rotate_90(self) -> Self;
    fn rotate_270(self) -> Self;
}

impl Vec2Ext for Vec2 {
    type Scalar = f32;
    #[inline]
    fn from_angle(radians: f32) -> Vec2 {
        vec2(cos(radians), sin(radians))
    }
    #[inline]
    fn rotate_90(self) -> Self {
        vec2(-self.y, self.x)
    }
    #[inline]
    fn rotate_270(self) -> Self {
        vec2(self.y, -self.x)
    }
}

impl Vec2Ext for Vec2d {
    type Scalar = f64;
    #[inline]
    fn from_angle(radians: f64) -> Vec2d {
        vec2d(cos(radians), sin(radians))
    }
    #[inline]
    fn rotate_90(self) -> Self {
        vec2d(-self.y, self.x)
    }
    #[inline]
    fn rotate_270(self) -> Self {
        vec2d(self.y, -self.x)
    }
}

impl Vec2Ext for Int2 {
    type Scalar = i32;
    #[inline]
    fn from_angle(_radians: i32) -> Int2 {
        panic!()
    }
    #[inline]
    fn rotate_90(self) -> Self {
        int2(-self.y, self.x)
    }
    #[inline]
    fn rotate_270(self) -> Self {
        int2(self.y, -self.x)
    }
}
