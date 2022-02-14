
use glam::{Mat4, Vec3};

// Remember that glam is basically just joml

// I will implement ortholinear rendering later
pub struct Transformation {
    projection_matrix: Mat4,
    model_matrix: Mat4,
    view_matrix: Mat4,
}

impl Transformation {

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }


    pub fn reset_projection_matrix(&mut self, fov: f32, width: f32, height: f32, z_near: f32, z_far: f32) {

        // this is the window of the game, the initial camera position and specifications
        self.projection_matrix = Mat4::perspective_rh_gl(fov.to_radians(), width / height, z_near, z_far);


        // camera rotation inversion goes here
        let camera_rotation: (f32, f32) = (0.0, 0.0);

        self.view_matrix = Mat4::IDENTITY;

        self.view_matrix *= Mat4::from_axis_angle(Vec3::new(0.0,1.0,0.0), camera_rotation.0);
        self.view_matrix *= Mat4::from_axis_angle(Vec3::new(1.0,0.0,0.0), camera_rotation.1);


        // camera position inversion goes here
        let camera_position : (f32, f32, f32) = (0.0, 0.0, -10.0);

        let my_vector: Vec3 = Vec3::new(camera_position.0, camera_position.1, camera_position.2);

        self.view_matrix *= Mat4::from_translation(my_vector);


    }
}


pub fn new() -> Transformation {

    let returning_transformation: Transformation = Transformation {
        projection_matrix: Mat4::IDENTITY,
        model_matrix: Mat4::IDENTITY,
        view_matrix: Mat4::IDENTITY
    };
    
    returning_transformation
}