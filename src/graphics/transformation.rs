
use glam::{Mat4, Vec3};

// Remember that glam is basically just joml

// I will implement ortholinear rendering later
pub struct Transformation {
    projection_matrix: Mat4,
    model_matrix: Mat4,
    view_matrix: Mat4, // <- this is an identity - reference Mat4 TODO: rename this to camera_matrix
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
        // camera rotation will be a vec2
        let camera_rotation: (f32, f32) = (0.0, 0.0);

        self.view_matrix = Mat4::IDENTITY;

        self.view_matrix *= Mat4::from_axis_angle(Vec3::new(0.0,1.0,0.0), camera_rotation.0);
        self.view_matrix *= Mat4::from_axis_angle(Vec3::new(1.0,0.0,0.0), camera_rotation.1);


        // camera position inversion goes here
        // camera position will be a vec3
        // this is also inverted
        let camera_position : (f32, f32, f32) = (0.0, 0.0, 10.0);

        let my_vector: Vec3 = Vec3::new(-camera_position.0, -camera_position.1, -camera_position.2);

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


pub fn new() -> Transformation {

    let returning_transformation: Transformation = Transformation {
        projection_matrix: Mat4::IDENTITY,
        model_matrix: Mat4::IDENTITY,
        view_matrix: Mat4::IDENTITY
    };
    
    returning_transformation
}