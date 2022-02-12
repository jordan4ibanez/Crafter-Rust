extern crate glfw;

pub struct Time {
    // fps fields
    previous_time: f64,
    current_time: f64,
    frame_count: i32,

    // delta fields
    previous_delta: f64,
    current_delta: f64
}

impl Time {
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

    pub fn calculate_delta(&mut self, glfw: &glfw::Glfw) -> f64 {
        self.current_delta = glfw.get_time();

        let delta = self.current_delta - self.previous_delta;

        self.previous_delta = glfw.get_time();

        delta
    }
}

pub fn new(glfw: &glfw::Glfw) -> Time {
    let current_time = glfw.get_time();
    Time {
        // timer fields
        previous_time: current_time,
        current_time,
        frame_count: 0,

        // delta fields
        previous_delta: current_time,
        current_delta: current_time
    }
}