use glfw::{Action, Key};
use rust_voxel::{display::Display, renderer::Renderer};

pub struct Game {
    display: Display,
    renderer: Renderer,
}

impl Game {
    pub fn run(&mut self) {
        while !self.display.should_close() {
            self.display.poll_events(|e| match e {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    false
                },
                _ => true,
            });

            self.renderer.prepare();

            self.display.swap_buffers();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            display: Display::new(1280, 720, "hi"),
            renderer: Renderer,
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.run();
}
