use nalgebra_glm::Vec3;

use crate::mouse::Mouse;

pub struct Camera {
    position: Vec3,
    rotation: (f32, f32, f32),
}

impl Camera {
    pub fn new(position: Vec3, rotation: (f32, f32, f32)) -> Self {
        Self { position, rotation }
    }

    pub fn move_camera(&mut self, mouse: &mut Mouse) {
        self.rotation.0 = -mouse.dy() as f32;
        self.rotation.1 = mouse.dx() as f32;
    }
}