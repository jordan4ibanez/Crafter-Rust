extern crate glfw;

mod graphics;
mod controls;
mod time;
mod game_debug;
mod world;

use glam::Vec3;
use glfw::*;

use graphics::window_controls::toggle_full_screen;
use rand::{
    thread_rng, prelude::ThreadRng
};

use std::{
    sync::mpsc::Receiver
};

use crate::{
    graphics::{
        shader_program::{
            ShaderProgram,
            self
        },
        texture::{
            self, Texture
        },
        mesh::{
            *
        },
        transformation::{
            self
        }, 
        camera::{
            self,
            Camera
        },
        gl_safety_wrappers,
        window_variables::{
            *, self
        }
    },

    controls::{
        mouse::{
            self,
            Mouse
        },
        keyboard::{
            self,
            Keyboard
        }
    },

    time::{
        time_object::{
            self
        }
    },
    game_debug::chunk_mesh_creation,

    world::{
        chunk::{
            *
        },
        map::{
            *
        }
    }
};

fn main() {

    // gets current working directory
    let path: String = std::env::current_dir()
                                .unwrap()
                                .as_os_str()
                                .to_str()
                                .unwrap()
                                .to_owned();


    // glfw initialization and configuration

    // initalize glfw
    let mut glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // borrow and mutate glfw
    // return created glfw window
    let (mut window, events) = graphics::set_up::set_up_glfw(&mut glfw, &path);


    // testing of 3D camera
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // a random number generator for debug
    let mut randy: ThreadRng = thread_rng();

    // fps counter object
    let mut time_object = time_object::new(&glfw);
    // inlined cache vars
    let mut delta: f64 = 0.0;

    // window title - reused pointer
    let mut window_title: String = String::new();

    println!("Current Working Path: {}", path);

    let mut test_shader_program: ShaderProgram = shader_program::new(
        path.to_string() + "/shader_code/vertex_shader.vs",
        path.to_string() + "/shader_code/fragment_shader.fs"
    );

    test_shader_program.create_uniform("projectionMatrix".to_string());
    test_shader_program.create_uniform("modelViewMatrix".to_string());
    test_shader_program.test();


    let debug_texture: Texture = texture::new(path.to_string() + "/textures/dirt.png");

    
    let mut tranformation = transformation::new();

    let mut mouse: Mouse = mouse::new(&window);
    let mut keyboard: Keyboard = keyboard::new();

    let mut camera: Camera = camera::new();

    let mut window_variables: WindowVariables = window_variables::new();


    // main program loop
    while !window.should_close() {       

        gl_safety_wrappers::clear_depth_and_color(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 1.0);

        test_shader_program.bind();

        glfw.poll_events();

        mouse.reset();

        // this is where all events are processed
        process_events(&mut glfw, &mut window, &events, &mut mouse, &mut keyboard, &mut window_variables);


        camera.on_tick(&keyboard, &mouse, delta as f32);

        // START fps debug

        let returned_value = time_object.count_fps(&glfw);



        if returned_value {

            window_title.push_str("FPS: ");
            window_title.push_str(&time_object.get_fps().to_string());

            window.set_title(&window_title);

            window_title.clear();
        }

        
        delta = time_object.calculate_delta(&glfw);

        tranformation.reset_projection_matrix(&camera, window.get_size().0 as f32, window.get_size().1 as f32, 0.01, 1000.0);

        test_shader_program.set_uniform_mat4("projectionMatrix".to_string(), tranformation.get_projection_matrix());

        test_shader_program.set_uniform_mat4("modelViewMatrix".to_string(), tranformation.update_model_matrix(Vec3::new(0.0,0.0, -2.0), Vec3::new(0.0, 0.0, 0.0)));

    
        let texture_clone = texture::clone(&debug_texture);
        
        let debug_mesh: Mesh = chunk_mesh_creation::create_chunk_mesh(texture_clone, &mut randy);

        debug_mesh.render();

        debug_mesh.clean_up(false);

        test_shader_program.unbind();

        window.swap_buffers();
     
    }

    test_shader_program.clean_up();


    println!("Program exited successfully!");

}


// event processing, keys, mouse, etc
fn process_events(glfw: &mut Glfw, window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, mouse: &mut Mouse, keyboard: &mut Keyboard, window_variables: &mut WindowVariables) {
    // iterate events
    for (_, event) in glfw::flush_messages(events) {


        mouse.process_events(&event);

        keyboard.process_events(&event);

        // match event enums
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // update the gl viewport to match the new window size
                unsafe {
                    gl::Viewport(0,0, width, height)
                }
            }

            // close the window on escape
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            glfw::WindowEvent::Key(Key::F11, _, Action::Press, _) => toggle_full_screen(glfw, window, window_variables),

            _ => ()
        }
    }
}