extern crate glfw;

mod graphics;
mod controls;
mod time;
mod game_debug;
mod world;

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
        }, render::{self, Renderer}
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
        world::{
            *,
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

    // window title - reused pointer
    let mut window_title: String = String::new();

    println!("Current Working Path: {}", path);

    let debug_texture: Texture = texture::new(path.to_string() + "/textures/dirt.png");    

    let mut mouse: Mouse = mouse::new(&window);
    let mut keyboard: Keyboard = keyboard::new();

    // construct the renderer
    let mut renderer: Renderer = render::new();
    let mut default_shader: ShaderProgram = shader_program::new(
        path.to_string() + "/shader_code/vertex_shader.vs",
        path.to_string() + "/shader_code/fragment_shader.fs"
    );
    default_shader.create_uniform("projectionMatrix".to_string());
    default_shader.create_uniform("modelViewMatrix".to_string());
    default_shader.test();
    renderer.add_shader_program("default".to_string(), default_shader);


    let mut window_variables: WindowVariables = window_variables::new();

    let mut world: World = world::world::new();


    let mut debug_x = -32;
    let mut debug_y = -32;

    let mut continue_debug = true;
    


    // main program loop
    while !window.should_close() {


        if continue_debug {
            
            let generated_chunk: Chunk = world::chunk::new(debug_x, debug_y);
            world.add(generated_chunk);

            let mesh: Mesh = chunk_mesh_creation::create_chunk_mesh(texture::clone(&debug_texture), &mut randy);

            world.get_chunk_mut(debug_x.to_string() + " " + &debug_y.to_string()).set_mesh(mesh);

            debug_x += 1;

            if debug_x > 32 {
                debug_x = -32;

                debug_y += 1;

                if debug_y > 32 {
                    continue_debug = false;
                    println!("DONE!");
                }
            }
        }

        let delta: f64 = time_object.calculate_delta(&glfw);

        glfw.poll_events();

        mouse.reset();

        // this is where all events are processed
        process_events(&mut glfw, &mut window, &events, &mut mouse, &mut keyboard, &mut window_variables);


        renderer.get_camera_mut().on_tick(&keyboard, &mouse, delta as f32);

        renderer.render(&window, &world);


        let returned_value = time_object.count_fps(&glfw);

        if returned_value {
            window_title.push_str("FPS: ");
            window_title.push_str(&time_object.get_fps().to_string());
            window.set_title(&window_title);
            window_title.clear();
        }

        window.swap_buffers();
     
    }

    world.clean_up();
    renderer.clean_up();

    debug_texture.clean_up();

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