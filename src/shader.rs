use std::{ffi::CString, path::Path};

use gl::types::{GLchar, GLuint};

#[derive(Debug, Clone)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl Into<u32> for ShaderType {
    fn into(self) -> u32 {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    program_id: GLuint,
}

impl Shader {
    pub fn from_files(vertex_path: impl AsRef<Path>, fragment_path: impl AsRef<Path>) -> std::io::Result<Self> {
        let vertex = Self::load_shader(vertex_path, ShaderType::Vertex)?;
        let fragment = Self::load_shader(fragment_path, ShaderType::Fragment)?;

        unsafe {
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex);
            gl::AttachShader(program_id, fragment);
            gl::LinkProgram(program_id);
            gl::ValidateProgram(program_id);

            let mut success = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

            if success != 1 {
                let mut log_len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut buf = Vec::with_capacity(log_len as usize);
                gl::GetProgramInfoLog(
                    program_id,
                    log_len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                panic!(
                    "Shader Link Failed:\n{}",
                    CString::from_raw(buf.as_mut_ptr()).to_string_lossy()
                );
            }

            Ok(Self {
                program_id
            })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn load_shader(path: impl AsRef<Path>, shader_type: ShaderType) -> std::io::Result<GLuint> {
        let file = std::fs::read_to_string(path)?;
        let file_len = file.len();
        let file_cstr = CString::new(file)?;

        let shader_id = unsafe { gl::CreateShader(shader_type.clone().into()) };
        unsafe {
            gl::ShaderSource(
                shader_id,
                1,
                [file_cstr.as_ptr()].as_ptr(),
                [file_len as i32].as_ptr(),
            );
            gl::CompileShader(shader_id);

            let mut success = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

            if success != 1 {
                let mut log_len = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut buf = Vec::with_capacity(log_len as usize);
                gl::GetShaderInfoLog(
                    shader_id,
                    log_len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                panic!(
                    "Shader Compilation Failed [{shader_type:?}]:\n{}",
                    CString::from_raw(buf.as_mut_ptr()).to_string_lossy()
                );
            }

            Ok(shader_id)
        }
    }
}
