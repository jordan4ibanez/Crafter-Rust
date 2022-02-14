use glam::Vec3;

use crate::controls::keyboard::Keyboard;


pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    rotation_vector: Vec3,
    fov: f32
}

impl Camera {

    // setters and getters

    pub fn set_pos(&mut self, new_position: &Vec3) {
        self.position.clone_from(new_position);
    }

    pub fn get_pos(&self) -> Vec3 {
        self.position
    }

    pub fn set_rot(&mut self, new_rotation: &Vec3) {
        self.rotation.clone_from(new_rotation);
    }

    pub fn get_rot(&self) -> Vec3 {
        self.rotation
    }

    pub fn set_rot_vec(&mut self, new_rotation_vector: &Vec3) {
        self.rotation_vector.clone_from(new_rotation_vector);
    }

    pub fn get_rot_vec(&self) -> Vec3 {
        self.rotation_vector
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    // methods

    // the rotation vector of the rotation
    fn calculate_rotation_vector(&mut self) {

        let x_z_length = (
            (self.rotation.x + 180.0).to_radians()
        ).cos();

        self.rotation_vector.z = x_z_length * (
            (self.rotation.y).to_radians()
        ).cos();

        self.rotation_vector.y = (
            (self.rotation.x + 180.0).to_radians()
        ).sin();

        self.rotation_vector.x = x_z_length * (
            (-self.rotation.y).to_radians()
        ).sin();

    }

    pub fn on_tick(&mut self, keyboard: &Keyboard) {
        if keyboard.get_backward() {
            self.position.z -= 0.01;
        }
        if keyboard.get_forward() {
            self.position.z += 0.01;
        }
    }

}

pub fn new() -> Camera {
    Camera {
        position: Vec3::new(0.0, 0.0,0.0),
        rotation: Vec3::new(0.0, 0.0,0.0),
        rotation_vector: Vec3::new(0.0, 0.0,0.0),
        fov: 60.0,
    }
}