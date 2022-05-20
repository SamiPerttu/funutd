//! Texture generators.

use super::color::*;
use super::distance::*;
use super::dna::*;
use super::ease::*;
use super::map3::*;
use super::map3base::*;
use super::math::*;
use super::noise::*;
use super::voronoi::*;
use super::*;

pub fn gen_metric(dna: &mut Dna, name: &str) -> Distance {
    match dna.get_choice(
        name,
        [
            (1.0, "1-norm"),
            (3.0, "2-norm"),
            (1.0, "4-norm"),
            (1.0, "8-norm"),
            (1.0, "max norm"),
        ],
    ) {
        0 => Distance::Norm1,
        1 => Distance::Norm2,
        2 => Distance::Norm4,
        3 => Distance::Norm8,
        _ => Distance::NormMax,
    }
}

/// Generate an ease that is smooth near zero.
pub fn gen_ease_smooth(dna: &mut Dna, name: &str) -> Ease {
    match dna.get_choice(
        name,
        [
            (1.0, "id"),
            (1.0, "smooth3"),
            (1.0, "smooth5"),
            (1.0, "smooth7"),
            (1.0, "smooth9"),
            (1.0, "squared"),
            (1.0, "cubed"),
            (1.0, "up arc"),
        ],
    ) {
        0 => Ease::Id,
        1 => Ease::Smooth3,
        2 => Ease::Smooth5,
        3 => Ease::Smooth7,
        4 => Ease::Smooth9,
        5 => Ease::Squared,
        6 => Ease::Cubed,
        _ => Ease::UpArc,
    }
}

/// Generate an ease.
pub fn gen_ease(dna: &mut Dna, name: &str) -> Ease {
    match dna.get_choice(
        name,
        [
            (1.0, "id"),
            (1.0, "smooth3"),
            (1.0, "smooth5"),
            (1.0, "smooth7"),
            (1.0, "smooth9"),
            (1.0, "sqrt"),
            (1.0, "squared"),
            (1.0, "cubed"),
            (1.0, "down arc"),
            (1.0, "up arc"),
        ],
    ) {
        0 => Ease::Id,
        1 => Ease::Smooth3,
        2 => Ease::Smooth5,
        3 => Ease::Smooth7,
        4 => Ease::Smooth9,
        5 => Ease::Sqrt,
        6 => Ease::Squared,
        7 => Ease::Cubed,
        8 => Ease::DownArc,
        _ => Ease::UpArc,
    }
}

/// Generate a texture with a palette.
pub fn genmap3palette(complexity: f32, dna: &mut Dna) -> Box<dyn Texture> {
    let space = match dna.get_choice("color space", [(1.0, "Okhsl"), (1.0, "Okhsv")]) {
        0 => Space::HSL,
        _ => Space::HSV,
    };
    let hue_amount = dna.get_f32_xform("hue amount", |x| xerp(0.2, 1.0, x));
    let hue_min = dna.get_f32_in("hue minimum", 0.0, 1.0);
    let value_min = dna.get_f32_in(
        "value minimum",
        0.0,
        match space {
            Space::HSV => 1.0,
            Space::HSL => 0.5,
        },
    );
    let saturation = dna.get_f32_in("saturation", 0.0, 1.0);
    let map = genmap3(complexity, false, dna);
    palette(space, hue_min, hue_amount, value_min, saturation, map)
}

/// Generate a texture.
pub fn genmap3(complexity: f32, is_fractal: bool, dna: &mut Dna) -> Box<dyn Texture> {
    let basis_weight = if complexity <= 10.0 { 1.5 } else { 0.01 };
    let unary_weight = if complexity >= 5.0 { 1.0 } else { 0.01 };
    let binary_weight = if complexity >= 8.0 { 1.0 } else { 0.01 };
    let fractal_weight: f32 = if complexity >= 9.0 {
        0.8
    } else if is_fractal {
        0.0
    } else {
        0.01
    };

    let choice = dna.get_choice(
        "node type",
        [
            (basis_weight, "basis"),
            (unary_weight, "unary"),
            (binary_weight, "binary"),
            (fractal_weight, "fractal"),
        ],
    );

    if choice == 0 {
        // Generate 1 octave of something.
        let seed = dna.get_u32("seed") as u64;
        let frequency = if is_fractal {
            2.0
        } else {
            dna.get_f32_xform("frequency", |x| xerp(4.0, 32.0, x))
        };
        let texture: Box<dyn Texture> = match dna.get_choice(
            "basis",
            [
                (1.0, "gradient noise"),
                (1.0, "value noise"),
                (1.0, "Voronoi"),
                (0.5, "camo"),
                (0.5, "Worley"),
            ],
        ) {
            0 => noise(seed, frequency, tile_all()),
            1 => {
                let ease = gen_ease_smooth(dna, "noise ease");
                vnoise(seed, frequency, ease, tile_all())
            }
            2 => {
                let pattern_x = dna.get_u32_in("Voronoi X pattern", 0, 25);
                let pattern_y = dna.get_u32_in("Voronoi Y pattern", 0, 25);
                let pattern_z = dna.get_u32_in("Voronoi Z pattern", 0, 25);
                let ease = gen_ease(dna, "Voronoi ease");
                let metric = gen_metric(dna, "distance metric");
                voronoi(
                    seed,
                    frequency,
                    ease,
                    metric,
                    tile_all(),
                    pattern_x as usize,
                    pattern_y as usize,
                    pattern_z as usize,
                )
            }
            3 => {
                let w0 = dna.get_f32_in("camo weight 0", 0.0, 0.5);
                let w1 = dna.get_f32_in("camo weight 1", 0.0, 0.5);
                let w2 = dna.get_f32_in("camo weight 2", 0.0, 0.5);
                let ease = gen_ease(dna, "camo ease");
                let metric = gen_metric(dna, "distance metric");
                camo(seed, frequency, ease, metric, tile_all(), w0, w1, w2)
            }
            _ => {
                let pattern_x = dna.get_u32_in("Worley X pattern", 0, 25);
                let pattern_y = dna.get_u32_in("Worley Y pattern", 0, 25);
                let pattern_z = dna.get_u32_in("Worley Z pattern", 0, 25);
                let ease = gen_ease(dna, "Worley ease");
                let metric = gen_metric(dna, "distance metric");
                worley(
                    seed,
                    frequency,
                    ease,
                    metric,
                    tile_all(),
                    pattern_x as usize,
                    pattern_y as usize,
                    pattern_z as usize,
                )
            }
        };
        texture
    } else if choice == 1 {
        // Shape a map with a unary operator.
        let child_complexity = complexity * 0.5 - 1.0;
        let child = dna.call(|dna| genmap3(child_complexity, is_fractal, dna));
        let unary_node = match dna.get_choice(
            "unary node",
            [
                (1.0, "saturate"),
                (1.0, "posterize"),
                (1.0, "overdrive"),
                (1.0, "vreflect"),
                (3.0, "reflect"),
            ],
        ) {
            0 => {
                let amount = dna.get_f32_in("amount", 2.0, 10.0);
                saturate(amount, child)
            }
            1 => {
                let levels = dna.get_f32_in("levels", 2.0, 10.0);
                let sharpness: f32 = dna.get_f32("sharpness");
                posterize(levels, sharpness, child)
            }
            2 => {
                let amount = dna.get_f32_in("amount", 2.0, 10.0);
                overdrive(amount, child)
            }
            3 => {
                let amount = dna.get_f32_in("amount", 2.0, 10.0);
                vreflect(amount, child)
            }
            _ => {
                let amount = dna.get_f32_in("amount", 1.0, 2.0);
                let x_offset = dna.get_f32_in("X offset", -1.0, 1.0);
                let y_offset = dna.get_f32_in("Y offset", -1.0, 1.0);
                let z_offset = dna.get_f32_in("Z offset", -1.0, 1.0);
                reflect(amount, vec3(x_offset, y_offset, z_offset), child)
            }
        };
        unary_node
    } else if choice == 2 {
        // Combine two maps with a binary operator.
        let child_complexity = complexity * 0.5 - 1.0;
        let child_a = dna.call(|dna| genmap3(child_complexity, is_fractal, dna));
        let child_b = dna.call(|dna| genmap3(child_complexity, is_fractal, dna));
        let binary_node = match dna.get_choice(
            "binary node",
            [
                (1.0, "rotate"),
                (1.0, "softmix"),
                (1.0, "layer"),
                (1.0, "displace"),
            ],
        ) {
            0 => {
                let amount = dna.get_f32_in("amount", 3.0, 10.0);
                rotate(amount, child_a, child_b)
            }
            1 => {
                let amount = dna.get_f32_in("amount", 1.0, 10.0);
                softmix3(amount, child_a, child_b)
            }
            2 => {
                let width = dna.get_f32_in("width", 1.0, 4.0);
                layer(width, child_a, child_b)
            }
            _ => {
                let amount = dna.get_f32_in("amount", 0.05, 0.25);
                displace(amount, child_a, child_b)
            }
        };
        binary_node
    } else {
        // Fractalize map by sampling many octaves.
        let child_complexity = min(8.0, complexity * 0.5 - 1.0);
        let child_basis = dna.call(|dna| genmap3(child_complexity, true, dna));
        let base_f = dna.get_f32_in("base frequency", 1.5, 9.0);
        let roughness = dna.get_f32_xform("roughness", |x| xerp(0.4, 0.9, x));
        let octaves = dna.get_u32_in("octaves", 2, 10) as usize;
        let lacunarity = dna.get_f32_xform("lacunarity", |x| xerp(1.5, 3.0, x));
        let displace = if dna.get_choice("displace", [(0.333, "on"), (0.666, "off")]) == 0 {
            dna.get_f32_in("amount", 0.0, 0.25)
        } else {
            0.0
        };
        let layer = if dna.get_choice("layer", [(0.333, "on"), (0.666, "off")]) == 0 {
            dna.get_f32_in("width", 1.0, 4.0)
        } else {
            0.0
        };
        fractal(
            base_f,
            octaves,
            roughness,
            lacunarity,
            displace,
            layer,
            child_basis,
        )
    }
}
