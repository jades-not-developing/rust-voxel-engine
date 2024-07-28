use crate::{entity::Entity, math, shader::Shader};


pub fn render(entity: &Entity, shader: &mut Shader) {
    let transform_matrix = math::create_transformation_matrix(
        entity.position, 
        entity.rotation, 
        entity.scale
    );

    unsafe {
        gl::BindVertexArray(entity.model.data.vao_id);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        entity.model.texture.bind();
        shader.bind();
        shader.uniform_mat4("u_Transform", transform_matrix);

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
