use crate::prelude::*;
use wrapping_arithmetic::wrappit;

/// LCG iteration is state <- state * m + p.
/// Returns the (m, p) pair that iterates by n steps at once.
/// Assumes (m, p) is full period.
#[wrappit]
pub fn get_jump(m: u128, p: u128, n: u128) -> (u128, u128) {
    // Algorithm from Brown, F. B., "Random Number Generation with Arbitrary Stride",
    // Transactions of the American Nuclear Society, 1994.
    let mut unit_m = m;
    let mut unit_p = p;
    let mut jump_m = 1;
    let mut jump_p = 0;
    let mut delta = n;

    while delta > 0 {
        if delta & 1 != 0 {
            jump_m = jump_m * unit_m;
            jump_p = jump_p * unit_m + unit_p;
        }
        unit_p = (unit_m + 1) * unit_p;
        unit_m = squared(unit_m);
        delta >>= 1;
    }
    (jump_m, jump_p)
}

/// LCG iteration is state <- state * m + p.
/// Returns the number of iterations between origin state and the given state.
/// Assumes (m, p) is full period.
#[wrappit]
pub fn get_iterations(m: u128, p: u128, origin: u128, state: u128) -> u128 {
    let mut jump_m  = m;
    let mut jump_p  = p;
    let mut ordinal = 0;
    let mut bit     = 1;
    let mut address = origin;

    while address != state && bit != 0 {
        if (bit & address) != (bit & state) {
            address = address * jump_m + jump_p;
            ordinal = ordinal + bit;
        }
        jump_p = (jump_m + 1) * jump_p;
        jump_m = squared(jump_m);
        bit <<= 1;
    }
    // Note: if address != state here, then (m, p) is not full period.
    ordinal
}

/// LCG iteration is state <- state * m + p.
/// Returns state after the specified number of iterations from origin state.
/// Assumes (m, p) is full period.
#[wrappit]
pub fn get_state(m: u128, p: u128, origin: u128, iterations: u128) -> u128 {
    let mut jump_m  = m;
    let mut jump_p  = p;
    let mut state   = origin;
    let mut ordinal = iterations;

    while ordinal != 0 {
        if ordinal & 1 != 0 {
            state = state * jump_m + jump_p;
        }
        jump_p = (jump_m + 1) * jump_p;
        jump_m = squared(jump_m);
        ordinal >>= 1;
    }
    state
}
