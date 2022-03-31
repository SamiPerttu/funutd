use super::hash::*;
use super::map3base::*;
use super::map3::*;
use super::math::*;
use super::*;

pub fn voronoi_pattern(i: usize) -> Vec3a {
    match i {
        0 => vec3a(1.0, 0.0, 0.0),
        1 => vec3a(-1.0, 1.0, 0.0),
        2 => vec3a(0.0, 1.0, 0.0),
        3 => vec3a(1.0, 1.0, 0.0),
        4 => vec3a(0.0, 0.0, 1.0),
        5 => vec3a(-1.0, 1.0, 1.0),
        6 => vec3a(1.0, -1.0, 1.0),
        7 => vec3a(0.0, -1.0, 1.0),
        8 => vec3a(-1.0, 0.0, 1.0),
        9 => vec3a(-1.0, -1.0, 1.0),
        10 => vec3a(1.0, 1.0, 1.0),
        11 => vec3a(1.0, 0.0, 1.0),
        _ => vec3a(0.0, 1.0, 1.0),
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
        }
    }

    pub fn distance_1(&self) -> f32 { self.distance1 }
    pub fn distance_2(&self) -> f32 { self.distance2 }
    pub fn distance_3(&self) -> f32 { self.distance3 }

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
        if distance_x < distance_y && distance_x < distance_z {
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
        } else if distance_y < distance_x && distance_y < distance_z {
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
            5 | 6 => 2,
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
            if i + 1 < n {
                hash = hash64c(hash);
            }
        }
    }
}

pub struct Voronoi<H: Hasher> {
    seed: u64,
    frequency: f32,
    hasher: H,
}

impl<H: Hasher> Voronoi<H> {}

impl<H: Hasher> Texture for Voronoi<H> {
    fn at(&self, point: Vec3a) -> Vec3a {
        self.at_frequency(self.frequency, point)
    }

    fn get_code(&self) -> String {
        format!(
            "voronoi({}, {}, {})",
            self.seed,
            self.frequency,
            self.hasher.get_code()
        )
    }
}

impl<H: Hasher> BasisTexture for Voronoi<H> {
    fn at_frequency(&self, frequency: f32, point: Vec3a) -> Vec3a {
        let mut state = VoronoiState::new(&self.hasher, self.seed, frequency, point);
        while state.expand_next(&self.hasher) {}
        // TODO replace components with dot products with chosen patterns.
        vec3a(state.distance_1(), state.distance_2(), state.distance_3())
    }

    fn get_basis_code(&self) -> String {
        format!("voronoi_basis({}, {})", self.seed, self.hasher.get_code())
    }
}