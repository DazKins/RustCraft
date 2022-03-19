use cgmath::{InnerSpace, Vector2};
use std::{collections::{hash_map::DefaultHasher}, hash::{Hash, Hasher}};

fn weigh(x: f32) -> f32 {
    6.0 * x * x * x * x * x - 15.0 * x * x * x * x + 10.0 * x * x * x
}

fn lerp(w: f32, a: f32, b: f32) -> f32 {
    a + w * (b - a)
}

#[derive(Clone, Copy)]
pub struct Noise {
    octaves: u8,
    persistence: f32,
    base_frequency: f32,
}

impl Noise {
    pub fn new(octaves: u8, persistence: f32, frequency: f32) -> Self {
        Noise {
            octaves,
            persistence,
            base_frequency: frequency,
        }
    }

    pub fn sample(&self, vec: Vector2<f32>) -> f32 {
        let mut sample: f32 = 0.0;
        let mut frequency: f32 = self.base_frequency;
        let mut amplitude = 1.0;
        let mut corrective_constant = 0.0;

        for _ in 0..self.octaves {
            let perlin_sample = get_sample(vec * frequency);
            sample += perlin_sample * amplitude;

            corrective_constant += amplitude;

            frequency *= 2.0;
            amplitude /= self.persistence;
        }

        sample / corrective_constant
    }
}

#[derive(Hash, PartialEq, Eq)]
struct GridCoordinate {
    seed: u64,
    x: i32,
    y: i32,
}

impl GridCoordinate {
    pub fn new(x: i32, y: i32) -> Self {
        GridCoordinate {
            seed: 0,
            x,
            y
        }
    }
}

fn get_grid_vector(mut grid_coordinate: GridCoordinate) -> Vector2<f32> {
    grid_coordinate.seed = 1600624470247646139;

    let mut hasher_x = DefaultHasher::new();
    grid_coordinate.hash(&mut hasher_x);
    let hash_x = hasher_x.finish();

    grid_coordinate.seed = 15735240048817799231;

    let mut hasher_y = DefaultHasher::new();
    grid_coordinate.hash(&mut hasher_y);
    let hash_y = hasher_y.finish();

    let vx = (hash_x as f32) / (u64::MAX as f32);
    let vy = (hash_y as f32) / (u64::MAX as f32);

    Vector2::new (vx, vy).normalize()
}

pub fn get_sample(vec: Vector2<f32>) -> f32 {
    let gv00 = Vector2::new(vec.x.floor(), vec.y.floor());
    let gv10 = Vector2::new((vec.x + 1.0).floor(), vec.y.floor());
    let gv01 = Vector2::new(vec.x.floor(), (vec.y + 1.0).floor());
    let gv11 = Vector2::new((vec.x + 1.0).floor(), (vec.y + 1.0).floor());

    let v00 = get_grid_vector(GridCoordinate::new(
        gv00.x as i32,
        gv00.y as i32,
    ));
    let v10 = get_grid_vector(GridCoordinate::new(
        gv10.x as i32,
        gv10.y as i32,
    ));
    let v01 = get_grid_vector(GridCoordinate::new(
        gv01.x as i32,
        gv01.y as i32,
    ));
    let v11 = get_grid_vector(GridCoordinate::new(
        gv11.x as i32,
        gv11.y as i32,
    ));

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

    static CORRECTIVE_CONSTANT: f32 =
        1.4142135623730950488016887242096980785696718753769480731766797381; // root(2)

    let v = lerp(sx, a, b) * CORRECTIVE_CONSTANT;

    v.clamp(-1.0, 1.0)
}
