pub mod prelude;
pub mod vec;
pub mod map4;
pub mod hash;

// Standard RNG.
pub type Rnd = rand_krull::Krull65;
pub use rand::Rng;

pub use vec::*;
