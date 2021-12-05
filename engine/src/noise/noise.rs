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
    grid_vectors: HashMap<GridCoordinate, Vector2<f32>>
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            grid_vectors: HashMap::new()
        }
    }

    fn get_grid_vector(&mut self, grid_coordinate: GridCoordinate) -> Vector2<f32> {
        let mut rng = rand::thread_rng();
        match self.grid_vectors.get(&grid_coordinate) {
            Some(v) => v.to_owned(),
            None => {
                let vec = Vector2::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
                self.grid_vectors.insert(grid_coordinate, vec);
                vec
            }
        }
    }

    pub fn sample(&mut self, vec: Vector2<f32>) -> f32 {
        let delta = 0.0001;

        let gv0 = Vector2::new(vec.x.floor() + delta, vec.y.floor() + delta);
        let gv1 = Vector2::new((vec.x + 1.0).floor() + delta, vec.y.floor() + delta);
        let gv2 = Vector2::new(vec.x.floor() + delta, (vec.y + 1.0).floor() + delta);
        let gv3 = Vector2::new((vec.x + 1.0).floor() + delta, (vec.y + 1.0).ceil() + delta);

        let v0 = self.get_grid_vector(GridCoordinate { x: gv0.x as i32, y: gv0.y as i32 });
        let v1 = self.get_grid_vector(GridCoordinate { x: gv1.x as i32, y: gv1.y as i32 });
        let v2 = self.get_grid_vector(GridCoordinate { x: gv2.x as i32, y: gv2.y as i32 });
        let v3 = self.get_grid_vector(GridCoordinate { x: gv3.x as i32, y: gv3.y as i32 });

        let dv0 = vec - gv0;
        let dv1 = vec - gv1;
        let dv2 = vec - gv2;
        let dv3 = vec - gv3;

        let w0 = v0.dot(dv0.normalize());
        let w1 = v1.dot(dv1.normalize());
        let w2 = v2.dot(dv2.normalize());
        let w3 = v3.dot(dv3.normalize());

        let sx = weigh(vec.x - vec.x.floor());
        let sy = weigh(vec.y - vec.y.floor());

        let a = lerp(sx, w0, w1);
        let b = lerp(sx, w2, w3);
        lerp(sy, a, b)
    }
}