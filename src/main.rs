use glfw::{Action, Key};
use rust_voxel::{display::Display, loader::Loader, renderer::MasterRenderer, shader::Shader};

pub struct Game {
    display: Display,
    renderer: MasterRenderer,
    loader: Loader,
}

impl Game {
    pub fn run(&mut self) {
        #[rustfmt::skip]
        let model = self.loader.load_to_vao(
            vec![
                -0.5,  0.5, 0.0,
                -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
                0.5,  0.5, 0.0,
            ],
            vec![
                0, 1, 2,
                2, 3, 0,
            ]
        );

        let mut shader = Shader::from_files("default.vert.glsl", "default.frag.glsl").unwrap();
        shader.bind();
        shader.uniform_vec3("u_Color", nalgebra_glm::vec3(0.0, 1.0, 0.0));

        while !self.display.should_close() {
            #[allow(clippy::match_like_matches_macro)]
            self.display.poll_events(|e| match e {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => false,
                _ => true,
            });

            shader.bind();
            self.renderer.prepare();
            self.renderer.render(&model);
            shader.unbind();

            self.display.swap_buffers();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            display: Display::new(1280, 720, "hi"),
            renderer: MasterRenderer,
            loader: Loader::default(),
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.run();
}
