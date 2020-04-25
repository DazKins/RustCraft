use std::vec::Vec;
use cgmath::Vector3;

use crate::model::Model;

pub struct ModelBuilder {
    vertices: Vec<f32>,
    indices: Vec<i32>
}

impl ModelBuilder {
    pub fn new() -> ModelBuilder {
        return ModelBuilder {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn push_vertex(&mut self, vertex: Vector3<f32>) {
        self.vertices.push(vertex.x);
        self.vertices.push(vertex.y);
        self.vertices.push(vertex.z);
    }

    pub fn push_index(&mut self, index: i32) {
        self.indices.push(index);
    }

    pub fn build(&self) -> Model {
        return Model::new(self.vertices.as_slice(), self.indices.as_slice());
    }
}
