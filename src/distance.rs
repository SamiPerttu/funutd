//! Distance metrics for Worley basis.

use super::math::*;
use super::*;
extern crate alloc;
use alloc::string::String;

#[derive(Debug, Clone)]
pub enum Distance {
    Norm1,
    Norm2,
    Norm4,
    Norm8,
    NormMax,
}

impl Distance {
    pub fn compute(&self, vector: Vec3a) -> f32 {
        match self {
            Distance::Norm1 => abs(vector.x) + abs(vector.y) + abs(vector.z),
            Distance::Norm2 => vector.length(),
            Distance::Norm4 => sqrt(sqrt(
                squared(squared(vector.x))
                    + squared(squared(vector.y))
                    + squared(squared(vector.z)),
            )),
            Distance::Norm8 => sqrt(sqrt(sqrt(
                squared(squared(squared(vector.x)))
                    + squared(squared(squared(vector.y)))
                    + squared(squared(squared(vector.z))),
            ))),
            Distance::NormMax => max(abs(vector.x), max(abs(vector.y), abs(vector.z))),
        }
    }
    pub fn get_code(&self) -> String {
        format!("Distance::{:?}", self)
    }
}
