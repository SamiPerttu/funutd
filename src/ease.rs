//! Easing functions.
use super::math::*;
extern crate alloc;
use alloc::string::String;

#[derive(Debug, Clone)]
pub enum Ease {
    Id,
    Smooth3,
    Smooth5,
    Smooth7,
    Smooth9,
    Sqrt,
    Squared,
    Cubed,
    UpArc,
    DownArc,
}

impl Ease {
    pub fn at(&self, x: f32) -> f32 {
        match self {
            Ease::Id => x,
            Ease::Smooth3 => smooth3(x),
            Ease::Smooth5 => smooth5(x),
            Ease::Smooth7 => smooth7(x),
            Ease::Smooth9 => smooth9(x),
            Ease::Sqrt => sqrt(x),
            Ease::Squared => squared(x),
            Ease::Cubed => cubed(x),
            Ease::UpArc => uparc(x),
            Ease::DownArc => downarc(x),
        }
    }
    pub fn get_code(&self) -> String {
        format!("Ease::{:?}", self)
    }
}
