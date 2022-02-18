extern crate glfw;

mod graphics;
mod controls;
mod time;
mod game_debug;
mod world;

use glfw::*;

use graphics::window_controls::toggle_full_screen;
use perlin2d::PerlinNoise2D;
use rand::ThreadRng;

use std::{
    sync::mpsc::Receiver
};

use crate::{
    graphics::{
        shader_program::{
            ShaderProgram
        },
        texture::{
            Texture
        },
        mesh::{
            *
        },
        window_variables::{
            *
        }, render::{Renderer}, resource_loader::get_path_string
    },

    controls::{
        mouse::{
            Mouse
        },
        keyboard::{
            Keyboard
        }
    },

    time::{
        time_object::{
            Time
        }
    },
    game_debug::{chunk_mesh_creation, chunk_mesh_generator_queue::{self, ChunkMeshGeneratorQueue, MeshUpdate}},

    world::{
        chunk::{
            *
        },
        world::{
            *,
        }, biome_generator::gen_biome
    }
};

fn main() {

    // glfw initialization and configuration

    // initalize glfw
    let mut glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // borrow and mutate glfw
    // return created glfw window
    let (mut window, events) = graphics::set_up::set_up_glfw(&mut glfw);


    // testing of 3D camera
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    let mut perlin: PerlinNoise2D = PerlinNoise2D::new(1, 0.5, 1.0, 1.0, 1.0, (10.0, 10.0), 0.5, 1213);
    let mut thread_rng: ThreadRng = rand::thread_rng();

    // fps counter object
    let mut time_object: Time = Time::new(&glfw);

    // window title - reused pointer
    let mut window_title: String = String::new();

    println!("Current Working Path: {}", get_path_string());

    let debug_texture: Texture = Texture::new("/textures/dirt.png");    

    let mut mouse: Mouse = Mouse::new(&window);
    let mut keyboard: Keyboard = Keyboard::new();

    // construct the renderer
    let mut renderer: Renderer = Renderer::new();
    let mut default_shader: ShaderProgram = ShaderProgram::new(
         "/shader_code/vertex_shader.vs",
        "/shader_code/fragment_shader.fs"
    );
    default_shader.create_uniform("projectionMatrix");
    default_shader.create_uniform("modelViewMatrix");
    default_shader.test();
    renderer.add_shader_program("default".to_string(), default_shader);


    let mut window_variables: WindowVariables = WindowVariables::new();

    let mut world: World = World::new();

    const RENDER_DISTANCE: i32 = 30;

    let mut debug_x = -RENDER_DISTANCE;
    let mut debug_y = -RENDER_DISTANCE;

    let mut continue_debug = true;

    let mut chunk_mesh_generator_queue: ChunkMeshGeneratorQueue = ChunkMeshGeneratorQueue::new();
    

    // main program loop
    while !window.should_close() {

        // here is testing for the logic of the chunk mesh generator queue
        {

            let mesh_update_option: Option<MeshUpdate> = chunk_mesh_generator_queue.pop_front();

            // does this update exist?
            match mesh_update_option {
                Some(mesh_update) => {
                    // add neighbors to queue if told to do so
                    if mesh_update.update_neighbors() {
                        chunk_mesh_generator_queue.batch_neighbor_update(mesh_update.get_x(), mesh_update.get_z());
                    }

                    let mesh: Option<Mesh> = chunk_mesh_creation::create_chunk_mesh(&world, mesh_update.get_x(), mesh_update.get_z(), Texture::clone(&debug_texture));
                    match mesh {
                        Some(unwrapped_mesh) => world.set_chunk_mesh(mesh_update.get_x().to_string() + " " + &mesh_update.get_z().to_string(), unwrapped_mesh),
                        None => (),
                    }
                },
                None => (),
            }

            
        }


        if continue_debug {
            
            let mut generated_chunk: Chunk = Chunk::new(debug_x, debug_y);

            gen_biome(&mut generated_chunk, &mut perlin, None);//Some(&mut thread_rng));

            world.add(generated_chunk);

            chunk_mesh_generator_queue.push_back(debug_x, debug_y, true);



            debug_x += 1;

            if debug_x > RENDER_DISTANCE {
                debug_x = -RENDER_DISTANCE;

                debug_y += 1;

                if debug_y > RENDER_DISTANCE {
                    continue_debug = false;
                    println!("DONE!");
                }
            }
        }

        let delta: f64 = time_object.calculate_delta(&glfw);

        glfw.poll_events();

        mouse.reset();

        // this is where all events are processed
        process_events(&mut glfw, &mut window, &events, &mut mouse, &mut keyboard, &mut window_variables);


        renderer.get_camera_mut().on_tick(&keyboard, &mouse, delta as f32);

        renderer.render(&window, &world);


        let returned_value = time_object.count_fps(&glfw);

        if returned_value {
            window_title.push_str("FPS: ");
            window_title.push_str(&time_object.get_fps().to_string());
            window.set_title(&window_title);
            window_title.clear();
        }

        window.swap_buffers();
     
    }

    world.clean_up();
    renderer.clean_up();

    debug_texture.clean_up();

    println!("Program exited successfully!");

}


// event processing, keys, mouse, etc
fn process_events(glfw: &mut Glfw, window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, mouse: &mut Mouse, keyboard: &mut Keyboard, window_variables: &mut WindowVariables) {
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
            glfw::WindowEvent::Key(Key::F11, _, Action::Press, _) => toggle_full_screen(glfw, window, window_variables),

            _ => ()
        }
    }
}