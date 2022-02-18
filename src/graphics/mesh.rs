use std::{
    mem,
    ffi::c_void,
    ptr
};

use gl::types::{
    GLfloat,
    GLsizeiptr, GLsizei
};

use super::texture::Texture;

pub struct Mesh {
    vao_id: u32, // object ID in GPU memory
    vbo_id: u32, // float data in GPU memory
    ebo_id: u32, // indices data in GPU memory
    vertex_count: i32,
    texture: Texture
}

impl Mesh {

    // constructor
    pub fn new(float_data: Vec<f32>, indices: Vec<u32>, texture: Texture) -> Self {
        let mut returning_mesh: Self = Self {
            vao_id: 0,
            vbo_id: 0,
            ebo_id: 0,
            vertex_count: 0,
            texture
        };
        
        returning_mesh.construct(float_data, indices);

        returning_mesh
    }

    // debug for prototyping meshes
    pub fn test(&self) {
        println!("---BEGIN MESH TEST--");
        println!("VAO: {}", self.vao_id);
        println!("VBO: {}", self.vbo_id);
        println!("EBO: {}", self.ebo_id);
        println!("V COUNT: {}", self.vertex_count);
        self.texture.test();
        println!("---END MESH TEST---");
    }

    // internal constructor
    // the improvement is this allows dynamic allocations
    pub fn construct(&mut self, float_data: Vec<f32>, indices: Vec<u32>) {

        unsafe {

            gl::GenVertexArrays(1, &mut self.vao_id);
            gl::GenBuffers(1, &mut self.vbo_id);
            gl::GenBuffers(1, &mut self.ebo_id);

            gl::BindVertexArray(self.vao_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (float_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &float_data[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
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
            

            // next add the vertex count to it's int
            self.vertex_count = indices.len() as i32;            

        }
    }


    pub fn batch_hook_texture(&self) {
        unsafe {
            // activate first texture bank
            gl::ActiveTexture(gl::TEXTURE0);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, self.texture.get_id());
        }
    }

    pub fn batch_render(&self) {
        unsafe {
            // this is for debug, wireframe mode
            // gl::PolygonMode(gl::FRONT_AND_BACK,gl::LINE);

            // bind the mesh vertex array
            gl::BindVertexArray(self.vao_id);

            // draw the mesh
            gl::DrawElements(gl::TRIANGLES, self.vertex_count, gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn render(&self){
        unsafe {
            // activate first texture bank
            gl::ActiveTexture(gl::TEXTURE0);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, self.texture.get_id());

            // bind the mesh vertex array
            gl::BindVertexArray(self.vao_id);

            // draw the mesh
            gl::DrawElements(gl::TRIANGLES, self.vertex_count, gl::UNSIGNED_INT, ptr::null());

            // restore data - this is unneeded, as it's overwritten with every draw call
            // gl::BindVertexArray(0);
        }
    }

    pub fn clean_up(&self, delete_texture: bool){
        unsafe {

            // de-allocate the memory in the GPU
            gl::DeleteVertexArrays(1, &self.vao_id);
            gl::DeleteBuffers(1, &self.vbo_id);
            gl::DeleteBuffers(1, &self.ebo_id);            

            // delete internal texture if specified to
            if delete_texture {
                self.texture.clean_up();
            }

        }
    }
}