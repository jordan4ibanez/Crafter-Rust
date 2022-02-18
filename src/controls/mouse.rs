use glam::Vec2;

use glfw::{
    WindowEvent,
    Window
};

pub struct Mouse {
    position: Vec2,
    old_position: Vec2,
    position_vector: Vec2,

    left_mouse_button: bool,
    right_mouse_button: bool,

    scroll: f32,
    
    in_window: bool,
    locked_to_window: bool
}

impl Mouse {

    pub fn new(window: &Window) -> Self {

        let pos_f64: (f64, f64) = window.get_cursor_pos();
    
        let def: (f32, f32) = (pos_f64.0 as f32, pos_f64.1 as f32);
    
        Self {
            position: Vec2::new(def.0, def.1),
            old_position: Vec2::new(def.0, def.1),
            position_vector: Vec2::new(def.0, def.1),
            left_mouse_button: false,
            right_mouse_button: false,
            scroll: 0.0,
            in_window: false,
            locked_to_window: false
        }
    }

    // public getters

    pub fn get_pos(&self) -> Vec2 {
        self.position
    }

    pub fn get_pos_vec(&self) -> Vec2 {
        self.position_vector
    }

    pub fn is_left_button_pressed(&self) -> bool {
        self.left_mouse_button
    }

    pub fn is_right_button_pressed(&self) -> bool {
        self.right_mouse_button
    }

    pub fn is_in_window(&self) -> bool {
        self.in_window
    }
    
    pub fn is_locked(&self) -> bool {
        self.locked_to_window
    }

    // protected setters
    fn set_pos(&mut self, x: &f64, y: &f64) {

        // set the old position in memory
        self.old_position.x = self.position.x;
        self.old_position.y = self.position.y;

        // update the new position
        self.position.x = *x as f32;
        self.position.y = *y as f32;

        // calculate the vector of the mouse
        self.position_vector.x = self.position.x - self.old_position.x;
        self.position_vector.y = self.position.y - self.old_position.y;
    }

    fn set_in_window(&mut self, in_window: bool) {

        self.in_window = in_window;

        // now other things can happen
    }

    fn set_scroll(&mut self, scroll: &f64) {
        self.scroll = *scroll as f32;

    }

    // this manually resets the position vector
    pub fn reset(&mut self) {
        self.set_pos(&(self.position.x as f64) as &f64, &(self.position.y as f64) as &f64);
    }

    pub fn process_events(&mut self, event: &WindowEvent){

        match event {

            // left mouse button
            glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, action, _) => self.left_mouse_button = action == &glfw::Action::Press,

            // right mouse button
            glfw::WindowEvent::MouseButton(glfw::MouseButtonRight, action, _) => self.right_mouse_button = action == &glfw::Action::Press,

            // mouse within window
            glfw::WindowEvent::CursorEnter(entered) => self.set_in_window(entered == &true),

            // mouse movement
            glfw::WindowEvent::CursorPos(x,y) => self.set_pos(x,y),

            // mouse scrolling
            glfw::WindowEvent::Scroll(_, scroll) => self.set_scroll(scroll),

            _ => ()

        }
    }
}