pub mod hash;
pub mod map4;
pub mod math;
pub mod vec;

// Standard RNG.
pub type Rnd = rand_krull::Krull65;
pub use rand::Rng;

pub use vec::*;
