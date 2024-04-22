//! FunUTD is a 3-D procedural texture library.
#![cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate alloc;

#[allow(non_snake_case)]
#[allow(clippy::excessive_precision)]
#[allow(clippy::too_many_arguments)]
pub mod color;
pub mod distance;
#[allow(clippy::too_many_arguments)]
pub mod dna;
pub mod ease;
#[allow(non_snake_case)]
pub mod hash;
pub mod lcg;
#[allow(clippy::too_many_arguments)]
pub mod map3;
pub mod map3base;
#[allow(clippy::let_and_return)]
pub mod map3gen;
pub mod math;
#[allow(clippy::manual_range_patterns)]
pub mod noise;
pub mod prelude;
pub mod rnd;
pub mod vec;
#[allow(clippy::too_many_arguments)]
#[allow(clippy::manual_range_patterns)]
pub mod voronoi;

pub use rnd::Rnd;
pub use vec::*;
