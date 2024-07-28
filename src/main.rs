use glfw::{Action, Key};
use rust_voxel::{display::Display, loader::Loader, renderer::MasterRenderer};

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

        while !self.display.should_close() {
            #[allow(clippy::match_like_matches_macro)]
            self.display.poll_events(|e| match e {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => false,
                _ => true,
            });

            self.renderer.prepare();
            self.renderer.render(&model);

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
