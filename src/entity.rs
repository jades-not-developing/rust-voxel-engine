use crate::model::Model;
use nalgebra_glm as glm;

pub struct Entity {
    pub model: Model,
    pub position: glm::Vec3,
    pub rotation: (f32, f32, f32),
    pub scale: f32,
}

impl Entity {
    pub fn new(model: Model, position: glm::Vec3, rotation: (f32, f32, f32), scale: f32) -> Self {
        Self {
            model,
            position,
            rotation,
            scale,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation.0 += x;
        self.rotation.1 += y;
        self.rotation.2 += z;
    }
}
