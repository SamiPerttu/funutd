//! Voronoi texture basis.

use super::distance::*;
use super::ease::*;
use super::hash::*;
use super::map3base::*;
use super::math::*;
use super::*;
extern crate alloc;
use alloc::{boxed::Box, string::String};

pub fn voronoi_pattern(i: usize, v: Vec3a) -> f32 {
    debug_assert!(i < 26);
    let p = match i / 2 {
        // All of the dot products below are non-negative.
        0 => min(1.0, v.dot(vec3a(1.0, 0.0, 0.0))),
        1 => min(1.0, v.dot(vec3a(-0.5, 1.0, 0.0))),
        2 => min(1.0, v.dot(vec3a(0.0, 1.0, 0.0))),
        3 => min(1.0, v.dot(vec3a(0.5, 0.5, 0.0))),
        4 => min(1.0, v.dot(vec3a(0.0, 0.0, 0.8))),
        5 => min(1.0, v.dot(vec3a(-0.5, 0.5, 0.5))),
        6 => min(1.0, v.dot(vec3a(0.5, -0.5, 0.5))),
        7 => min(1.0, v.dot(vec3a(0.0, -0.5, 1.0))),
        8 => min(1.0, v.dot(vec3a(-0.5, 0.0, 1.0))),
        9 => min(1.0, v.dot(vec3a(-0.3, -0.3, 1.0))),
        10 => min(1.0, v.dot(vec3a(0.3, 0.3, 0.3))),
        11 => min(1.0, v.dot(vec3a(0.4, 0.0, 0.4))),
        _ => min(1.0, v.dot(vec3a(0.0, 0.4, 0.4))),
    };
    if i & 1 == 0 {
        p
    } else {
        1.0 - p
    }
}

pub struct VoronoiState {
    basis: Basis,
    /// Offset of minimum processed cell.
    min_cell: Vec3i,
    /// Offset of maximum processed cell.
    max_cell: Vec3i,
    metric: Distance,
    /// Current distances to closest feature points found.
    distance1: f32,
    distance2: f32,
    distance3: f32,
    color_sharpness: f32,
    color: Vec3a,
    color_weight: f32,
}

impl VoronoiState {
    pub fn new<H: Hasher>(
        hasher: &H,
        seed: u64,
        frequency: f32,
        metric: Distance,
        color_sharpness: f32,
        point: Vec3a,
    ) -> Self {
        Self {
            basis: hasher.query(seed, frequency, point),
            min_cell: vec3i(0, 0, 0),
            max_cell: vec3i(0, 0, 0),
            metric,
            distance1: f32::INFINITY,
            distance2: f32::INFINITY,
            distance3: f32::INFINITY,
            color_sharpness,
            color: vec3a(0.0, 0.0, 0.0),
            color_weight: 0.0,
        }
    }

    pub fn color(&self) -> Vec3a {
        self.color / self.color_weight
    }

    pub fn distance_1(&self) -> f32 {
        self.distance1
    }
    pub fn distance_2(&self) -> f32 {
        self.distance2
    }
    pub fn distance_3(&self) -> f32 {
        self.distance3
    }

    /// Expands next cell or returns false if we are done.
    pub fn expand_next<H: Hasher>(&mut self, hasher: &H) -> bool {
        let dxpos = self.basis.d.x - self.min_cell.x as f32;
        let dxneg = 1.0 - self.basis.d.x + self.max_cell.x as f32;
        let (positive_x, distance_x) = if dxpos < dxneg {
            (true, dxpos)
        } else {
            (false, dxneg)
        };
        let dypos = self.basis.d.y - self.min_cell.y as f32;
        let dyneg = 1.0 - self.basis.d.y + self.max_cell.y as f32;
        let (positive_y, distance_y) = if dypos < dyneg {
            (true, dypos)
        } else {
            (false, dyneg)
        };
        let dzpos = self.basis.d.z - self.min_cell.z as f32;
        let dzneg = 1.0 - self.basis.d.z + self.max_cell.z as f32;
        let (positive_z, distance_z) = if dzpos < dzneg {
            (true, dzpos)
        } else {
            (false, dzneg)
        };
        if distance_x <= distance_y && distance_x <= distance_z {
            if distance_x >= self.distance3 {
                return false;
            }
            let expand_x = if positive_x {
                self.min_cell.x -= 1;
                self.min_cell.x
            } else {
                self.max_cell.x += 1;
                self.max_cell.x
            };
            for y in self.min_cell.y..=self.max_cell.y {
                for z in self.min_cell.z..=self.max_cell.z {
                    self.process_cell(hasher, expand_x, y, z);
                }
            }
        } else if distance_y <= distance_x && distance_y <= distance_z {
            if distance_y >= self.distance3 {
                return false;
            }
            let expand_y = if positive_y {
                self.min_cell.y -= 1;
                self.min_cell.y
            } else {
                self.max_cell.y += 1;
                self.max_cell.y
            };
            for x in self.min_cell.x..=self.max_cell.x {
                for z in self.min_cell.z..=self.max_cell.z {
                    self.process_cell(hasher, x, expand_y, z);
                }
            }
        } else {
            if distance_z >= self.distance3 {
                return false;
            }
            let expand_z = if positive_z {
                self.min_cell.z -= 1;
                self.min_cell.z
            } else {
                self.max_cell.z += 1;
                self.max_cell.z
            };
            for x in self.min_cell.x..=self.max_cell.x {
                for y in self.min_cell.y..=self.max_cell.y {
                    self.process_cell(hasher, x, y, expand_z);
                }
            }
        }
        true
    }

    pub fn process_cell<H: Hasher>(&mut self, hasher: &H, dx: i32, dy: i32, dz: i32) {
        let hx = hasher.hash_x(&self.basis, 0, dx);
        let hxy = hasher.hash_y(&self.basis, hx, dy);
        let mut hash = hasher.hash_z(&self.basis, hxy, dz);

        let n = match hash & 7 {
            0 | 1 | 2 | 3 => 1,
            4 | 5 | 6 => 2,
            _ => 3,
        };
        let offset = Vec3a::new(
            dx as f32 - self.basis.d.x,
            dy as f32 - self.basis.d.y,
            dz as f32 - self.basis.d.z,
        );
        for i in 0..n {
            // Feature location.
            let p = hash_01(hash);
            let delta = p + offset;
            let distance = self.metric.compute(delta);
            if distance < self.distance3 {
                if distance < self.distance1 {
                    self.distance3 = self.distance2;
                    self.distance2 = self.distance1;
                    self.distance1 = distance;
                } else if distance < self.distance2 {
                    self.distance3 = self.distance2;
                    self.distance2 = distance;
                } else {
                    self.distance3 = distance;
                }
            }
            let color_w = exp(80.0 - distance * lerp(15.0, 100.0, self.color_sharpness));
            let color = hash_11(hash64a(hash));
            self.color += color * color_w;
            self.color_weight += color_w;
            if i + 1 < n {
                hash = hash64c(hash);
            }
        }
    }
}

/// Voronoi basis. Cell colors are omitted here, unlike in Camo.
#[derive(Clone)]
pub struct Voronoi<H: Hasher> {
    seed: u64,
    frequency: f32,
    ease: Ease,
    hasher: H,
    metric: Distance,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
}

impl<H: Hasher> Texture for Voronoi<H> {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let frequency = frequency.unwrap_or(self.frequency);
        let mut state = VoronoiState::new(
            &self.hasher,
            self.seed,
            frequency,
            self.metric.clone(),
            1.0,
            point,
        );
        state.process_cell(&self.hasher, 0, 0, 0);
        while state.expand_next(&self.hasher) {}
        let d_vec = vec3a(state.distance_1(), state.distance_2(), state.distance_3());
        vec3a(
            self.ease.at(voronoi_pattern(self.pattern_x, d_vec)) * 2.0 - 1.0,
            self.ease.at(voronoi_pattern(self.pattern_y, d_vec)) * 2.0 - 1.0,
            self.ease.at(voronoi_pattern(self.pattern_z, d_vec)) * 2.0 - 1.0,
        )
    }

    fn get_code(&self) -> String {
        format!(
            "voronoi({}, {:?}, {}, {}, {}, {}, {}, {})",
            self.seed,
            self.frequency,
            self.ease.get_code(),
            self.metric.get_code(),
            self.hasher.get_code(),
            self.pattern_x,
            self.pattern_y,
            self.pattern_z
        )
    }

    fn get_basis_code(&self) -> String {
        format!(
            "voronoi_basis({}, {}, {}, {}, {}, {}, {})",
            self.seed,
            self.ease.get_code(),
            self.metric.get_code(),
            self.hasher.get_code(),
            self.pattern_x,
            self.pattern_y,
            self.pattern_z
        )
    }
}

pub fn voronoi<H: 'static + Hasher>(
    seed: u64,
    frequency: f32,
    ease: Ease,
    metric: Distance,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
) -> Box<dyn Texture> {
    Box::new(Voronoi {
        seed,
        frequency,
        ease,
        metric,
        hasher,
        pattern_x,
        pattern_y,
        pattern_z,
    })
}

pub fn voronoi_basis<H: 'static + Hasher>(
    seed: u64,
    ease: Ease,
    metric: Distance,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
) -> Box<dyn Texture> {
    Box::new(Voronoi {
        seed,
        frequency: 1.0,
        ease,
        metric,
        hasher,
        pattern_x,
        pattern_y,
        pattern_z,
    })
}

/// Camo basis. A colored Worley basis.
#[derive(Clone)]
pub struct Camo<H: Hasher> {
    seed: u64,
    frequency: f32,
    ease: Ease,
    metric: Distance,
    hasher: H,
    border: f32,
    sharpness: f32,
    gradient: f32,
}

impl<H: Hasher> Texture for Camo<H> {
    fn at_frequency(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let frequency = frequency.unwrap_or(self.frequency);
        let mut state = VoronoiState::new(
            &self.hasher,
            self.seed,
            frequency,
            self.metric.clone(),
            self.sharpness,
            point,
        );
        state.process_cell(&self.hasher, 0, 0, 0);
        while state.expand_next(&self.hasher) {}
        let mut color = state.color();
        if state.distance_1() + self.border > state.distance_2() {
            color = lerp(
                Vec3a::zero(),
                color,
                1.0 - self
                    .ease
                    .at(1.0 - (state.distance_2() - state.distance_1()) / self.border),
            );
        }
        let d1 = self.ease.at(min(1.0, state.distance_1()));
        let d = 1.0 - self.gradient * d1;
        vec3a(d * color.x, d * color.y, d * color.z)
    }

    fn get_code(&self) -> String {
        format!(
            "camo({}, {:?}, {}, {}, {}, {:?}, {:?}, {:?})",
            self.seed,
            self.frequency,
            self.ease.get_code(),
            self.metric.get_code(),
            self.hasher.get_code(),
            self.border,
            self.sharpness,
            self.gradient
        )
    }

    fn get_basis_code(&self) -> String {
        format!(
            "camo_basis({}, {}, {}, {}, {:?}, {:?}, {:?})",
            self.seed,
            self.ease.get_code(),
            self.metric.get_code(),
            self.hasher.get_code(),
            self.border,
            self.sharpness,
            self.gradient
        )
    }
}

pub fn camo<H: 'static + Hasher>(
    seed: u64,
    frequency: f32,
    ease: Ease,
    metric: Distance,
    hasher: H,
    border: f32,
    sharpness: f32,
    gradient: f32,
) -> Box<dyn Texture> {
    Box::new(Camo {
        seed,
        frequency,
        ease,
        metric,
        hasher,
        border,
        sharpness,
        gradient,
    })
}

pub fn camo_basis<H: 'static + Hasher>(
    seed: u64,
    ease: Ease,
    metric: Distance,
    hasher: H,
    border: f32,
    sharpness: f32,
    gradient: f32,
) -> Box<dyn Texture> {
    Box::new(Camo {
        seed,
        frequency: 1.0,
        ease,
        metric,
        hasher,
        border,
        sharpness,
        gradient,
    })
}
