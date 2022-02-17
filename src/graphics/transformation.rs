
use glam::{Mat4, Vec3};

use super::camera::Camera;

// Remember that glam is basically just joml

// I will implement ortholinear rendering later
pub struct Transformation {
    projection_matrix: Mat4,
    model_matrix: Mat4,
    view_matrix: Mat4, // <- this is an identity - reference Mat4 TODO: rename this to camera_matrix
}

impl Transformation {

    pub fn new() -> Self {
        Self {
            projection_matrix: Mat4::IDENTITY,
            model_matrix: Mat4::IDENTITY,
            view_matrix: Mat4::IDENTITY
        }
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }


    pub fn reset_projection_matrix(&mut self, camera: &Camera, width: f32, height: f32, z_near: f32, z_far: f32) {

        // this is the window of the game, the initial camera position and specifications
        self.projection_matrix = Mat4::perspective_rh_gl(camera.get_fov().to_radians(), width / height, z_near, z_far);

        self.view_matrix = Mat4::IDENTITY;

        self.view_matrix *= Mat4::from_axis_angle(Vec3::new(1.0,0.0,0.0), camera.get_rot().x.to_radians());
        self.view_matrix *= Mat4::from_axis_angle(Vec3::new(0.0,1.0,0.0), camera.get_rot().y.to_radians());

        // this is also inverted
        let my_vector: Vec3 = Vec3::new(-camera.get_pos().x, -camera.get_pos().y, -camera.get_pos().z);

        self.view_matrix *= Mat4::from_translation(my_vector);

    }

    // this version of the matrix check makes it so you can inline the function
    pub fn update_model_matrix(&mut self, pos: Vec3, rot: Vec3) -> Mat4 {
        
        // works from the view matrix
        self.model_matrix = Mat4::from(self.view_matrix);

        // pos

        self.model_matrix *= Mat4::from_translation(pos);

        // rotation

        self.model_matrix *= Mat4::from_axis_angle(Vec3::new(1.0,0.0,0.0), rot.x);
        self.model_matrix *= Mat4::from_axis_angle(Vec3::new(0.0,1.0,0.0), rot.y);
        self.model_matrix *= Mat4::from_axis_angle(Vec3::new(0.0,0.0,1.0), rot.z);

        self.model_matrix
    }
}