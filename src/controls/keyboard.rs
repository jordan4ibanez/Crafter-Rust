use glfw::WindowEvent;

// hardcoded for now
pub struct Keyboard {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,

    jump: bool,
    sneak: bool
}

// only getters for keyboard right now except polling
impl Keyboard {

    pub fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
            jump: false,
            sneak: false,
        }
    }


    pub fn get_forward(&self) -> bool {
        self.forward
    }

    pub fn get_backward(&self) -> bool {
        self.backward
    }

    pub fn get_left(&self) -> bool {
        self.left
    }

    pub fn get_right(&self) -> bool {
        self.right
    }

    pub fn get_jump(&self) -> bool {
        self.jump
    }

    pub fn get_sneak(&self) -> bool {
        self.sneak
    }

    pub fn process_events(&mut self, event: &WindowEvent){
        /*
        I'm leaving this here for when settings are implemented
        
        key is the variable name in the full implementation

        key => checks against settings => self.forward = action == &glfw::Action::Press
        
        glfw::WindowEvent::Key(key, key_code, enum_action, modifier)
        */
        match event {

            // forward
            glfw::WindowEvent::Key(glfw::Key::W, _, action, _) => self.forward = action == &glfw::Action::Press || action == &glfw::Action::Repeat,

            // backward
            glfw::WindowEvent::Key(glfw::Key::S, _, action, _) => self.backward = action == &glfw::Action::Press || action == &glfw::Action::Repeat,

            // left
            glfw::WindowEvent::Key(glfw::Key::A, _, action, _) => self.left = action == &glfw::Action::Press || action == &glfw::Action::Repeat,

            // right
            glfw::WindowEvent::Key(glfw::Key::D, _, action, _) => self.right = action == &glfw::Action::Press || action == &glfw::Action::Repeat,

            // jump
            glfw::WindowEvent::Key(glfw::Key::Space, _, action, _) => self.jump = action == &glfw::Action::Press || action == &glfw::Action::Repeat,

            // sneak
            glfw::WindowEvent::Key(glfw::Key::LeftShift, _, action, _) => self.sneak = action == &glfw::Action::Press || action == &glfw::Action::Repeat,

            _ => ()
        }
    }
}