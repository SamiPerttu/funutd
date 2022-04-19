//! Utility functions for working with LCGs (linear congruential generators).

use super::math::*;

/// LCG iteration is state <- state * m + p.
/// Returns the (m, p) pair that iterates by n steps at once.
/// Assumes (m, p) is full period.
pub fn get_jump<T: Int>(m: T, p: T, n: T) -> (T, T) {
    // Algorithm from Brown, F. B., "Random Number Generation with Arbitrary Stride",
    // Transactions of the American Nuclear Society, 1994.
    let mut unit_m = m;
    let mut unit_p = p;
    let mut jump_m = T::one();
    let mut jump_p = T::zero();
    let mut delta = n;

    while delta > T::zero() {
        if delta & T::one() == T::one() {
            jump_m = jump_m.wrapping_mul(unit_m);
            jump_p = jump_p.wrapping_mul(unit_m).wrapping_add(unit_p);
        }
        unit_p = (unit_m + T::one()).wrapping_mul(unit_p);
        unit_m = unit_m.wrapping_mul(unit_m);
        delta = delta >> 1;
    }
    (jump_m, jump_p)
}

/// LCG iteration is state <- state * m + p.
/// Returns the number of iterations between origin state and the given state.
/// Assumes (m, p) is full period.
pub fn get_iterations<T: Int>(m: T, p: T, origin: T, state: T) -> T {
    let mut jump_m = m;
    let mut jump_p = p;
    let mut ordinal = T::zero();
    let mut bit = T::one();
    let mut address = origin;

    while address != state {
        if (bit & address) != (bit & state) {
            address = address.wrapping_mul(jump_m).wrapping_add(jump_p);
            ordinal = ordinal | bit;
        }
        jump_p = (jump_m.wrapping_add(T::one())).wrapping_mul(jump_p);
        jump_m = jump_m.wrapping_mul(jump_m);
        bit = bit << 1;
    }
    ordinal
}

/// LCG iteration is state <- state * m + p.
/// Returns state after the specified number of iterations from the origin state.
/// Assumes (m, p) is full period.
pub fn get_state<T: Int>(m: T, p: T, origin: T, iterations: T) -> T {
    let mut jump_m = m;
    let mut jump_p = p;
    let mut state = origin;
    let mut ordinal = iterations;

    while ordinal > T::zero() {
        if ordinal & T::one() == T::one() {
            state = state.wrapping_mul(jump_m).wrapping_add(jump_p);
        }
        jump_p = (jump_m.wrapping_add(T::one())).wrapping_mul(jump_p);
        jump_m = jump_m.wrapping_mul(jump_m);
        ordinal = ordinal >> 1;
    }
    state
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    #[test]
    pub fn run_tests() {
        let mut rnd_state: u128 = 0;
        let mut rnd = || -> u128 {
            rnd_state = rnd_state.wrapping_mul(rnd::LCG_M128_1).wrapping_add(0xffff);
            rnd_state
        };

        for _ in 0..1 << 12 {
            let m = match rnd() % 3 {
                0 => rnd::LCG_M128_1,
                1 => rnd::LCG_M128_2,
                _ => rnd::LCG_M128_3,
            };
            let p = rnd() | 1;
            let origin = rnd();

            assert_eq!(
                origin.wrapping_mul(m).wrapping_add(p),
                get_state(m, p, origin, 1)
            );
            assert_eq!(
                1,
                get_iterations(m, p, origin, origin.wrapping_mul(m).wrapping_add(p))
            );

            // Run some consistency tests.
            let state = rnd();
            let n = get_iterations(m, p, origin, state);
            assert_eq!(state, get_state(m, p, origin, n));

            let (m_total, p_total) = get_jump(m, p, n);
            assert_eq!(origin.wrapping_mul(m_total).wrapping_add(p_total), state);

            let n = rnd();
            let state = get_state(m, p, origin, n);
            assert_eq!(n, get_iterations(m, p, origin, state));

            // Get h <= n.
            let h = n & rnd();
            let state_h = get_state(m, p, origin, h);
            assert_eq!(n - h, get_iterations(m, p, state_h, state));
        }
    }
}
