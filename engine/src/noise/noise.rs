use std::collections::HashMap;
use cgmath::{Vector2, InnerSpace};

use rand::Rng;

#[derive(Hash, PartialEq, Eq)]
struct GridCoordinate {
    x: i32,
    y: i32
}

fn weigh(x: f32) -> f32 {
    6.0 * x * x * x * x * x - 15.0 * x * x * x * x + 10.0 * x * x * x
}

fn lerp(w: f32, a: f32, b: f32) -> f32 {
    a + w * (b - a)
}

pub struct Noise {
    octaves: u8,
    persistence: f32,
    base_frequency: f32,
    perlin_noise: PerlinNoise
}

impl Noise {
    pub fn new(octaves: u8, persistence: f32,frequency: f32) -> Self {
        Noise {
            octaves,
            persistence,
            base_frequency: frequency,
            perlin_noise: PerlinNoise::new()
        }
    }

    pub fn sample(&mut self, vec: Vector2<f32>) -> f32 {
        let mut sample: f32 = 0.0;
        let mut frequency: f32 = self.base_frequency;
        let mut amplitude = 1.0;
        let mut corrective_constant = 0.0;

        for _ in 0..self.octaves {
            let perlin_sample = self.perlin_noise.sample(vec * frequency);
            sample += perlin_sample * amplitude;

            corrective_constant += amplitude;

            frequency  *= 2.0;
            amplitude /= self.persistence;
        }

        sample / corrective_constant
    }
}

struct PerlinNoise {
    grid_vectors: HashMap<GridCoordinate, Vector2<f32>>
}

impl PerlinNoise {
    pub fn new() -> Self {
        PerlinNoise {
            grid_vectors: HashMap::new()
        }
    }

    fn get_grid_vector(&mut self, grid_coordinate: GridCoordinate) -> Vector2<f32> {
        let mut rng = rand::thread_rng();
        match self.grid_vectors.get(&grid_coordinate) {
            Some(v) => v.to_owned(),
            None => {
                let vec = Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
                self.grid_vectors.insert(grid_coordinate, vec);
                vec
            }
        }
    }

    pub fn sample(&mut self, vec: Vector2<f32>) -> f32 {
        let gv00 = Vector2::new(vec.x.floor(), vec.y.floor());
        let gv10 = Vector2::new((vec.x + 1.0).floor(), vec.y.floor());
        let gv01 = Vector2::new(vec.x.floor(), (vec.y + 1.0).floor());
        let gv11 = Vector2::new((vec.x + 1.0).floor(), (vec.y + 1.0).floor());

        let v00 = self.get_grid_vector(GridCoordinate { x: gv00.x as i32, y: gv00.y as i32 });
        let v10 = self.get_grid_vector(GridCoordinate { x: gv10.x as i32, y: gv10.y as i32 });
        let v01 = self.get_grid_vector(GridCoordinate { x: gv01.x as i32, y: gv01.y as i32 });
        let v11 = self.get_grid_vector(GridCoordinate { x: gv11.x as i32, y: gv11.y as i32 });

        let dv00 = vec - gv00;
        let dv10 = vec - gv10;
        let dv01 = vec - gv01;
        let dv11 = vec - gv11;

        let w00 = v00.dot(dv00);
        let w10 = v10.dot(dv10);
        let w01 = v01.dot(dv01);
        let w11 = v11.dot(dv11);

        let sx = weigh(vec.x - vec.x.floor());
        let sy = weigh(vec.y - vec.y.floor());

        let a = lerp(sy, w00, w01);
        let b = lerp(sy, w10, w11);

        static CORRECTIVE_CONSTANT: f32 = 1.4142135623730950488016887242096980785696718753769480731766797381; // root(2)

        let v = lerp(sx, a, b) * CORRECTIVE_CONSTANT;

        v.clamp(-1.0, 1.0)
    }
}