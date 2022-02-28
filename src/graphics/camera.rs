use std::f32::consts::PI;

use glam::{Vec3, IVec2};

use crate::controls::{mouse::Mouse, keyboard::Keyboard};


pub struct Camera {
    position: Vec3,
    // old_position: Vec3,

    pos_floored: IVec2,
    old_pos_floored: IVec2,

    rotation: Vec3,
    rotation_vector: Vec3,
    fov: f32
}

impl Camera {

    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 80.0,0.0),
            // old_position: Vec3::new(0.0, 129.0,0.0),
            pos_floored: IVec2::new(0, 0),
            old_pos_floored: IVec2::new(0, 0),
            rotation: Vec3::new(0.0, 0.0,0.0),
            rotation_vector: Vec3::new(0.0, 0.0,0.0),
            fov: 60.0,
        }
    }

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

    pub fn on_tick(&mut self, mouse: &Mouse, keyboard: &Keyboard, delta: f32) -> bool {

        self.old_pos_floored.clone_from(&self.pos_floored);

        let movement_speed: f32 = delta * 40.0;

        let keyboard = keyboard;
        let mouse = mouse;

        // z axis
        if keyboard.get_forward() {
            let yaw: f32 = self.rotation.y.to_radians() + PI;
            self.position.x += -yaw.sin() * movement_speed;
            self.position.z += yaw.cos() * movement_speed;
        }

        if keyboard.get_backward() {
            let yaw: f32 = self.rotation.y.to_radians();
            self.position.x += -yaw.sin() * movement_speed;
            self.position.z += yaw.cos() * movement_speed;
        }

        // x axis
        if keyboard.get_left() {
            let yaw: f32 = self.rotation.y.to_radians() + (PI / 2.0);
            self.position.x += -yaw.sin() * movement_speed;
            self.position.z += yaw.cos() * movement_speed;
        }

        if keyboard.get_right() {
            let yaw: f32 = self.rotation.y.to_radians() - (PI / 2.0);
            self.position.x += -yaw.sin() * movement_speed;
            self.position.z += yaw.cos() * movement_speed;
        }

        // y axis
        if keyboard.get_sneak() {
            self.position.y -= movement_speed;
        }

        if keyboard.get_jump() {
            self.position.y += movement_speed;
        }

        // rotation
        const MOUSE_SENSITIVITY: f32 = 0.09;
        self.rotation.x += mouse.get_pos_vec().y * MOUSE_SENSITIVITY;
        self.rotation.y += mouse.get_pos_vec().x * MOUSE_SENSITIVITY;

        // limit rotation pitch
        if self.rotation.x > 90.0 {
            self.rotation.x = 90.0;
        } else if self.rotation.x < -90.0 {
            self.rotation.x = -90.0;
        }

        // loop camera yaw
        if self.rotation.y > 360.0 {
            self.rotation.y -= 360.0;
        } else if self.rotation.y < 0.0 {
            self.rotation.y += 360.0;
        }


        // check if the world needs to resort the mesh order

        self.pos_floored.x = (self.position.x / 16.0).floor() as i32;
        self.pos_floored.y = (self.position.z / 16.0).floor() as i32;

        let mut update_chunk_ordering: bool = false;
        if self.old_pos_floored.ne(&self.pos_floored) {
            update_chunk_ordering = true;
        }
        
        println!("camera rotation: {}", self.rotation.y);

        update_chunk_ordering
    }

}