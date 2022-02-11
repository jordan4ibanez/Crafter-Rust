use std::ptr::{self, null};
use std::{collections::HashMap, os::raw::c_int};

use glfw::ffi::{glfwSetErrorCallback, glfwInit, glfwDefaultWindowHints, glfwWindowHint};

extern crate glfw;

use glfw::{ffi::*, Window, Context, WindowHint};


fn test () {

}

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    
    unsafe {

        if glfwInit() == 0 {
            println!("Unable to initialize GLFW!")
        } else {
            println!("GLFW initialized properly!")
        }

        
        glfwWindowHint(CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(CONTEXT_VERSION_MINOR, 2);

        glfwDefaultWindowHints();

        glfwWindowHint(OPENGL_PROFILE, OPENGL_COMPAT_PROFILE);

        glfwWindowHint(OPENGL_FORWARD_COMPAT, TRUE);

        let primary_monitor = glfwGetPrimaryMonitor();

        let width: &mut i32 = &mut 0;
        let height: &mut i32 = &mut 0;

        glfwGetMonitorPhysicalSize(primary_monitor, width, height);



        //let title: *const i8 = 5 as *const i8;


        //let test = window.window_ptr();

        //let handle = glfwCreateWindow(*width, *height, title, primary_monitor, window);

    }

    window.set_should_close(false);

    while !window.should_close() {
        glfw.poll_events();
        println!("{}", window.should_close());
        unsafe {
            //glfwWindowShouldClose(window.get_wgl_context());
        }
    }
}
