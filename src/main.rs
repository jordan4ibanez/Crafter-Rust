use rand::{thread_rng, Rng};

extern crate glfw;

use glfw::*;

mod time_object;


fn main() {


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

    // remove vsync
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // load the opengl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // debug
    println!("Window Resolution: {} , {}", monitor_size.0, monitor_size.1);

    // A basic boolean for the window
    window.set_should_close(false);

    // a random number generator for debug
    let mut randy = thread_rng();

    // fps counter object
    let mut fps_counter = time_object::new(&glfw);
    // inlined cache vars
    let mut returned_value: (bool, i32) = (false,0);

    // window title - reused pointer
    let mut window_title: String = String::new();

    let mut counter = 0;

    while !window.should_close() {

        unsafe {
            // this will probably give you a seizure
            //gl::ClearColor(randy.gen(), randy.gen(), randy.gen(), 1.0);

            gl::ClearColor(0.5,0.5,0.5,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();


        // START fps debug

        returned_value = fps_counter.count(&glfw);
        
        if returned_value.0 {

            window_title.push_str("FPS: ");
            window_title.push_str(&returned_value.1.to_string());

            window.set_title(&window_title);

            window_title.clear();
        }

        // END fps debug

        window.swap_buffers();

        counter += 1;

        if counter >= 30_000 {
            panic!("Your demo is over boi");
        }
    }
}
