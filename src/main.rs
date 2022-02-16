extern crate glfw;

mod graphics;
mod controls;
mod time;
mod game_debug;

use glam::Vec3;
use glfw::*;

use rand::{
    thread_rng, prelude::ThreadRng
};

use std::{
    sync::mpsc::Receiver,
    vec
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
            self,
            *
        },
        transformation::{
            self
        }, 
        camera::{
            self,
            Camera
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
            self,
            Time
        }
    },
    game_debug::chunk_mesh_creation,
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



    let mut color_test: f32 = 0.0;
    let mut go_up = true;




    let debug_texture: Texture = texture::new(path.to_string() + "/textures/dirt.png");

    
    let mut tranformation = transformation::new();

    let mut mouse: Mouse = mouse::new(&window);
    let mut keyboard: Keyboard = keyboard::new();

    let mut camera: Camera = camera::new();


    // main program loop
    while !window.should_close() {       

        unsafe {
            gl::ClearColor(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }

        test_shader_program.bind();

        glfw.poll_events();

        mouse.reset();

        // this is where all events are processed
        process_events(&mut glfw, &mut window, &events, &mut mouse, &mut keyboard);


        camera.on_tick(&keyboard, &mouse, delta as f32);

        // START fps debug

        let returned_value = time_object.count_fps(&glfw);

        if returned_value {

            window_title.push_str("FPS: ");
            window_title.push_str(&time_object.get_fps().to_string());

            window.set_title(&window_title);

            window_title.clear();
        }

        // END fps debug

        // START delta debug
        
        delta = time_object.calculate_delta(&glfw);

        /*
        if go_up {
            color_test += delta as f32;

            if color_test >= 1.0 {
                color_test = 1.0;
                go_up = false;
            }
        } else {
            color_test -= delta as f32;
            
            if color_test <= 0.0 {
                color_test = 0.0;
                go_up = true;
            }
        }
        */

        // assert_eq!(delta, delta);
        // println!("{}", delta);

        // END delta debug



        tranformation.reset_projection_matrix(&camera, window.get_size().0 as f32, window.get_size().1 as f32, 0.01, 1000.0);

        test_shader_program.set_uniform_mat4("projectionMatrix".to_string(), tranformation.get_projection_matrix());

        // color_test * 10.0 (as z position)
        // color_test * 90_f32.to_radians() (as y rotation)
        test_shader_program.set_uniform_mat4("modelViewMatrix".to_string(), tranformation.update_model_matrix(Vec3::new(0.0,0.0, -2.0), Vec3::new(0.0, 0.0, 0.0)));

    
        let texture_clone = texture::clone(&debug_texture);
        
        //debug_mesh.test();
        let debug_mesh: Mesh = chunk_mesh_creation::create_chunk_mesh(texture_clone, &mut randy);

        debug_mesh.render();

        debug_mesh.clean_up(false);


        test_shader_program.unbind();

        window.swap_buffers();

        // counter += 1;

        /*
        if counter >= 30_000 {
            println!("Your demo is over boi!");
            window.set_should_close(true);
        }
        */        
    }

    test_shader_program.clean_up();

    // debug_mesh.clean_up(true);

    // texture_test.clean_up();
    // texture_test2.clean_up();

    println!("Program exited successfully!");

}


fn debug_full_screen(glfw: &mut Glfw, window: &mut Window){ 
    glfw.with_primary_monitor(|glfw, m: Option<&Monitor> | {
        // must unwrap the safety chain
        let monitor_reference: &Monitor = m.unwrap();
        let video_mode_option: Option<VidMode> = monitor_reference.get_video_mode();
        let video_mode: VidMode = video_mode_option.unwrap();

        window.set_monitor(glfw::WindowMode::FullScreen(&monitor_reference), video_mode.width as i32 / 2, video_mode.height as i32 / 2, video_mode.width, video_mode.height, Some(video_mode.refresh_rate));

        glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
    });    
    println!("f11");
}


// event processing, keys, mouse, etc
fn process_events(glfw: &mut Glfw, window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, mouse: &mut Mouse, keyboard: &mut Keyboard) {
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
            glfw::WindowEvent::Key(Key::F11, _, Action::Press, _) => debug_full_screen(glfw, window),

            _ => ()
        }
    }
}