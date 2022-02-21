use std::collections::HashMap;

use glam::Vec3;
use glfw::Window;

use crate::{world::world::World};

use super::{gl_safety_wrappers, shader_program::{ShaderProgram}, transformation::{Transformation}, camera::{Camera}, mesh_component_system::MeshComponentSystem};



pub struct Renderer {
    shaders: HashMap<String, ShaderProgram>,
    transformation: Transformation,
    camera: Camera,
    render_distance: f32
}

impl Renderer {

    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            transformation: Transformation::new(),
            camera: Camera::new(),
            render_distance: 0.0
        }
    }

    pub fn set_render_distance(&mut self, distance: f32) {
        self.render_distance = distance;
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
    pub fn render(&mut self, mcs: &MeshComponentSystem, window: &Window, world: &World) {
        
        gl_safety_wrappers::clear_depth_and_color(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 1.0);
        // gl_safety_wrappers::clear_depth_and_color(113.0 / 255.0, 112.0 / 255.0, 114.0 / 255.0, 1.0);

    
        let default_shader = self.shaders.get("default").unwrap();

        default_shader.bind();

        self.transformation.reset_projection_matrix(&self.camera, window.get_size().0 as f32, window.get_size().1 as f32, 0.01, self.render_distance);

        default_shader.set_uniform_mat4("projection_matrix", self.transformation.get_projection_matrix());

        // default_shader.set_light_uniform("game_render_distance", self.render_distance);
    
        // begin batched render
        let mut batch_hook = false;

        let mut worker_pos_vec = Vec3::splat(0.0);
        let worker_rot_vec = Vec3::splat(0.0);

        for chunk in world.iter_map_sorted(self.camera.get_pos()) {
        // for chunk in world.iter_map(){
            match chunk.get_mesh_id(){
                Some(mesh_id) => {

                    worker_pos_vec.x = *&chunk.get_pos().x as f32 * 16.0;
                    worker_pos_vec.z = *&chunk.get_pos().y as f32 * 16.0;

                    default_shader.set_uniform_mat4(
                        "model_matrix", 
                        self.transformation.update_model_matrix(
                            worker_pos_vec,
                            worker_rot_vec
                        )
                    );

                    // inialize batch
                    if !batch_hook {
                        batch_hook = true;
                        mcs.batch_hook_texture(*mesh_id);
                    }

                    mcs.batch_render(*mesh_id);
                    
                },
                None => (),
            }
        }

        default_shader.unbind();
    }
}

