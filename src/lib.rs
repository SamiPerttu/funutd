#[allow(non_snake_case)]
#[allow(clippy::excessive_precision)]
pub mod color;
pub mod dna;
#[allow(non_snake_case)]
pub mod hash;
pub mod lcg;
pub mod map3;
pub mod map3base;
pub mod map3gen;
pub mod math;
pub mod noise;
pub mod rnd;
pub mod vec;
pub mod voronoi;

pub use rnd::Rnd;
pub use vec::*;
