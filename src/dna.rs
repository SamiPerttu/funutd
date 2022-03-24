use super::*;

/*
A parameter system for procedural generation.

The Dna object contains the necessary, mutable
context that is threaded through the generation process.

Inside Dna, we have a vector of raw data as 32-bit integers.
We use the raw data to supply parameters for procedural generators.
Each integer supplies bits for one parameter.

Procedural generator parameter sets are tree shaped.
The index for each parameter is hashed from a local tree address.
Collisions are ignored. Collisions diminish the state space
available, but we can always use a larger vector of raw data
to reduce collisions.

We keep the current address inside Dna and update it as parameters are drawn.
*/

const ADDRESS_LEVELS: usize = 8;

#[derive(Clone)]
pub struct Dna {
    /// Current address.
    /// When we draw a parameter, we increase the address of the bottommost node by 1.
    /// When we descend, we add a new level.
    address: Vec<u32>,
    data: Vec<u32>,
}

impl Dna {
    /// Create a new Dna from u64 seed.
    pub fn new(size: usize, seed: u64) -> Dna {
        let mut rnd = Rnd::from_u64(seed);
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(rnd.next_u32());
        }
        Dna {
            address: vec![0],
            data,
        }
    }

    /// Calculates the current parameter hash based on the current local address.
    fn get_hash(&self) -> u64 {
        let l = self.address.len();
        let n = ADDRESS_LEVELS.min(l);
        let mut h: u64 = n as u64;
        // Use an ad hoc hash.
        for i in (l - n)..l {
            h = (h ^ self.address[i] as u64 ^ (h >> 32)).wrapping_mul(0xd6e8feb86659fd93);
        }
        (h ^ (h >> 32)).wrapping_mul(0xd6e8feb86659fd93)
    }

    /// Used when drawing a parameter.
    /// Calculates the current index where parameters are drawn.
    /// Advances the current address.
    fn draw_index(&mut self) -> usize {
        let h = self.get_hash();
        *self.address.last_mut().unwrap() += 1;
        h as usize % self.data.len()
    }

    /// Returns a full range u32 parameter.
    pub fn get_u32(&mut self) -> u32 {
        let index = self.draw_index();
        self.data[index]
    }

    /// Returns a full range i32 parameter.
    pub fn get_i32(&mut self) -> i32 {
        let index = self.draw_index();
        self.data[index] as i32
    }

    /// Returns a u32 parameter in the given inclusive range.
    pub fn get_u32_in(&mut self, minimum: u32, maximum: u32) -> u32 {
        debug_assert!(minimum > 0 || maximum < u32::MAX);
        self.get_u32() % (maximum - minimum + 1) + minimum
    }

    /// Returns an i32 parameter in the given inclusive range.
    pub fn get_i32_in(&mut self, minimum: i32, maximum: i32) -> i32 {
        self.get_i32() % (maximum - minimum + 1) + minimum
    }

    /// Returns an f32 parameter in right exclusive range [0, 1[.
    pub fn get_f32(&mut self) -> f32 {
        let index = self.draw_index();
        self.data[index] as f32 / ((1u64 << 32) as f32)
    }

    /// Returns an f32 parameter in right exclusive range [minimum, maximum[.
    pub fn get_f32_in(&mut self, minimum: f32, maximum: f32) -> f32 {
        self.get_f32() * (maximum - minimum) + minimum
    }

    /// Returns an f64 parameter in right exclusive range [0, 1[.
    pub fn get_f64(&mut self) -> f64 {
        let index = self.draw_index();
        self.data[index] as f64 / ((1u64 << 32) as f64)
    }

    /// Returns an f64 parameter in right exclusive range [minimum, maximum[.
    pub fn get_f64_in(&mut self, minimum: f64, maximum: f64) -> f64 {
        self.get_f64() * (maximum - minimum) + minimum
    }

    /// Calls a subgenerator.
    pub fn call<X, F: Fn(&mut Dna) -> X>(&mut self, f: F) -> X {
        self.address.push(0);
        let x = f(self);
        self.address.pop();
        *self.address.last_mut().unwrap() += 1;
        x
    }
}
