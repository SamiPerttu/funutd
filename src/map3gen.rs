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
extern crate alloc;
use alloc::boxed::Box;

/// Generate a distance metric.
pub fn gen_metric(dna: &mut Dna, name: &str) -> Distance {
    dna.choice(
        name,
        [
            (1.0, "1-norm", Distance::Norm1),
            (4.0, "2-norm", Distance::Norm2),
            (1.0, "4-norm", Distance::Norm4),
            (1.0, "8-norm", Distance::Norm8),
            (1.0, "max norm", Distance::NormMax),
        ],
    )
}

/// Generate an ease that is smooth near zero.
pub fn gen_ease_smooth(dna: &mut Dna, name: &str) -> Ease {
    dna.choice(
        name,
        [
            (1.0, "smooth3", Ease::Smooth3),
            (2.0, "smooth5", Ease::Smooth5),
            (1.0, "smooth7", Ease::Smooth7),
            (1.0, "smooth9", Ease::Smooth9),
            (1.0, "squared", Ease::Squared),
            (1.0, "cubed", Ease::Cubed),
            (1.0, "up arc", Ease::UpArc),
        ],
    )
}

/// Generate an ease suitable for the Voronoi basis.
pub fn gen_ease_voronoi(dna: &mut Dna, name: &str) -> Ease {
    dna.choice(
        name,
        [
            (1.0, "id", Ease::Id),
            (1.0, "smooth3", Ease::Smooth3),
            (1.0, "smooth5", Ease::Smooth5),
            (1.0, "smooth7", Ease::Smooth7),
            (1.0, "smooth9", Ease::Smooth9),
            (1.0, "squared", Ease::Squared),
        ],
    )
}

/// Generate an ease.
pub fn gen_ease(dna: &mut Dna, name: &str) -> Ease {
    dna.choice(
        name,
        [
            (1.0, "id", Ease::Id),
            (1.0, "smooth3", Ease::Smooth3),
            (1.0, "smooth5", Ease::Smooth5),
            (1.0, "smooth7", Ease::Smooth7),
            (1.0, "smooth9", Ease::Smooth9),
            (1.0, "sqrt", Ease::Sqrt),
            (1.0, "squared", Ease::Squared),
            (1.0, "cubed", Ease::Cubed),
            (1.0, "down arc", Ease::DownArc),
            (1.0, "up arc", Ease::UpArc),
        ],
    )
}

/// Generate a texture with a palette.
pub fn genmap3palette(complexity: f32, tiling: TilingMode, dna: &mut Dna) -> Box<dyn Texture> {
    match tiling {
        TilingMode::None => genmap3palette_hasher(complexity, tile_none(), dna),
        TilingMode::Z => genmap3palette_hasher(complexity, tile_z(), dna),
        TilingMode::XY => genmap3palette_hasher(complexity, tile_xy(), dna),
        TilingMode::All => genmap3palette_hasher(complexity, tile_all(), dna),
    }
}

/// Generate a texture with a palette.
pub fn genmap3palette_hasher<H: 'static + Hasher>(
    complexity: f32,
    hasher: H,
    dna: &mut Dna,
) -> Box<dyn Texture> {
    let h1 = dna.f32("hue 1");
    let s1 = dna.f32("saturation 1");
    let l1 = dna.f32_xform("lightness 1", sqrt);
    let h2 = dna.f32("hue 2");
    let s2 = dna.f32("saturation 2");
    let l2 = dna.f32_xform("lightness 2", sqrt);
    let h3 = dna.f32("hue 3");
    let s3 = dna.f32("saturation 3");
    let l3 = dna.f32_xform("lightness 3", sqrt);
    let map = genmap3_hasher(complexity, false, hasher, dna);

    palette(h1, s1, l1, h2, s2, l2, h3, s3, l3, map)
}

/// Generate a texture.
pub fn genmap3(complexity: f32, tiling: TilingMode, dna: &mut Dna) -> Box<dyn Texture> {
    match tiling {
        TilingMode::None => genmap3_hasher(complexity, false, tile_none(), dna),
        TilingMode::Z => genmap3_hasher(complexity, false, tile_z(), dna),
        TilingMode::XY => genmap3_hasher(complexity, false, tile_xy(), dna),
        TilingMode::All => genmap3_hasher(complexity, false, tile_all(), dna),
    }
}

/// Generate a texture.
pub fn genmap3_hasher<H: 'static + Hasher>(
    complexity: f32,
    is_fractal: bool,
    hasher: H,
    dna: &mut Dna,
) -> Box<dyn Texture> {
    let basis_weight = if complexity <= 10.0 {
        1.5
    } else if complexity <= 40.0 {
        0.8
    } else if complexity <= 80.0 {
        0.4
    } else {
        0.2
    };
    let unary_weight = if complexity >= 20.0 {
        1.5
    } else if complexity >= 5.0 {
        1.0
    } else {
        0.01
    };
    let binary_weight = if complexity >= 8.0 { 1.0 } else { 0.01 };
    let fractal_weight: f32 = if is_fractal {
        // If we are a child of a fractalizer, we cannot start a new one.
        0.0
    } else if complexity >= 9.0 {
        0.8
    } else {
        // A small weight is enough to allow the user to edit the value in the editor.
        0.01
    };

    let choice = dna.index(
        "node type",
        [
            (basis_weight, "basis"),
            (unary_weight, "unary"),
            (binary_weight, "binary"),
            (fractal_weight, "fractal"),
        ],
    );

    if choice == 0 {
        dna.group();
        // Generate 1 octave of something.
        let seed = dna.u32("seed") as u64;
        let frequency = if is_fractal {
            // The frequency comes from the fractalizer so we can choose any value.
            2.0
        } else {
            dna.f32_xform("frequency", |x| xerp(2.0, 32.0, x))
        };
        let texture: Box<dyn Texture> = match dna.index(
            "basis",
            [
                (1.0, "gradient noise"),
                (1.0, "value noise"),
                (1.0, "Voronoi"),
                (0.5, "camo"),
            ],
        ) {
            0 => noise(seed, frequency, hasher.clone()),
            1 => {
                dna.group();
                let ease = gen_ease_smooth(dna, "noise ease");
                dna.ungroup();
                vnoise(seed, frequency, ease, hasher.clone())
            }
            2 => {
                dna.group();
                let pattern_x = dna.u32_in("Voronoi X pattern", 0, 25);
                let pattern_y = dna.u32_in("Voronoi Y pattern", 0, 25);
                let pattern_z = dna.u32_in("Voronoi Z pattern", 0, 25);
                let ease = gen_ease_voronoi(dna, "Voronoi ease");
                let metric = gen_metric(dna, "distance metric");
                dna.ungroup();
                voronoi(
                    seed,
                    frequency,
                    ease,
                    metric,
                    hasher.clone(),
                    pattern_x as usize,
                    pattern_y as usize,
                    pattern_z as usize,
                )
            }
            _ => {
                dna.group();
                let border = dna.generate(|dna| {
                    if dna.index("border", [(0.5, "on"), (0.5, "off")]) == 0 {
                        dna.f32_in("border width", 0.01, 0.10)
                    } else {
                        0.0
                    }
                });
                let sharpness = dna.f32_in("camo sharpness", 0.0, 1.0);
                let gradient = dna.f32_in("camo gradient", 0.0, 1.0);
                let ease = gen_ease_smooth(dna, "camo ease");
                let metric = gen_metric(dna, "distance metric");
                dna.ungroup();
                camo(
                    seed,
                    frequency,
                    ease,
                    metric,
                    hasher.clone(),
                    border,
                    sharpness,
                    gradient,
                )
            }
        };
        dna.ungroup();
        texture
    } else if choice == 1 {
        // Shape a map with a unary operator.
        dna.group();
        let child_complexity = complexity * 0.5 - 1.0;
        let unary_node = match dna.index(
            "unary node",
            [
                (1.0, "saturate"),
                (1.0, "posterize"),
                (1.0, "overdrive"),
                (1.0, "vreflect"),
                (2.0, "reflect"),
                (3.0, "shift"),
            ],
        ) {
            0 => {
                dna.group();
                let amount = dna.f32_in("amount", 1.0, 5.0);
                let child = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                saturate(amount * amount, child)
            }
            1 => {
                dna.group();
                let levels = dna.f32_in("levels", 2.0, 10.0);
                let sharpness: f32 = dna.f32("sharpness");
                let child = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                posterize(levels, sharpness, child)
            }
            2 => {
                dna.group();
                let amount = dna.f32_in("amount", 1.0, 5.0);
                let child = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                overdrive(amount * amount, child)
            }
            3 => {
                dna.group();
                let amount = dna.f32_in("amount", 1.0, 10.0);
                let child = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                vreflect(amount, child)
            }
            4 => {
                dna.group();
                let amount = dna.f32_in("amount", 1.0, 2.0);
                let x_offset = dna.f32_in("X offset", -1.0, 1.0);
                let y_offset = dna.f32_in("Y offset", -1.0, 1.0);
                let z_offset = dna.f32_in("Z offset", -1.0, 1.0);
                let child = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                reflect(amount, vec3(x_offset, y_offset, z_offset), child)
            }
            _ => {
                dna.group();
                let seed = dna.u32("seed");
                let child = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                shift(seed, child)
            }
        };
        dna.ungroup();
        unary_node
    } else if choice == 2 {
        // Combine two maps with a binary operator.
        dna.group();
        let child_complexity = complexity * 0.5 - 1.0;
        let binary_node = match dna.index(
            "binary node",
            [
                (1.0, "rotate"),
                (1.0, "softmix"),
                (1.0, "layer"),
                (2.0, "displace"),
            ],
        ) {
            0 => {
                dna.group();
                let amount = dna.f32_in("amount", 1.0, 3.0);
                let child_a = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                let child_b = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                rotate(amount, child_a, child_b)
            }
            1 => {
                dna.group();
                let amount = dna.f32_in("amount", 1.0, 5.0);
                let displacement = dna.f32_in("displacement", 0.0, 0.5);
                let child_a = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                let child_b = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                softmix3(amount * amount, displacement, child_a, child_b)
            }
            2 => {
                dna.group();
                let width = dna.f32_in("width", 1.0, 3.0);
                let ease = gen_ease(dna, "layer ease");
                let child_a = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                let child_b = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                layer(width, ease, child_a, child_b)
            }
            _ => {
                dna.group();
                let amount = dna.f32_in("amount", 0.0, 0.5);
                let child_a = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                let child_b = dna.generate(|dna| {
                    genmap3_hasher(child_complexity, is_fractal, hasher.clone(), dna)
                });
                dna.ungroup();
                displace(amount, child_a, child_b)
            }
        };
        dna.ungroup();
        binary_node
    } else {
        // Fractalize map by sampling many octaves.
        dna.group();
        let child_complexity = min(20.0, complexity * 0.5 - 1.0);
        let base_f = dna.f32_in("base frequency", 1.5, 9.0);
        let roughness = dna.f32_xform("roughness", |x| xerp(0.4, 0.9, x));
        let octaves = dna.u32_in("octaves", 2, 10) as usize;
        let first_octave = dna.u32_in("first octave", 0, octaves as u32 - 1) as usize;
        let lacunarity = dna.f32_xform("lacunarity", |x| xerp(1.5, 3.0, x));
        let displace = dna.generate(|dna| {
            if dna.index("displace", [(0.333, "on"), (0.666, "off")]) == 0 {
                dna.f32_in("amount", 0.0, 0.5)
            } else {
                0.0
            }
        });
        let layer = dna.generate(|dna| {
            if dna.index("layer", [(0.333, "on"), (0.666, "off")]) == 0 {
                dna.f32_in("width", 1.0, 4.0)
            } else {
                0.0
            }
        });
        let child_basis =
            dna.generate(|dna| genmap3_hasher(child_complexity, true, hasher.clone(), dna));
        dna.ungroup();
        fractal(
            base_f,
            octaves,
            first_octave,
            roughness,
            lacunarity,
            displace,
            layer,
            child_basis,
        )
    }
}
