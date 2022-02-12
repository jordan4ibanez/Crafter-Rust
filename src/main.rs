use std::{sync::mpsc::Receiver, ffi::{CString, c_void}, mem, ptr};

extern crate glfw;

use cgmath::{Vector4, Matrix4, Vector3};
use gl::types::{GLfloat, GLsizeiptr, GLsizei};
use glfw::*;
use rand::{thread_rng, Rng};

use crate::{resource_loader::load_resource, shader_program::ShaderProgram};

mod time_object;
mod resource_loader;
mod shader_program;


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
    let mut returned_value: (bool, i32);
    let mut delta: f64 = 0.0;

    // window title - reused pointer
    let mut window_title: String = String::new();

    // let mut counter = 0;

    // gets current working directory
    let path = std::env::current_dir()
                                .unwrap()
                                .as_os_str()
                                .to_str()
                                .unwrap()
                                .to_owned();

    println!("Current Working Path: {}", path);


    let vertex_shader: String = load_resource(path.to_string() + "/shader_code/vertex_shader.vs");

    let fragment_shader: String = load_resource(path.to_string() + "/shader_code/fragment_shader.fs");

    let mut test_shader_program: ShaderProgram = shader_program::new(vertex_shader.clone(), fragment_shader.clone());
    test_shader_program.create_uniform("color".to_string());
    test_shader_program.create_uniform("pos".to_string());
    test_shader_program.test();



    // A LONG TEST


    let vao: u32 =  unsafe {
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
             0.5, -0.5, 0.0, // right
             0.0,  0.5, 0.0  // top
        ];
        let (mut VBO, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        vao
    };


    // END TEST




    


    let mut color_test: f32 = 0.0;
    let mut go_up = true;


    // main program loop
    while !window.should_close() {       

        unsafe {
            // this will probably give you a seizure
            //gl::ClearColor(randy.gen(), randy.gen(), randy.gen(), 1.0);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        test_shader_program.bind();

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

        if go_up {
            color_test += delta as f32 * 10.0;

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

        unsafe {

            let my_vector4f: Vector4<f32> = Vector4::new(color_test, color_test / 1.24, 0.0, color_test);

            test_shader_program.set_uniform_vec4("pos".to_string(), my_vector4f);

            let my_vector: Vector4<f32> = Vector4::new(color_test, 0.0,0.0,1.0);

            test_shader_program.set_uniform_vec4("color".to_string(), my_vector);

            gl::BindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized

            //let color_name = CString::new("color").unwrap();

            // let vertex_color_location = gl::GetUniformLocation(test_shader_program.get_program(), color_name.as_ptr());
            //let vertex_color_location: i32 = test_shader_program.get_uniform_location("color".to_string()).clone();

            //let mut test = 0.0;

            //gl::GetUniformfv(test_shader_program.get_program(), vertex_color_location, &mut test);

            //println!("{}", test);

            //gl::Uniform4f(vertex_color_location, 0.0, 1.0, 0.0, 1.0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // let ourColor = CString::new("ourColor").unwrap();
            // let vertexColorLocation = gl::GetUniformLocation(test_shader_program.get_program(), ourColor.as_ptr());
            // gl::Uniform4f(vertexColorLocation, 0.0, 1.0, 0.0, 1.0);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

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