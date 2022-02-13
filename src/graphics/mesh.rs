use std::{mem, ffi::c_void, ptr};

use gl::types::{GLfloat, GLsizeiptr, GLsizei, GLint};

use super::texture::Texture;

// OOP in rust, don't yell at me plz
pub struct Mesh {
    vao_id: u32,
    pos_vbo_id: u32,
    color_vbo_id: u32,
    texture_vbo_id: u32,
    idx_vbo_id: u32,
    vertex_count: i32,
    texture: Texture
}


impl Mesh {
    // internal constructor
    // the improvement is this allows dynamic allocations
    pub fn construct(&mut self, positions: Vec<f32>, colors: Vec<f32>, indices: Vec<i32>, texture_coordinates: Vec<f32>, texture: Texture){

        unsafe { 

            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

            // the VAO is basically the master key for the GL object
            gl::GenVertexArrays(1, &mut self.vao_id);

            gl::BindVertexArray(self.vao_id);

            // position vbo - as index 0 in GL
            gl::GenBuffers(1, &mut self.pos_vbo_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.pos_vbo_id);

            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (positions.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                &positions.as_slice()[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());




            // color vbo - as index 1 in GL
            gl::GenBuffers(1, &mut self.color_vbo_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_vbo_id);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (colors.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &colors.as_slice()[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());




            // texture coordinates vbo - as index 2 in GL
            gl::GenBuffers(1, &mut self.texture_vbo_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_vbo_id);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (texture_coordinates.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &colors.as_slice()[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(2);

            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());


            // index (indices) vbo

            gl::GenBuffers(1, &mut self.idx_vbo_id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.idx_vbo_id);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                &indices.as_slice()[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);


            // next add the vertex count to it's int
            self.vertex_count = indices.len() as i32;
            

            self.texture = texture;

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

    pub fn clean_up(&mut self, delete_texture: bool){
        unsafe {
            // bind buffer 0
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // disable all previously enabled vertex attribution arrays
            gl::DisableVertexAttribArray(2);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(0);

            // clear the buffer data in gpu
            gl::DeleteBuffers(1, &mut self.pos_vbo_id);
            gl::DeleteBuffers(1, &mut self.color_vbo_id);
            gl::DeleteBuffers(1, &mut self.texture_vbo_id);
            gl::DeleteBuffers(1, &mut self.idx_vbo_id);

            // explicitly break the previous bindings
            gl::BindVertexArray(0);

            // delete the whole object
            gl::DeleteVertexArrays(1, &mut self.vao_id);


            // delete internal texture if specified to
            if delete_texture {
                self.texture.clean_up();
            }

        }
    }
}


pub fn new() {

}