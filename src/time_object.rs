extern crate glfw;

pub struct Time {
    previous_time: f64,
    current_time: f64,
    frame_count: i32,
}

impl Time {
    #[inline(always)]
    pub fn count_fps(&mut self, glfw: &glfw::Glfw) -> (bool, i32) {

        self.current_time = glfw.get_time();

        self.frame_count += 1;

        if self.current_time - self.previous_time >= 1.0 {

            self.previous_time = self.current_time;

            let counted_fps = self.frame_count.clone();

            self.frame_count = 0;

            return (true, counted_fps);

        }

        (false, 0)
    }
}

#[inline(always)]
pub fn new(glfw: &glfw::Glfw) -> Time {
    let current_time = glfw.get_time();
    Time {
        previous_time: current_time.clone(),
        current_time: current_time.clone(),
        frame_count: 0,
    }
}