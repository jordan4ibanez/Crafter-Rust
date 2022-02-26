use std::{sync::mpsc::Receiver, fs::File, io::BufReader};

use glfw::{

    Glfw,
    WindowHint,
    OpenGlProfileHint,
    WindowEvent,
    Context,
    Monitor,
    VidMode,
    Window,
    ffi::{
        GLFWimage,
        glfwSetWindowIcon
    }, PixelImage
};
use image::{ImageBuffer, Rgba};



use super::resource_loader::{create_image_buffer};

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

    set_window_icon(&mut window,"/cache/icon.png");

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
    // window.set_raw_mouse_motion(true);


    window.set_cursor_enter_polling(true);
    
    window.set_cursor_pos_polling(true);

    // enable adaptive vsync
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // load the opengl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // debug
    println!("Window Resolution: {} , {}", monitor_size.0, monitor_size.1);


    // A basic boolean for the window
    window.set_should_close(false);

    // set OpenGL variables
    unsafe {
        gl::Viewport(0,0, monitor_size.0 as i32 / 2, monitor_size.1 as i32 / 2);

        // depth testing
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);

        //back face culling
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);

        // Support for transparencies
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    }

    (window, events)
    
}

fn set_window_icon(window: &mut Window, path: &str) {

    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = create_image_buffer(path);
    
    let glfw_image: GLFWimage = GLFWimage {
        width: image_buffer.width() as i32,
        height: image_buffer.height() as i32,
        pixels: image_buffer.as_ptr()
    };

    let test: Vec<PixelImage> = Vec::new();

    window.set_icon_from_pixels(test);


    unsafe {
        glfwSetWindowIcon(window.window_ptr(), 1, &glfw_image as *const GLFWimage);
    }

    drop(glfw_image);

}