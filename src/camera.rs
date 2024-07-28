use glfw::Key;
use nalgebra_glm as glm;
use nalgebra_glm::{Mat4, Vec3};

use crate::display::Display;

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: (f32, f32, f32),
    pub velocity: f32,

    pub speed: f32,
    pub turn_speed: f32,
}

impl Camera {
    pub fn new(position: Vec3, rotation: (f32, f32, f32)) -> Self {
        Self {
            position,
            rotation,
            velocity: 0.0,
            speed: 0.1,
            turn_speed: 0.1,
        }
    }

    pub fn move_camera(&mut self, display: &mut Display) {
        if display.keyboard.is_pressed(Key::W) {
            self.velocity = -self.speed;
        }
        else if display.keyboard.is_pressed(Key::S) {
            self.velocity = self.speed;
        } else {
            self.velocity = 0.;
        }

        self.rotation.0 -= -display.mouse.dy() as f32 * self.turn_speed;
        self.rotation.1 += display.mouse.dx() as f32 * self.turn_speed;

        let dx = -(self.velocity * self.rotation.1.to_radians().sin());
        let dy = self.velocity * self.rotation.0.to_radians().sin();
        let dz = self.velocity * self.rotation.1.to_radians().cos();

        self.position.x += dx;
        self.position.y += dy;
        self.position.z += dz;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let matrix = Mat4::identity();

        let matrix = glm::rotate(
            &matrix,
            self.rotation.0.to_radians(),
            &glm::vec3(1., 0., 0.),
        );
        let matrix = glm::rotate(
            &matrix,
            self.rotation.1.to_radians(),
            &glm::vec3(0., 1., 0.),
        );
        let matrix = glm::rotate(
            &matrix,
            self.rotation.2.to_radians(),
            &glm::vec3(0., 0., 1.),
        );
        glm::translate(
            &matrix,
            &glm::vec3(-self.position.x, -self.position.y, -self.position.z),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(glm::vec3(0., 0., 0.), (0., 0., 0.))
    }
}