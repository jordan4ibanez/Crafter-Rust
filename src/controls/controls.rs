use glfw::Window;

use super::{keyboard::{Keyboard}, mouse::Mouse};

pub struct Controls {
    pub keyboard: Keyboard,
    pub mouse: Mouse
}

impl Controls {
    pub fn new(window: &Window) -> Self {
        Controls {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(window),
        }
    }
}