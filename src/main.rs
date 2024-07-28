
use std::{sync::mpsc, time::Duration};

use glfw::{Action, Key};
use nalgebra_glm as glm;
use rust_voxel::{
    camera::Camera, display::Display, entity::Entity, loader::{Loader, RawModel}, model::Model, renderer::MasterRenderer, shader::Shader
};

pub enum TerrainGenMsg {
    UpdateTerrain(Vec<Entity>),
    UpdateCamera(Camera),
    CloseTerrainGen,
}

pub struct Game {
    display: Display,
    renderer: MasterRenderer,
    loader: Loader,
    camera: Camera,
    cube_model: RawModel,
}

impl Game {
    pub fn run(&mut self) {
        let texture = self.loader.load_texture("res/img/dirt.png");

        let model = Model {
            data: self.cube_model.clone(),
            texture,
        };

        let mut entities: Vec<Entity> = vec![];


        let mut shader = Shader::from_files("default.vert.glsl", "default.frag.glsl").unwrap();
        shader.bind();
        shader.uniform_vec3("u_Color", nalgebra_glm::vec3(0.0, 1.0, 0.0));

        let (to_gen_tx, to_gen_rx) = mpsc::channel::<TerrainGenMsg>();
        let (from_gen_tx, from_gen_rx) = mpsc::channel::<TerrainGenMsg>();

        let camera = self.camera.clone();
        let _ = std::thread::spawn(move || {
            let mut camera = camera;
            let mut entities: Vec<Entity> = vec![];
            let mut entities_changed = false;

            loop {
                match to_gen_rx.try_recv() {
                    Ok(TerrainGenMsg::CloseTerrainGen) => break,
                    Ok(TerrainGenMsg::UpdateCamera(new_camera)) => {
                        camera = new_camera;
                    }
                    _ => {},
                }

                for x in (camera.position.x - 10.) as i64..(camera.position.x + 10.) as i64 {
                    for z in (camera.position.z - 10.) as i64..(camera.position.z + 10.) as i64 {
                        if entities.iter().find(|&e| e.position.x == x as f32 && e.position.z == z as f32).is_none() {
                            entities.push(Entity::new(&model, glm::vec3(x as f32, 0., z as f32), (0., 0., 0.), 1.0));
                            entities_changed = true;
                        }
                    }
                }

                if entities_changed {
                    from_gen_tx.send(TerrainGenMsg::UpdateTerrain(entities.clone())).unwrap();
                    entities_changed = false;
                }
            }
        });

        while !self.display.should_close() {
            if let Ok(TerrainGenMsg::UpdateTerrain(new_entities)) = from_gen_rx.try_recv() {
                entities = new_entities 
            }

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

            self.camera.move_camera(&mut self.display);

            to_gen_tx.send(TerrainGenMsg::UpdateCamera(self.camera.clone())).unwrap();


            shader.bind();
            shader.uniform_mat4("u_View", self.camera.get_view_matrix());
            self.renderer.prepare();
            for entity in &entities {
                self.renderer.render(entity, &mut shader);
            }
            shader.unbind();

            self.display.swap_buffers();
        }



        to_gen_tx.send(TerrainGenMsg::CloseTerrainGen).unwrap();
    }
}

impl Default for Game {
    fn default() -> Self {
        let display = Display::new(1280, 720, "hi");
        let mut loader = Loader::default();

        #[rustfmt::skip]
        let cube_model = loader.load_to_vao(
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
        Self {
            display,
            loader,
            cube_model,
            renderer: MasterRenderer::new(1280, 720),
            camera: Camera::default(),
        }
    }
}

fn main() {
    let mut game = Game::default();
    game.run();
}
