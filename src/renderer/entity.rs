use crate::model::Model;


pub fn render(model: &Model) {
    unsafe {
        gl::BindVertexArray(model.data.vao_id);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        model.texture.bind();
        gl::DrawElements(
            gl::TRIANGLES,
            model.data.index_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        model.texture.unbind();
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
    }
}
