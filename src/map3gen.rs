//! Texture generators.

use super::color::*;
use super::dna::*;
use super::map3::*;
use super::map3base::*;
use super::math::*;
use super::noise::*;
use super::voronoi::*;
use super::*;

pub fn genmap3palette(complexity: f32, dna: &mut Dna) -> Box<dyn Texture> {
    let hue_amount = dna.get_f32_in(0.2, 1.0);
    let hue_min = dna.get_f32_in(0.000001, 1.0);
    let space = match dna.get_u32_in(0, 1) {
        0 => Space::HSL,
        _ => Space::HSV,
    };
    let map = genmap3(complexity, dna);
    palette(space, hue_min, hue_amount, map)
}

pub fn genmap3(complexity: f32, dna: &mut Dna) -> Box<dyn Texture> {
    let basis_weight = if complexity <= 10.0 { 0.20 } else { 0.0 };
    let unary_weight = if complexity > 5.0 { 0.30 } else { 0.0 };
    let binary_weight = if complexity > 8.0 { 0.25 } else { 0.0 };
    let fractal_weight: f32 = if complexity > 10.0 { 0.25 } else { 0.0 };

    let x = dna.get_f32_in(
        0.0,
        basis_weight + unary_weight + binary_weight + fractal_weight,
    );

    if x < basis_weight {
        // Generate 1 octave of something.
        let seed = dna.get_u32() as u64;
        let frequency = xerp(1.5, 32.0, dna.get_f32());
        let texture: Box<dyn Texture> = match dna.get_u32_in(0, 1) {
            0 => vnoise(seed, frequency, tile_all()),
            _ => {
                let pattern_x = dna.get_u32_in(0, 12);
                let pattern_y = dna.get_u32_in(0, 12);
                let pattern_z = dna.get_u32_in(0, 12);
                voronoi(
                    seed,
                    frequency,
                    tile_all(),
                    pattern_x as usize,
                    pattern_y as usize,
                    pattern_z as usize,
                )
            }
        };
        texture
    } else if x < basis_weight + unary_weight {
        // Shape a map with a unary operator.
        let child_complexity = complexity * 0.5 - 1.0;
        let child = dna.call(|dna| genmap3(child_complexity, dna));
        let unary_node = match dna.get_u32_in(0, 4) {
            0 => {
                let amount = dna.get_f32_in(2.0, 10.0);
                saturate(amount, child)
            }
            1 => {
                let levels = dna.get_f32_in(2.0, 10.0);
                let sharpness: f32 = dna.get_f32();
                posterize(levels, sharpness, child)
            }
            2 => {
                let amount = dna.get_f32_in(2.0, 10.0);
                overdrive(amount, child)
            }
            3 => {
                let amount = dna.get_f32_in(2.0, 10.0);
                vreflect(amount, child)
            }
            _ => {
                let amount = dna.get_f32_in(1.0, 5.0);
                let x_offset = dna.get_f32_in(-1.0, 1.0);
                let y_offset = dna.get_f32_in(-1.0, 1.0);
                let z_offset = dna.get_f32_in(-1.0, 1.0);
                reflect(amount, vec3a(x_offset, y_offset, z_offset), child)
            }
        };
        unary_node
    } else if x < basis_weight + unary_weight + binary_weight {
        // Combine two maps with a binary operator.
        let child_complexity = complexity * 0.5 - 1.0;
        let child_a = dna.call(|dna| genmap3(child_complexity, dna));
        let child_b = dna.call(|dna| genmap3(child_complexity, dna));
        let binary_node = match dna.get_u32_in(0, 2) {
            0 => {
                let amount = dna.get_f32_in(3.0, 10.0);
                rotate(amount, child_a, child_b)
            }
            1 => {
                let amount = dna.get_f32_in(1.0, 10.0);
                softmix3(amount, child_a, child_b)
            }
            _ => {
                let amount = dna.get_f32_in(0.05, 0.25);
                displace(amount, child_a, child_b)
            }
        };
        binary_node
    } else {
        // Fractalize map by sampling many octaves.
        let child_complexity = min(10.0, complexity * 0.5 - 1.0);
        let child_basis = dna.call(|dna| genmap3(child_complexity, dna));
        let base_f = dna.get_f32_in(1.5, 8.5);
        let roughness = xerp(0.4, 0.8, dna.get_f32_in(0.0, 1.0));
        let octaves = dna.get_u32_in(2, 8) as usize;
        let displace = if dna.get_f32() < 0.5 {
            dna.get_f32_in(0.0, 0.3)
        } else {
            0.0
        };
        fractal(base_f, octaves, roughness, displace, child_basis)
    }
}
