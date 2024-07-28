use glfw::Key;
use nalgebra_glm::{Mat4, Vec3};
use nalgebra_glm as glm;

use crate::display::Display;

pub struct Camera {
    position: Vec3,
    rotation: (f32, f32, f32),
    speed: f32,
    turn_speed: f32,
}

impl Camera {
    pub fn new(position: Vec3, rotation: (f32, f32, f32)) -> Self {
        Self { position, rotation, speed: 0.05, turn_speed: 0.1 }
    }

    pub fn move_camera(&mut self, display: &mut Display) {
        if display.keyboard.is_pressed(Key::W) {
            self.position.z -= self.speed;
        }
        if display.keyboard.is_pressed(Key::S) {
            self.position.z += self.speed;
        }

        self.rotation.0 -= -display.mouse.dy() as f32 * self.turn_speed;
        self.rotation.1 += display.mouse.dx() as f32 * self.turn_speed;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let matrix = Mat4::identity();

        let matrix = glm::rotate(&matrix, self.rotation.0.to_radians(), &glm::vec3(1., 0., 0.));
        let matrix = glm::rotate(&matrix, self.rotation.1.to_radians(), &glm::vec3(0., 1., 0.));
        let matrix = glm::rotate(&matrix, self.rotation.2.to_radians(), &glm::vec3(0., 0., 1.));
        let matrix = glm::translate(
            &matrix, 
            &glm::vec3(
                -self.position.x,
                -self.position.y,
                -self.position.z
            )
        );

        matrix
    }
}