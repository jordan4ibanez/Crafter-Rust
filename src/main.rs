extern crate glfw;

mod graphics;
mod time_object;
mod controls;

use glam::Vec3;
use glfw::*;

use rand::{
    thread_rng
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
            self
        },
        mesh::{
            self,
            *
        },
        transformation::{
            self
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
    }
};

fn debug_mesh(path: &str) -> Mesh {

    // this is the light attrib in crafter
    let colors: Vec<f32> = vec![
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    let indices: Vec<i32> = vec![
        //tri 1
		0, 1, 3,

		//tri 2
		3, 1, 2,
    ];

    let positions: Vec<f32> = vec![        
        -0.5,  0.5, -0.15, //top left
		-0.5, -0.5, -0.15, //bottom left
		 0.5, -0.5, -0.15, //bottom right
		 0.5,  0.5, -0.15, //top right
    ];

    let texture_coordinates: Vec<f32> = vec![
        1.0, 1.0, //bottom right
        0.0, 1.0, //bottom left
        0.0, 0.0, //top left
        1.0, 0.0, //top right
        
    ];

    let this_texture = texture::new(path.to_string() + "/textures/debug.png");

    let returning_mesh = mesh::new(
        positions,
        colors,
        indices,
        texture_coordinates,
        this_texture
    );

    returning_mesh
}


fn main() {

    // glfw initialization and configuration

    // initalize glfw
    let mut glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // borrow and mutate glfw
    // return created glfw window
    let (mut window, events) = graphics::set_up::set_up_glfw(&mut glfw);


    // testing of 3D camera
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // a random number generator for debug
    let mut randy = thread_rng();

    // fps counter object
    let mut time_object = time_object::new(&glfw);
    // inlined cache vars
    let mut delta: f64 = 0.0;

    // window title - reused pointer
    let mut window_title: String = String::new();

    // gets current working directory
    let path = std::env::current_dir()
                                .unwrap()
                                .as_os_str()
                                .to_str()
                                .unwrap()
                                .to_owned();

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


    let debug_mesh: Mesh = debug_mesh(&path);

    let mut tranformation = transformation::new();

    let mut mouse: Mouse = mouse::new();
    let mut keyboard: Keyboard = keyboard::new();


    // main program loop
    while !window.should_close() {       

        unsafe {
            // this will probably give you a seizure
            //gl::ClearColor(randy.gen(), randy.gen(), randy.gen(), 1.0);

            gl::ClearColor(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        test_shader_program.bind();

        glfw.poll_events();

        // this is where all events are processed
        process_events(&mut window, &events, &mut mouse, &mut keyboard);


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

        // assert_eq!(delta, delta);
        // println!("{}", delta);

        // END delta debug



        tranformation.reset_projection_matrix(60.0, window.get_size().0 as f32, window.get_size().1 as f32, 0.01, 1000.0);

        test_shader_program.set_uniform_mat4("projectionMatrix".to_string(), tranformation.get_projection_matrix());

        // color_test * 10.0 (as z position)
        // color_test * 90_f32.to_radians() (as y rotation)
        test_shader_program.set_uniform_mat4("modelViewMatrix".to_string(), tranformation.update_model_matrix(Vec3::new(0.0,0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)));


        
        //debug_mesh.test();
        debug_mesh.render();


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

// event processing, keys, mouse, etc
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, mouse: &mut Mouse, keyboard: &mut Keyboard) {
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

            _ => ()
        }
    }
}