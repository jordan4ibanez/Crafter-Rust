pub struct WindowVariables {
    full_screen: bool
}

impl WindowVariables {
    pub fn set_full_screen(&mut self, full_screen: bool) {
        self.full_screen = full_screen;
    }

    pub fn get_full_screen(&self) -> bool {
        self.full_screen
    }
}

pub fn new() -> WindowVariables {
    WindowVariables {
        full_screen: false
    }
}