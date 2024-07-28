use crate::{entity::Entity, shader::Shader};
use nalgebra_glm as glm;

pub mod entity;

pub struct MasterRenderer {
    projection_matrix: glm::Mat4,
}

impl MasterRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            projection_matrix: glm::Mat4::new_perspective(
                (width as f32) / (height as f32),
                70.,
                0.1,
                10000.,
            ),
        }
    }

    pub fn prepare(&self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.4, 0.7, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn render(&self, entity: &Entity, shader: &mut Shader) {
        shader.bind();
        shader.uniform_mat4("u_Projection", self.projection_matrix);
        shader.unbind();
        entity::render(entity, shader);
    }
}
