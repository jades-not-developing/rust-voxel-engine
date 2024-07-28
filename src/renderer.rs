use crate::loader::RawModel;

pub struct MasterRenderer;

impl MasterRenderer {
    pub fn prepare(&self) {
        unsafe {
            gl::ClearColor(0.4, 0.7, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render(&self, model: &RawModel) {
        EntityRenderer::render(model);
    }
}

pub struct EntityRenderer;

impl EntityRenderer {
    pub fn render(model: &RawModel) {
        unsafe {
            gl::BindVertexArray(model.vao_id);
            gl::EnableVertexAttribArray(0);
            gl::DrawArrays(gl::TRIANGLES, 0, model.vertex_count);
            gl::DisableVertexAttribArray(0);
        }
    }
}
