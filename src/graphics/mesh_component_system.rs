use std::{
    mem,
    ffi::c_void,
    ptr
};

use gl::types::{
    GLfloat,
    GLsizeiptr, GLsizei
};

use image::{ImageBuffer, Rgba};



use super::resource_loader::{create_image_buffer};

pub struct MeshComponentSystem {

    // mesh data
    mesh_index:   u32, // how big this component system is
    vao_id:       Vec<u32>, // object ID in GPU memory
    vbo_id:       Vec<u32>, // float data in GPU memory
    ebo_id:       Vec<u32>, // indices data in GPU memory
    vertex_count: Vec<i32>,
    mesh_texture: Vec<u32>,

    // texture data
    texture_index:  u32,
    texture_id:     Vec<u32>,
    texture_width:  Vec<u32>,
    texture_height: Vec<u32>
}

impl MeshComponentSystem {

    // initializer
    pub fn init() -> Self {
        MeshComponentSystem {
            // mesh data
            mesh_index:     0,
            vao_id:         vec![0, 0],
            vbo_id:         vec![0, 0],
            ebo_id:         vec![0, 0],
            vertex_count:   vec![0, 0],
            mesh_texture:   vec![0, 0],

            // texture data
            texture_index:  0,
            texture_id:     vec![0, 0],
            texture_width:  vec![0, 0],
            texture_height: vec![0, 0]
        }
    }


    // texture methods

    fn grow_texture(&mut self, current_texture_id: u32) {
        if current_texture_id >= self.texture_index {
            self.texture_id.push(0);
            self.texture_width.push(0);
            self.texture_height.push(0);
            self.texture_index += 1;
        }
    }

    pub fn new_texture(&mut self, texture_path: &str) -> u32 {
        self.construct_texture(texture_path)
    }

    pub fn construct_texture(&mut self, path: &str) -> u32 {

        let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = create_image_buffer(path);

        let image_ptr = image_buffer.as_ptr();

        let width: u32 = image_buffer.width();
        let height: u32 = image_buffer.height();
        
        // do something with it
        let id: u32 = self.create_gl_texture(image_ptr, width, height);

        // manually free - probably don't have to do this
        drop(image_ptr);

        self.grow_texture(id);

        let index: usize = id as usize;

        self.texture_id[index] = id;
        self.texture_width[index] = width;
        self.texture_height[index] = height;

        id
    }


    fn create_gl_texture(&self, texture: *const u8, width: u32, height: u32) -> u32 {

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
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture as *const c_void);
        }

        // texture_id
        texture_id
    }

    pub fn get_texture_width(&self, id: u32) -> u32 {
        self.texture_width[id as usize]
    }

    pub fn get_texture_height(&self, id: u32) -> u32 {
        self.texture_height[id as usize]
    }

    pub fn delete_texture(&mut self, id: u32) {
        unsafe {
            gl::DeleteTextures(1, &id);
        }

        let index: usize = id as usize;

        self.texture_id[index] = 0;
        self.texture_width[index] = 0;
        self.texture_height[index] = 0;
    }




    // mesh methods

    // returns the VAO ID that OpenGL needs to bind to
    pub fn new_mesh(&mut self, float_data: Vec<f32>, indices: Vec<u32>, texture_id: u32) -> u32 {
        self.construct_mesh(float_data, indices, texture_id)
    }

    fn grow_mesh(&mut self, current_vao_id: u32) {
        if current_vao_id >= self.mesh_index {
            self.vao_id.push(0);
            self.vbo_id.push(0);
            self.ebo_id.push(0);
            self.vertex_count.push(0);
            self.mesh_texture.push(0);
            self.mesh_index += 1;
        }
    }


    // internal constructor
    // the improvement is this allows dynamic allocations
    pub fn construct_mesh(&mut self, input_float_data: Vec<f32>, input_indices: Vec<u32>, input_texture_id: u32) -> u32 {

        let mut computed_vao_id:       u32 = 0;
        let mut computed_vbo_id:       u32 = 0;
        let mut computed_ebo_id:       u32 = 0;

        // we must first step into an unsafe block to talk to OpenGL through FFI
        unsafe {
            gl::GenVertexArrays(1, &mut computed_vao_id);

            gl::GenBuffers(1, &mut computed_vbo_id);
            gl::GenBuffers(1, &mut computed_ebo_id);

            gl::BindVertexArray(computed_vao_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, computed_vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (input_float_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &input_float_data[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, computed_ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (input_indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &input_indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW
            );

            /*
            stride is the width of the values in a single vertex
            Example in current production usage:

            pos:
            1.0, 1.0, 1.0
            color:
            0.0, 0.0, 0.0
            texture (texture mapping):
            0.5, 0.5

            So when it is interlaced it will look like this in memory:
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.5, 0.5

            So now we count:
            1    2    3    4    5    6    7    8
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.5, 0.5

            So therefore, the stride is 8
            */

            let stride = 8* mem::size_of::<GLfloat>() as GLsizei;

            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute - skip over the first 3 (0,1,2) values of the vertex data
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
            // texture attribute - skip over the first 6 (0,1,2,3,4,5) values of the vertex data
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);
        }

        // next add the vertex count to it's int
        let computed_vertex_count: i32 = input_indices.len() as i32;

        // step out of unsafe and push into self component sytem
        // this is inserting into vao_id mainly as a debug for now
        let index: usize = computed_vao_id as usize;
        
        self.grow_mesh(computed_vao_id);
        
        self.vao_id[index] = computed_vao_id;
        self.vbo_id[index] = computed_vbo_id;
        self.ebo_id[index] = computed_ebo_id;
        self.vertex_count[index] = computed_vertex_count;
        self.mesh_texture[index] = input_texture_id;

        computed_vao_id
    }


    pub fn batch_hook_texture(&self, id: u32) {
        unsafe {
            // activate first texture bank
            gl::ActiveTexture(gl::TEXTURE0);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, self.mesh_texture[id as usize]);
        }
    }

    pub fn batch_render(&self, id: u32) {

        let usize_id: usize = id as usize;

        unsafe {
            // this is for debug, wireframe mode
            // gl::PolygonMode(gl::FRONT_AND_BACK,gl::LINE);

            // bind the mesh vertex array
            gl::BindVertexArray(id);

            // draw the mesh
            gl::DrawElements(gl::TRIANGLES, self.vertex_count[usize_id], gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn render(&self, id: u32){
        unsafe {
            // activate first texture bank
            gl::ActiveTexture(gl::TEXTURE0);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, self.mesh_texture[id as usize]);

            // bind the mesh vertex array
            gl::BindVertexArray(id);

            // draw the mesh
            gl::DrawElements(gl::TRIANGLES, self.vertex_count[id as usize], gl::UNSIGNED_INT, ptr::null());

            // restore data - this is unneeded, as it's overwritten with every draw call
            // gl::BindVertexArray(0);
        }
    }

    pub fn delete_mesh(&mut self, id: u32, delete_texture: bool){
        let index: usize = id as usize;
        unsafe {

            // de-allocate the memory in the GPU            
            gl::DeleteVertexArrays(1, &self.vao_id[index]);
            gl::DeleteBuffers(1, &self.vbo_id[index]);
            gl::DeleteBuffers(1, &self.ebo_id[index]);            

            // delete internal texture if specified to
            if delete_texture {
                self.delete_texture(self.mesh_texture[index]);
            }

        }

        self.vao_id[index] = 0;
        self.vbo_id[index] = 0;
        self.ebo_id[index] = 0;
        self.vertex_count[index] = 0;
        self.mesh_texture[index] = 0;
    }
}