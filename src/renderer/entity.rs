use crate::{entity::Entity, shader::Shader};


pub fn render(entity: &Entity, shader: &mut Shader) {
    unsafe {
        gl::BindVertexArray(entity.model.data.vao_id);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        entity.model.texture.bind();
        shader.bind();
        shader.uniform_mat4("u_Transform", entity.get_transformation_matrix());

        gl::DrawElements(
            gl::TRIANGLES,
            entity.model.data.index_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );

        shader.unbind();
        entity.model.texture.unbind();
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
    }
}
