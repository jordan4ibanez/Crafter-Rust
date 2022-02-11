use std::ptr::{self, null};
use std::{collections::HashMap, os::raw::c_int};

use glfw::ffi::{glfwSetErrorCallback, glfwInit, glfwDefaultWindowHints, glfwWindowHint, glfwGetPrimaryMonitor, glfwGetMonitorPhysicalSize};

extern crate glfw;

use glfw::*;
use winit::monitor;


fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();    

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    println!("GLFW initialized properly!");

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

    println!("{} , {}", monitor_size.0, monitor_size.1);

    window.set_should_close(false);

    while !window.should_close() {
        glfw.poll_events();
        
        
    }
}
