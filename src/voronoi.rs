//! Voronoi texture basis.

use super::hash::*;
use super::map3base::*;
use super::math::*;
use super::ease::*;
use super::*;

pub fn voronoi_pattern(i: usize, v: Vec3a) -> f32 {
    debug_assert!(i < 26);
    let p = match i % 13 {
        // All of the dot products below are non-negative.
        0 => clamp01(v.dot(vec3a(1.0, 0.0, 0.0))) * 2.0 - 1.0,
        1 => clamp01(v.dot(vec3a(-1.0, 1.0, 0.0))) * 2.0 - 1.0,
        2 => clamp01(v.dot(vec3a(0.0, 1.0, 0.0))) * 2.0 - 1.0,
        3 => clamp01(v.dot(vec3a(0.5, 0.5, 0.0))) * 2.0 - 1.0,
        4 => clamp01(v.dot(vec3a(0.0, 0.0, 1.0))) * 2.0 - 1.0,
        5 => clamp01(v.dot(vec3a(-0.5, 0.5, 0.5))) * 2.0 - 1.0,
        6 => clamp01(v.dot(vec3a(1.0, -1.0, 1.0))) * 2.0 - 1.0,
        7 => clamp01(v.dot(vec3a(0.0, -1.0, 1.0))) * 2.0 - 1.0,
        8 => clamp01(v.dot(vec3a(-1.0, 0.0, 1.0))) * 2.0 - 1.0,
        9 => clamp01(v.dot(vec3a(-0.5, -0.5, 1.0))) * 2.0 - 1.0,
        10 => clamp01(v.dot(vec3a(0.333, 0.333, 0.333))) * 2.0 - 1.0,
        11 => clamp01(v.dot(vec3a(0.5, 0.0, 0.5))) * 2.0 - 1.0,
        _ => clamp01(v.dot(vec3a(0.0, 0.5, 0.5))) * 2.0 - 1.0,
    };
    if i < 13 {
        p
    } else {
        -p
    }
}

pub struct VoronoiState {
    basis: Basis,
    /// Offset of minimum processed cell.
    min_cell: Vec3i,
    /// Offset of maximum processed cell.
    max_cell: Vec3i,
    /// Current distances to closest feature points found.
    distance1: f32,
    distance2: f32,
    distance3: f32,
    color: Vec3a,
    color_weight: f32,
}

impl VoronoiState {
    pub fn new<H: Hasher>(hasher: &H, seed: u64, frequency: f32, point: Vec3a) -> Self {
        Self {
            basis: hasher.query(seed, frequency, point),
            min_cell: vec3i(0, 0, 0),
            max_cell: vec3i(0, 0, 0),
            distance1: f32::INFINITY,
            distance2: f32::INFINITY,
            distance3: f32::INFINITY,
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
            let distance2: f32 = (p + offset).length_squared();
            if distance2 < self.distance3 * self.distance3 {
                let distance = sqrt(distance2);
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
            let color_w = exp(50.0 - distance2 * 25.0);
            let color = hash_11(hash64a(hash));
            self.color += color * color_w;
            self.color_weight += color_w;
            if i + 1 < n {
                hash = hash64c(hash);
            }
        }
    }
}

pub struct Voronoi<H: Hasher> {
    seed: u64,
    frequency: f32,
    ease: Ease,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
}

impl<H: Hasher> Texture for Voronoi<H> {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let frequency = frequency.unwrap_or(self.frequency);
        let mut state = VoronoiState::new(&self.hasher, self.seed, frequency, point);
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
            "voronoi({}, {}, {}, {}, {}, {}, {})",
            self.seed,
            self.frequency,
            self.ease.get_code(),
            self.hasher.get_code(),
            self.pattern_x,
            self.pattern_y,
            self.pattern_z
        )
    }

    fn get_basis_code(&self) -> String {
        format!(
            "voronoi_basis({}, {}, {}, {}, {}, {})",
            self.seed,
            self.ease.get_code(),
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
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
) -> Box<dyn Texture> {
    Box::new(Voronoi {
        seed,
        frequency,
        ease,
        hasher,
        pattern_x,
        pattern_y,
        pattern_z,
    })
}

pub fn voronoi_basis<H: 'static + Hasher>(
    seed: u64,
    ease: Ease,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
) -> Box<dyn Texture> {
    Box::new(Voronoi {
        seed,
        frequency: 1.0,
        ease,
        hasher,
        pattern_x,
        pattern_y,
        pattern_z,
    })
}

pub struct Camo<H: Hasher> {
    seed: u64,
    frequency: f32,
    ease: Ease,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
}

impl<H: Hasher> Texture for Camo<H> {
    fn at(&self, point: Vec3a, frequency: Option<f32>) -> Vec3a {
        let frequency = frequency.unwrap_or(self.frequency);
        let mut state = VoronoiState::new(&self.hasher, self.seed, frequency, point);
        state.process_cell(&self.hasher, 0, 0, 0);
        while state.expand_next(&self.hasher) {}
        let d_vec = vec3a(state.distance_1(), state.distance_2(), state.distance_3());
        let color = state.color();
        vec3a(
            (self.ease.at(voronoi_pattern(self.pattern_x, d_vec)) * 2.0 - 1.0) * color.x,
            (self.ease.at(voronoi_pattern(self.pattern_y, d_vec)) * 2.0 - 1.0) * color.y,
            (self.ease.at(voronoi_pattern(self.pattern_z, d_vec)) * 2.0 - 1.0) * color.z,
        )
    }

    fn get_code(&self) -> String {
        format!(
            "camo({}, {}, {}, {}, {}, {}, {})",
            self.seed,
            self.frequency,
            self.ease.get_code(),
            self.hasher.get_code(),
            self.pattern_x,
            self.pattern_y,
            self.pattern_z
        )
    }

    fn get_basis_code(&self) -> String {
        format!(
            "camo_basis({}, {}, {}, {}, {}, {})",
            self.seed,
            self.ease.get_code(),
            self.hasher.get_code(),
            self.pattern_x,
            self.pattern_y,
            self.pattern_z
        )
    }
}

pub fn camo<H: 'static + Hasher>(
    seed: u64,
    frequency: f32,
    ease: Ease,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
) -> Box<dyn Texture> {
    Box::new(Camo {
        seed,
        frequency,
        ease,
        hasher,
        pattern_x,
        pattern_y,
        pattern_z,
    })
}

pub fn camo_basis<H: 'static + Hasher>(
    seed: u64,
    ease: Ease,
    hasher: H,
    pattern_x: usize,
    pattern_y: usize,
    pattern_z: usize,
) -> Box<dyn Texture> {
    Box::new(Camo {
        seed,
        frequency: 1.0,
        ease,
        hasher,
        pattern_x,
        pattern_y,
        pattern_z,
    })
}
