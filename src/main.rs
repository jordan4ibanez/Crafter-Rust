use std::sync::mpsc::Receiver;

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

    // enable internal C calls
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    //window.set_raw_mouse_motion(true);
    window.set_cursor_enter_polling(true);
    println!("You can utilize set_cursor_enter_polling to save resources on loss of os focus");
    window.set_cursor_pos_polling(true);


    // remove vsync
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // load the opengl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // debug
    println!("Window Resolution: {} , {}", monitor_size.0, monitor_size.1);

    // A basic boolean for the window
    window.set_should_close(false);

    // a random number generator for debug
    // let mut randy = thread_rng();

    // fps counter object
    let mut time_object = time_object::new(&glfw);
    // inlined cache vars
    let mut returned_value: (bool, i32);
    let mut delta: f64 = 0.0;

    // window title - reused pointer
    let mut window_title: String = String::new();

    // let mut counter = 0;


    // main program loop
    while !window.should_close() {

        unsafe {
            // this will probably give you a seizure
            //gl::ClearColor(randy.gen(), randy.gen(), randy.gen(), 1.0);

            gl::ClearColor(0.5,0.5,0.5,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();

        // this is where all events are processed
        process_events(&mut window, &events);


        // START fps debug

        returned_value = time_object.count_fps(&glfw);
        
        if returned_value.0 {

            window_title.push_str("FPS: ");
            window_title.push_str(&returned_value.1.to_string());

            window.set_title(&window_title);

            window_title.clear();
        }

        // END fps debug

        // START delta debug
        
        delta = time_object.calculate_delta(&glfw);

        // assert_eq!(delta, delta);
        // println!("{}", delta);

        // END delta debug

        window.swap_buffers();

        // counter += 1;

        /*
        if counter >= 30_000 {
            println!("Your demo is over boi!");
            window.set_should_close(true);
        }
        */
    }
}

// event processing, keys, mouse, etc
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    // iterate events
    for (_, event) in glfw::flush_messages(events) {

        println!("{:?}", event);

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