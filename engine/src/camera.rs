use cgmath::{perspective, Deg, Matrix4, Rad, Vector3};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            fov,
            aspect,
            near,
            far,
        }
    }

    pub fn get_transform_matrix(&self) -> Matrix4<f32> {
        let perspective: Matrix4<f32> =
            perspective(Deg(self.fov), self.aspect, self.near, self.far).into();
        let position = Matrix4::from_translation(-self.position);

        let rotation = Matrix4::from_angle_x(Rad(self.rotation.x))
            * Matrix4::from_angle_y(Rad(self.rotation.y))
            * Matrix4::from_angle_z(Rad(self.rotation.z));

        return perspective * rotation * position;
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation
    }
}
