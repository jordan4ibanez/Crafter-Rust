use std::sync::mpsc::Receiver;

use glfw::{

    Glfw,
    WindowHint,
    OpenGlProfileHint,
    WindowEvent,
    Context,
    Monitor,
    VidMode

};

// utility file

// this is just a lonely set up file to clean up main()

pub fn set_up_glfw(glfw: &mut Glfw) -> (glfw::Window, Receiver<(f64, WindowEvent)>){

    // redundantly instatialize defaul hints in case of bad drivers
    glfw.default_window_hints();

    // base profile
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // base version
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(2));

    // allow driver optimizations
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));


    // the actual glfw window creation
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
    window.set_scroll_polling(true);
    

    // this ignores hardware acceleration
    window.set_raw_mouse_motion(true);


    window.set_cursor_enter_polling(true);
    
    window.set_cursor_pos_polling(true);

    // enable adaptive vsync
    glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

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

    (window, events)
    
}