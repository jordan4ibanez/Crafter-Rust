use std::collections::HashMap;

use glam::Vec3;
use glfw::Window;

use crate::{world::world::World};

use super::{gl_safety_wrappers, shader_program::{ShaderProgram}, transformation::{Transformation}, camera::{Camera}};

pub struct Renderer {
    shaders: HashMap<String, ShaderProgram>,
    transformation: Transformation,
    camera: Camera
}

impl Renderer {

    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            transformation: Transformation::new(),
            camera: Camera::new()
        }
    }

    pub fn add_shader_program(&mut self, shader_name: &str, shader_program: ShaderProgram) {
        self.shaders.insert(shader_name.to_string(), shader_program);
    }

    pub fn get_shader_program(&mut self, shader_name: String) -> &ShaderProgram {
        self.shaders.get(&shader_name).unwrap()
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn clean_up(&mut self) {
        self.shaders.values().into_iter().for_each( | shader: &ShaderProgram | {
            shader.clean_up();
        });
    }

    // this is a test
    pub fn render(&mut self, window: &Window, world: &World) {
        
        gl_safety_wrappers::clear_depth_and_color(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 1.0);
    
        let default_shader = self.shaders.get("default").unwrap();

        default_shader.bind();

        self.transformation.reset_projection_matrix(&self.camera, window.get_size().0 as f32, window.get_size().1 as f32, 0.01, 1000.0);

        default_shader.set_uniform_mat4("projection_matrix".to_string(), self.transformation.get_projection_matrix());
    

        // begin batched render
        let mut batch_hook = false;

        for chunk in world.iter_map() {
            match chunk.get_mesh(){
                Some(mesh) => {
                    default_shader.set_uniform_mat4(
                        "model_view_matrix".to_string(), 
                        self.transformation.update_model_matrix(
                            Vec3::new(*&chunk.get_pos().x as f32 * 16.0,0.0, *&chunk.get_pos().y as f32 * 16.0), 
                            Vec3::new(0.0, 0.0, 0.0)
                        )
                    );

                    // inialize batch
                    if !batch_hook {
                        batch_hook = true;
                        mesh.batch_hook_texture();
                    }

                    mesh.batch_render();
                    
                },
                None => (),
            }
        }
        // let texture_clone = texture::clone(texture_map);
        
        // let debug_mesh: Mesh = chunk_mesh_creation::create_chunk_mesh(texture_clone, randy);

        // debug_mesh.render();

        // debug_mesh.clean_up(false);

        default_shader.unbind();
    }
}

