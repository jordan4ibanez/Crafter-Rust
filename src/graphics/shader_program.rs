use std::{collections::HashMap, ffi::CString, ptr};
use gl::types::{GLint, GLchar};
use glam::{Vec4, Vec3, Mat4};

use super::resource_loader;

// "class fields"
pub struct ShaderProgram {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
    // the original java code should have been using hashmap
    uniforms: HashMap<String, i32>
}

// "class methods"
impl ShaderProgram {

    pub fn new(vertex_code_path: &str, fragment_code_path: &str) -> Self {

        // we must create a mutable version of this object
        let mut shader_program: Self = Self {
            program_id: 0,
            vertex_shader_id: 0,
            fragment_shader_id: 0,
            uniforms: HashMap::new(),
        };
    
        unsafe {
            shader_program.program_id = gl::CreateProgram();
        }
    
        shader_program.uniforms = HashMap::new();
    
        let vertex_code: String = resource_loader::load_resource(vertex_code_path.to_string());
    
        shader_program.vertex_shader_id = shader_program.create_vertex_shader(vertex_code);
    
        let fragment_code: String = resource_loader::load_resource(fragment_code_path.to_string());
    
        shader_program.fragment_shader_id = shader_program.create_fragment_shader(fragment_code);
    
        shader_program.link();
    
        shader_program
    }

    // this isn't in the original, it's just for me to test
    pub fn test(&self) {
        println!("----STARTING SHADER OBJECT TEST---");
        println!("PROGRAM ID: {}", self.program_id);
        println!("VERT ID: {}", self.vertex_shader_id);
        println!("FRAG ID: {}", self.fragment_shader_id);
        println!("UNIFORMS: {:#?}", self.uniforms);
        println!("---SHADER OBJECT TEST HAS ENDED---");
    }

    // the unsafe is wrapped around a safe
    // this is so you don't have to use unsafe blocks everywhere
    pub fn create_uniform(&mut self, uniform_name: &str) {
        unsafe {
            // convert name to C string
            let c_string: CString = CString::new(uniform_name.clone().as_bytes()).unwrap();

            // get location
            let uniform_location: i32 = gl::GetUniformLocation(self.program_id, c_string.as_ptr());

            self.uniforms.insert(uniform_name.to_string(), uniform_location);

            // manual memory drop to ensure no memory leaks
            drop(c_string);

            if uniform_location < 0 {
                panic!("COULD NOT CREATE UNIFORM!");
            }
        }
    }

    pub fn get_uniform_location(&self, uniform_name: &str) -> &i32 {
        self.uniforms.get(uniform_name).expect("TRIED TO GET A UNIFORM THAT DOES NOT EXIST!")
    }

    pub fn get_program(&self) -> u32 {
        self.program_id
    }

    // cannot overload so the name will end with the value
    pub fn set_uniform_mat4(&self, uniform_name: &str, value: Mat4) {
        let location: &i32 = self.uniforms.get(uniform_name).expect("COULD NOT LOAD UNIFORM! (M<F32>)");
        // todo: error handling
        unsafe {
            gl::UniformMatrix4fv(*location, 1, gl::FALSE, &value.to_cols_array()[0]);
        }


        // println!("{:#?}", &mut value.col(0)[0] as *const f32);
    }

    pub fn set_light_uniform(&self, uniform_name: &str, value: f32) {
        let location: &i32 = self.uniforms.get(uniform_name).expect("COULD NOT LOAD UNIFORM! (F32)");
        // todo: error handling
        unsafe {
            gl::Uniform1f(*location, value);
        }
    } 

    pub fn set_uniform_i32(&self, uniform_name: &str, value: i32) {
        let location: &i32 = self.uniforms.get(uniform_name).expect("COULD NOT LOAD UNIFORM! (I32)");
        // todo: error handling
        unsafe {
            gl::Uniform1i(*location, value);
        }
    }

    pub fn set_uniform_vec4(&self, uniform_name: &str, value: Vec4) {
        let location: &i32 = self.uniforms.get(uniform_name).expect("COULD NOT LOAD UNIFORM! (Vec4)");
        unsafe {            
            // todo: error handling
            gl::Uniform4f(*location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_uniform_vec3(&self, uniform_name: &str, value: Vec3) {      
        let location: &i32 = self.uniforms.get(uniform_name).expect("COULD NOT LOAD UNIFORM! (Vec3)");
        unsafe {            
            // todo: error handling
            gl::Uniform3f(*location, value.x, value.y, value.z);
        }  
    }

    pub fn create_vertex_shader(&self, vertex_shader_code: String) -> u32 {
        self.create_shader(vertex_shader_code, gl::VERTEX_SHADER)
    }

    pub fn create_fragment_shader(&self, fragment_shader_code: String) -> u32 {
        self.create_shader(fragment_shader_code, gl::FRAGMENT_SHADER)
    }

    // here is where the brunt of the errors can happen
    pub fn create_shader(&self, shader_code: String, shader_type: u32) -> u32 {
        let shader_id: u32;

        unsafe {
            shader_id = gl::CreateShader(shader_type);

            // must convert the shader into C code
            let c_str_vert = CString::new(shader_code.clone().as_bytes()).unwrap();

            gl::ShaderSource(shader_id, 1, &c_str_vert.as_ptr(), ptr::null());

            gl::CompileShader(shader_id);

            let mut success = false as GLint;

            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            
            if success != gl::TRUE as GLint {
                let mut info_log = vec![0; 512];

                gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);

                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n\n {}", String::from_utf8(info_log).unwrap());
                panic!("ERROR COMPILING SHADER!");
            }

            gl::AttachShader(self.program_id, shader_id);

            // then drop the value in case of a memory leak
            drop(c_str_vert);

        }

        // finally return the shader id
        shader_id
    }

    // this is pretty much one giant unsafe block hidden behind a safe method
    pub fn link(&self) {
        unsafe {

            // let success be mutable
            let mut success = gl::FALSE as GLint;

            gl::LinkProgram(self.program_id);

            // check it
            gl::GetProgramiv(self.program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                panic!("COULD NOT LINK SHADER!")
            }

            // delete the shaders, they are now one in the main shader program
            gl::DeleteShader(self.vertex_shader_id);
            gl::DeleteShader(self.fragment_shader_id);

            // now validate the code in the gpu
            gl::ValidateProgram(self.program_id);

            gl::GetProgramiv(self.program_id, gl::VALIDATE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                panic!("SHADER PROGRAM COULD NOT BE VALIDATED!");
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn clean_up(&self) {
        self.unbind();

        unsafe {
            gl::DeleteProgram(self.program_id);
        }

        // remove self from memory
        drop(self);
    }
}