use std::ops::Mul;

use glam::{Mat4, Vec3, Quat};

// Remember that cgmath is basically just joml

// I will implement ortholinear rendering later
pub struct Transformation {
    projection_matrix: Mat4,
    model_matrix: Mat4,
    view_matrix: Mat4,
}

impl Transformation {
    pub fn reset_projection_matrix(&mut self, fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) {

        self.projection_matrix = Mat4::perspective_rh_gl(fov.to_radians(), width / height, z_near, z_far);


        let camera_rotation: (f32, f32) = (0.0, 0.0);

        
        self.view_matrix = Mat4::IDENTITY;
        
        // self.view_matrix *= Mat4::from_rotation_x(camera_rotation.0.to_radians());
        // self.view_matrix *= Mat4::from_rotation_y(camera_rotation.1.to_radians());
        

        let my_vector: Vec3 = Vec3::new(0.0, 0.0, -10.0);

        self.view_matrix = Mat4::from_translation(my_vector);


    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }
}


pub fn new() -> Transformation {

    let t = Transformation {
        projection_matrix: Mat4::IDENTITY,
        model_matrix: Mat4::IDENTITY,
        view_matrix: Mat4::IDENTITY
    };
    
    t
}