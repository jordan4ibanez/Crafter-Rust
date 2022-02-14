use std::{sync::mpsc::Receiver, ffi::{CString, c_void}, mem, ptr, vec};

extern crate glfw;

mod graphics;
mod time_object;
mod resource_loader;

// use cgmath::{Vector4, Matrix4, Vector3};

//use glam::Vec4;
use glfw::*;
use rand::{thread_rng};

use crate::{
    resource_loader::load_resource,
    graphics::{
        shader_program::{
            ShaderProgram,
            self
        },
        texture::{self},
        mesh::{self, *},
        transformation::{self}
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

    let boi = transformation::new();

    // glfw initialization and configuration

    // initalize glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // redundantly instatialize defaul hints in case of bad drivers
    glfw.default_window_hints();

    // base profile
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // base version
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(2));

    // allow driver optimizations
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // create glfw window
    let (mut window, events) = glfw.create_window(300, 300, "FPS: 0", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

    println!("GLFW window initialized properly!");
    
    // get primary monitor and size
    let mut monitor_size: (u32, u32) = (0, 0);

    glfw.with_primary_monitor(|_, m: Option<&Monitor> | {

        // must unwrap the safety chain
        let monitor_reference: &Monitor = m.unwrap();
        let video_mode_option: Option<VidMode> = monitor_reference.get_video_mode();
        let video_mode: VidMode = video_mode_option.unwrap();

        // finally gotten the hard values
        monitor_size = (video_mode.width, video_mode.height);
        
        // drop everything just in case memory leak
        drop(monitor_reference);
        drop(video_mode_option);
        drop(video_mode);

    });

    
    // make window size half the resolution
    window.set_size(monitor_size.0 as i32 / 2, monitor_size.1 as i32 / 2);

    // center the window
    window.set_pos((monitor_size.0 as i32 - (monitor_size.0 as i32 / 2)) / 2, (monitor_size.1 as i32 - (monitor_size.1 as i32 / 2)) / 2);

    // make context current
    window.make_current();

    // enable internal C calls
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    //window.set_raw_mouse_motion(true);
    window.set_cursor_enter_polling(true);
    println!("\n A NOTE:\nYou can utilize set_cursor_enter_polling to save resources on loss of os focus\n");
    window.set_cursor_pos_polling(true);


    // remove vsync
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // load the opengl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // debug
    println!("Window Resolution: {} , {}", monitor_size.0, monitor_size.1);


    // this needs to be wrapped into a window object
    unsafe {
        gl::Viewport(0,0, monitor_size.0 as i32 / 2, monitor_size.1 as i32 / 2)
    }

    // A basic boolean for the window
    window.set_should_close(false);

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
        load_resource(path.to_string() + "/shader_code/vertex_shader.vs"),
        load_resource(path.to_string() + "/shader_code/fragment_shader.fs"));
    test_shader_program.create_uniform("projectionMatrix".to_string());
    test_shader_program.create_uniform("modelViewMatrix".to_string());
    test_shader_program.test();



    let mut color_test: f32 = 0.0;
    let mut go_up = true;


    let debug_mesh: Mesh = debug_mesh(&path);


    let mut tranformation = transformation::new();


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
        process_events(&mut window, &events);


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


        test_shader_program.set_uniform_mat4("modelViewMatrix".to_string(), tranformation.get_view_matrix());
        
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
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    // iterate events
    for (_, event) in glfw::flush_messages(events) {

        // println!("{:?}", event);

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

            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => println!("SPACEY!"),

            _ => {}
        }
    }
}