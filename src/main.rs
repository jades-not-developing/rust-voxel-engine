
use glfw::{Action, Key};
use nalgebra_glm as glm;
use rust_voxel::{
    camera::Camera, display::Display, entity::Entity, loader::Loader, model::Model, mouse::Mouse, renderer::MasterRenderer, shader::Shader
};

pub struct Game {
    display: Display,
    renderer: MasterRenderer,
    loader: Loader,
}

impl Game {
    pub fn run(&mut self) {
        #[rustfmt::skip]
        let raw_model = self.loader.load_to_vao(
            vec![
				-0.5,  0.5, -0.5,	
				-0.5, -0.5, -0.5,	
				 0.5, -0.5, -0.5,	
				 0.5,  0.5, -0.5,		
				
				-0.5,  0.5, 0.5,	
				-0.5, -0.5, 0.5,	
				 0.5, -0.5, 0.5,	
				 0.5,  0.5, 0.5,
				
				0.5,  0.5, -0.5,	
				0.5, -0.5, -0.5,	
				0.5, -0.5,  0.5,	
				0.5,  0.5,  0.5,
				
				-0.5,  0.5, -0.5,	
				-0.5, -0.5, -0.5,	
				-0.5, -0.5,  0.5,	
				-0.5,  0.5,  0.5,
				
				-0.5, 0.5,  0.5,
				-0.5, 0.5, -0.5,
				 0.5, 0.5, -0.5,
				 0.5, 0.5,  0.5,
				
				-0.5, -0.5,  0.5,
				-0.5, -0.5, -0.5,
				 0.5, -0.5, -0.5,
				 0.5, -0.5,  0.5
            ],
            vec![
                0,1,3,	
				3,1,2,	
				4,5,7,
				7,5,6,
				8,9,11,
				11,9,10,
				12,13,15,
				15,13,14,	
				16,17,19,
				19,17,18,
				20,21,23,
				23,21,22
            ],
            vec![
				0.,0.,
				0.,1.,
				1.,1.,
				1.,0.,			
				0.,0.,
				0.,1.,
				1.,1.,
				1.,0.,			
				0.,0.,
				0.,1.,
				1.,1.,
				1.,0.,
				0.,0.,
				0.,1.,
				1.,1.,
				1.,0.,
				0.,0.,
				0.,1.,
				1.,1.,
				1.,0.,
				0.,0.,
				0.,1.,
				1.,1.,
				1.,0.
            ]
        );
        let texture = self.loader.load_texture("res/img/dirt.png");

        let model = Model {
            data: raw_model,
            texture,
        };

        let mut entity = Entity::new(model, glm::vec3(0., 0., -2.), (0., 0., 0.), 1.0);

        let mut shader = Shader::from_files("default.vert.glsl", "default.frag.glsl").unwrap();
        shader.bind();
        shader.uniform_vec3("u_Color", nalgebra_glm::vec3(0.0, 1.0, 0.0));

        let mut camera = Camera::new(glm::vec3(0., 0., 0.), (0., 0., 0.));


        while !self.display.should_close() {
            #[allow(clippy::match_like_matches_macro)]
            self.display.poll_events(move |e, mouse, keyboard| match e {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => false,
                glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                    keyboard.press(key);
                    true
                },
                glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                    keyboard.release(key);
                    true
                },
                glfw::WindowEvent::CursorEnter(_) => {
                    mouse.lock();
                    true
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    mouse.handle_move(x, y);
                    true
                },
                _ => true,
            });
            self.display.mouse.unlock();

            entity.rotate(0., 0.5, 0.5);
            camera.move_camera(&mut self.display);


            shader.bind();
            shader.uniform_mat4("u_View", camera.get_view_matrix());
            self.renderer.prepare();
            self.renderer.render(&entity, &mut shader);
            shader.unbind();

            self.display.swap_buffers();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            display: Display::new(1280, 720, "hi"),
            renderer: MasterRenderer::new(1280, 720),
            loader: Loader::default(),
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.run();
}
