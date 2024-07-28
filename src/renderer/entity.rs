use crate::loader::RawModel;

pub fn render(model: &RawModel) {
    unsafe {
        gl::BindVertexArray(model.vao_id);
        gl::EnableVertexAttribArray(0);
        gl::DrawArrays(gl::TRIANGLES, 0, model.vertex_count);
        gl::DisableVertexAttribArray(0);
    }
}