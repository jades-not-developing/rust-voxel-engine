use std::ffi::c_void;

use gl::types::{GLint, GLuint};

pub struct RawModel {
    pub vao_id: GLuint,
    pub index_count: GLint,
}

#[derive(Default)]
pub struct Loader {
    vaos: Vec<GLuint>,
    vbos: Vec<GLuint>,
    ebos: Vec<GLuint>,
}

impl Loader {
    pub fn load_to_vao(&mut self, vertices: Vec<f32>, indices: Vec<u32>) -> RawModel {
        let vao_id = self.create_vao();
        let index_count = indices.len() as GLint;
        self.store_data_in_attrib_list(vertices, 0, 3);
        self.bind_indices_buffer(indices);
        unsafe {
            gl::BindVertexArray(0);
        }

        RawModel {
            vao_id,
            index_count,
        }
    }

    fn create_vao(&mut self) -> GLuint {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
        }

        assert!(vao_id != 0, "Failed to create VAO");

        self.vaos.push(vao_id);

        vao_id
    }

    fn store_data_in_attrib_list(
        &mut self,
        data: Vec<f32>,
        attrib_number: GLuint,
        dimensions: i32,
    ) {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as isize,
                data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                attrib_number,
                dimensions,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        self.vbos.push(vbo_id);
    }

    fn bind_indices_buffer(&mut self, indices: Vec<u32>) {
        let mut ebo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                 indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        for vao in &self.vaos {
            unsafe {
                gl::DeleteVertexArrays(1, vao);
            }
        }

        for vbo in &self.vbos {
            unsafe {
                gl::DeleteBuffers(1, vbo);
            }
        }

        for ebo in &self.ebos {
            unsafe {
                gl::DeleteBuffers(1, ebo);
            }
        }
    }
}
