use std::ptr::{self, null};
use std::{collections::HashMap, os::raw::c_int};

use rand::{thread_rng, Rng};

use glfw::ffi::{glfwSetErrorCallback, glfwInit, glfwDefaultWindowHints, glfwWindowHint, glfwGetPrimaryMonitor, glfwGetMonitorPhysicalSize};

extern crate glfw;

use glfw::*;
use winit::monitor;


fn main() {

    // initalize glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();    

    // create glfw window
    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    println!("GLFW window initialized properly!");

    // redundantly instatialize defaul hints in case of bad drivers
    glfw.default_window_hints();

    // base profile
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // base version
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(2));

    // allow driver optimizations
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    
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

    // load the opengl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // remove vsync
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // debug
    println!("Window Resolution: {} , {}", monitor_size.0, monitor_size.1);

    // A basic boolean for the window
    window.set_should_close(false);

    // a random number generator for debug
    let mut randy = thread_rng();


    // timer variables
    let mut previous_time: f64 = glfw.get_time();
    let mut current_time: f64 = glfw.get_time();
    let mut frame_count: i64 = 0;
    let mut window_title: String = String::new();


    while !window.should_close() {

        glfw.poll_events();


        // START fps debug

        current_time = glfw.get_time();

        frame_count += 1;

        if current_time - previous_time >= 1.0 {

            //println!("FPS: {}", frame_count);

            // push the raw data
            window_title.push_str("FPS: ");
            window_title.push_str(&frame_count.to_string());

            window.set_title(&window_title);

            window_title.clear();

            frame_count = 0;

            previous_time = current_time;
        }

        // END fps debug





        // START window title debug

        // let test = randy.gen_range(0..100000).to_string();

        // println!("{}", test);

        // window.set_title(&test);

        // END window title debug
        
        // function/event processing goes here

        window.swap_buffers();        
    }
}
