pub struct Renderer;

impl Renderer {
    pub fn prepare(&self) {
        unsafe {
            gl::ClearColor(0.4, 0.7, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}