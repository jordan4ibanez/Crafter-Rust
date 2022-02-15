use std::{ffi::c_void};

use gl::{
    types::GLint,
    TEXTURE_MIN_FILTER,
    RGBA
};
use stb_image_rust;

use super::resource_loader;

pub struct Texture {
    id: u32,
    width: i32,
    height: i32
}

impl Drop for Texture {
    fn drop(& mut self){
        self.clean_up();
    }
}

impl Texture {

    // this is only for debugging
    pub fn test(&self) {
        println!("-- BEGINNING TEXTURE TEST ---");
        println!("ID: {:#?}", self.id);
        println!("WIDTH: {}", self.width);
        println!("HEIGHT: {}", self.height);
        println!("-- END TEXTURE TEST --");
    }

    
    pub fn construct(&mut self, path: String) {
        
        let mut data = resource_loader::load_texture(path);

        // next we will use rust to hold the memory
        let mut computed: i32 = 0;
        let image: *mut u8;

        // calling to stbi unsafely
        unsafe {
            image = stb_image_rust::stbi_load_from_memory(
                data.as_mut_ptr(),
                data.len() as i32,
                &mut self.width,
                &mut self.height,
                &mut computed,
                stb_image_rust::STBI_rgb_alpha
            );
        }
        
        // do something with it
        self.id = self.create_gl_texture(image);

        // finally free the memory, this uses a special call
        unsafe {
            stb_image_rust::c_runtime::free(image);
        }
    }

    fn create_gl_texture(&self, texture: *mut u8) -> u32 {

        let mut texture_id = 0;

        unsafe {

            // create a new texture in the gpu
            gl::GenTextures(1, &mut texture_id);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // tell opengl how to unpack the rgba bytes, each compenent is 1 byte in size
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            // gl::LINEAR gives it a smoothened look like an n64 game
            // gl::NEAREST gives it a blocky look
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // do not repeat texture
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

            // this was here in the original code
            // Generate Mip Map
            //glGenerateMipmap(GL_TEXTURE_2D);

            // upload the texture data 
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width,
                self.height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture as *const u8 as *const c_void);
        }

        // texture_id
        texture_id
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn clean_up(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }

        drop(self);
    }
}

// constructor
pub fn new(texture_path: String) -> Texture {
    let mut returning_texture: Texture = Texture {
        id: 0,
        width: 0,
        height: 0,
    };
    returning_texture.construct(texture_path);

    returning_texture
}