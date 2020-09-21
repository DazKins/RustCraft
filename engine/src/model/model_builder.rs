use std::vec::Vec;
use cgmath::{ Vector3, Vector2 };

use crate::model::Model;

pub struct ModelBuilder {
    x: f32,
    y: f32,
    z: f32,

    u: f32,
    v: f32,

    vertex_data: Vec<f32>,
    indices: Vec<i32>,

    vertex_count: i32
}

impl ModelBuilder {
    pub fn new() -> ModelBuilder {
        return ModelBuilder {
            x: 0.0,
            y: 0.0,
            z: 0.0,

            u: 0.0,
            v: 0.0,

            vertex_count: 0,

            vertex_data: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn set_uv(&mut self, uv: Vector2<f32>) -> &mut ModelBuilder {
        self.u = uv.x;
        self.v = uv.y;

        self
    }

    pub fn set_xyz(&mut self, xyz: Vector3<f32>) -> &mut ModelBuilder {
        self.x = xyz.x;
        self.y = xyz.y;
        self.z = xyz.z;

        self
    }

    pub fn set_x(&mut self, x: f32) -> &mut ModelBuilder {
        self.x = x;

        self
    }

    pub fn set_y(&mut self, y: f32) -> &mut ModelBuilder {
        self.y = y;

        self
    }

    pub fn set_z(&mut self, z: f32) -> &mut ModelBuilder {
        self.z = z;

        self
    }

    pub fn set_u(&mut self, u: f32) -> &mut ModelBuilder {
        self.u = u;

        self
    }

    pub fn set_v(&mut self, v: f32) -> &mut ModelBuilder {
        self.v = v;

        self
    }

    pub fn push_vertex(&mut self) {
        self.vertex_data.push(self.x);
        self.vertex_data.push(self.y);
        self.vertex_data.push(self.z);

        self.vertex_data.push(self.u);
        self.vertex_data.push(self.v);

        self.indices.push(self.vertex_count);

        self.vertex_count += 1;
    }

    // I'm not planning to do any complex model loading atm so I'll leave indices abstracted for now
    // pub fn push_index(&mut self, index: i32) {
    //     self.indices.push(index);
    // }

    pub fn build(&self) -> Model {
        return Model::new(self.vertex_data.as_slice(), self.indices.as_slice());
    }
}
