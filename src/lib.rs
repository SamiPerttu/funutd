//! FunUTD is a 3-D procedural texture library.

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
pub mod map3gen;
pub mod math;
pub mod noise;
pub mod prelude;
pub mod rnd;
pub mod vec;
#[allow(clippy::too_many_arguments)]
pub mod voronoi;

pub use rnd::Rnd;
pub use vec::*;
