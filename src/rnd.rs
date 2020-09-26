use crate::prelude::*;
use wrapping_arithmetic::wrappit;

/// LCG multiplier.
const LCG_M: u128 = 0x2360ed051fc65da44385df649fccf645;

#[derive(Copy, Clone)]
pub struct Rnd {
    state: u128,
    stream: u128,
}

// Krull64 RNG properties.
// -high quality, non-cryptographic
// -256-bit state with full period, 64-bit output
// -2**128 streams of length 2**128 supporting random access
// -streams are equidistributed with each 64-bit output appearing 2**64 times
// -technically this is an LCG using SplitMix64 for output mixing, so like an overengineered PCG.
// Why Krull64?
// -RNGs are too fast with weak streaming and seeding procedures that barely clear the bar
// -mixing of streams is not proven
// -confident randomness with properly seeded streams for parallel simulations
// -every bit of the 256-bit state is sufficiently mixed in the output

impl Rnd {
    #[inline]
    fn multiplier(&self) -> u128 {
        LCG_M
    }

    #[inline]
    fn increment(&self) -> u128 {
        // One bit of stream does not fit the increment.
        // We discard the highest bit to keep adjacent streams maximally decorrelated.
        (self.stream << 1) | 1
    }

    #[wrappit] #[inline]
    fn step(&mut self) {
        self.state = self.state * self.multiplier() + self.increment();
    }

    #[inline]
    fn get(&self) -> u64 {
        // Take high 64 bits from the LCG and mix high 64 bits of stream into it
        // before the output hash.
        let x = ((self.state ^ self.stream) >> 64) as u64;
        // The output hash is SplitMix64 by Sebastiano Vigna.
        hashc(x)
    }

    #[inline]
    pub fn next(&mut self) -> u64 {
        self.step();
        self.get()
    }

    /// Creates a new Krull RNG with a default all-zeros state.
    pub fn new() -> Self {
        Rnd { state: 0, stream: 0 }
    }

    /// Creates a new Krull RNG with a seeded stream.
    pub fn from_seed(seed: u128) -> Self {
        Rnd { state: 0, stream: seed }
    }

    /// Jumps forward (if steps > 0) or backward (if steps < 0).
    pub fn jump(&mut self, steps: i128) {
        self.state = crate::lcg::get_state(self.multiplier(), self.increment(), self.state, steps as u128);
    }

    /// Returns current position in stream.
    pub fn position(&self) -> u128 {
        crate::lcg::get_iterations(self.multiplier(), self.increment(), 0, self.state)
    }

    /// Sets position in stream.
    pub fn set_position(&mut self, position: u128) {
        self.state = crate::lcg::get_state(self.multiplier(), self.increment(), 0, position);
    }

    /// Returns current stream.
    pub fn stream(&self) -> u128 {
        self.stream
    }

    /// Sets stream.
    pub fn set_stream(&mut self, stream: u128) {
        self.stream = stream;
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }

    pub fn next_u64(&mut self) -> u64 {
        self.next()
    }
}
