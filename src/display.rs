use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub struct Display {
    pub glfw: Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Display {
    pub fn new(width: u32, height: u32, title: impl AsRef<str>) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        let (mut window, events) = glfw.create_window(width, height, title.as_ref(), glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|s| glfw.get_proc_address_raw(s));

        Self {
            glfw,
            window,
            events,
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn poll_events<P>(&mut self, p: P)
    where
        P: Fn(glfw::WindowEvent) -> bool
    {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            if !p(event) {
                self.window.set_should_close(true);
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}