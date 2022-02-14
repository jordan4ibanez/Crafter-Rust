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

        let mut transform: Mat4 = Mat4::IDENTITY;

        transform *= Mat4::perspective_rh_gl(fov.to_radians(), width / height, z_near, z_far);

        //transform = transform * Mat4::from_translation(Vec3::new(0.5, -0.5, 0.0));
        //transform = transform * Mat4::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 0.0);
        
        self.projection_matrix = transform;
        /*

        // where the GL window is "looking"
        self.projection_matrix *= Mat4::perspective_rh(
            fov.to_radians(),
            width / height,
            z_near,
            z_far
        );
        */


        // let camera_rotation = (0.0, 1.0);
        


        //let perspect = perspective(Rad(fov),width / height,z_near,z_far);

        //self.projection_matrix.swap_elements(perspect);

        //self.projection_matrix;

    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
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