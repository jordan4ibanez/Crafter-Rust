use std::{
    mem,
    ffi::c_void,
    ptr
};

use gl::types::{
    GLfloat,
    GLsizeiptr, GLsizei
};

pub struct MeshComponentSystem {
    current_index: u32, // how big this component system is
    vao_id:        Vec<u32>, // object ID in GPU memory
    vbo_id:        Vec<u32>, // float data in GPU memory
    ebo_id:        Vec<u32>, // indices data in GPU memory
    vertex_count:  Vec<i32>,
    texture_id:    Vec<u32>
}

impl MeshComponentSystem {

    // initializer
    pub fn init() -> Self{
        MeshComponentSystem {
            current_index: 0,
            vao_id:        vec![0, 0],
            vbo_id:        vec![0, 0],
            ebo_id:        vec![0, 0],
            vertex_count:  vec![0, 0],
            texture_id:    vec![0, 0],
        }
    }

    // returns the VAO ID that OpenGL needs to bind to
    pub fn new_mesh(&mut self, float_data: Vec<f32>, indices: Vec<u32>, texture_id: u32) -> u32 {
        self.construct(float_data, indices, texture_id)
    }

    fn grow(&mut self, current_vao_id: u32) {
        if current_vao_id >= self.current_index {
            self.vao_id.push(0);
            self.vbo_id.push(0);
            self.ebo_id.push(0);
            self.vertex_count.push(0);
            self.texture_id.push(0);
            self.current_index += 1;
            // println!("Mesh Component System is now: {}", self.current_index);
        }
    }


    // internal constructor
    // the improvement is this allows dynamic allocations
    pub fn construct(&mut self, input_float_data: Vec<f32>, input_indices: Vec<u32>, input_texture_id: u32) -> u32 {

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
        
        self.grow(computed_vao_id);
        
        self.vao_id[index] = computed_vao_id;
        self.vbo_id[index] = computed_vbo_id;
        self.ebo_id[index] = computed_ebo_id;
        self.vertex_count[index] = computed_vertex_count;
        self.texture_id[index] = input_texture_id;

        computed_vao_id
    }


    pub fn batch_hook_texture(&self, id: u32) {
        unsafe {
            // activate first texture bank
            gl::ActiveTexture(gl::TEXTURE0);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id[id as usize]);
        }
    }

    pub fn batch_render(&self, id: u32) {

        let usize_id: usize = id as usize;
        unsafe {
            // this is for debug, wireframe mode
            // gl::PolygonMode(gl::FRONT_AND_BACK,gl::LINE);

            // bind the mesh vertex array
            gl::BindVertexArray(self.vao_id[usize_id]);

            // draw the mesh
            gl::DrawElements(gl::TRIANGLES, self.vertex_count[usize_id], gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn render(&self, id: u32){
        unsafe {
            // activate first texture bank
            gl::ActiveTexture(gl::TEXTURE0);

            // bind the texture
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id[id as usize]);

            // bind the mesh vertex array
            gl::BindVertexArray(self.vao_id[id as usize]);

            // draw the mesh
            gl::DrawElements(gl::TRIANGLES, self.vertex_count[id as usize], gl::UNSIGNED_INT, ptr::null());

            // restore data - this is unneeded, as it's overwritten with every draw call
            // gl::BindVertexArray(0);
        }
    }

    pub fn delete(&mut self, id: u32, delete_texture: bool){
        let index: usize = id as usize;
        unsafe {

            // de-allocate the memory in the GPU            
            gl::DeleteVertexArrays(1, &self.vao_id[index]);
            gl::DeleteBuffers(1, &self.vbo_id[index]);
            gl::DeleteBuffers(1, &self.ebo_id[index]);            

            // delete internal texture if specified to
            if delete_texture {
                println!("remember to hook in the texture component system into here, or move it into here");
                //self.texture.clean_up();
            }

        }


        self.vao_id[index] = 0;
        self.vbo_id[index] = 0;
        self.ebo_id[index] = 0;
        self.vertex_count[index] = 0;
        self.texture_id[index] = 0;
        //self.texture_id.insert(index, texture_id);
    }
}