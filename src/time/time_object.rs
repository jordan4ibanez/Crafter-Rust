extern crate glfw;

pub struct Time {
    // fps fields
    previous_time: f64,
    current_time: f64,
    frame_count: i32,
    reset: bool,

    // delta fields
    previous_delta: f64,
    current_delta: f64
}

impl Time {
    pub fn count_fps(&mut self, glfw: &glfw::Glfw) -> bool {

        if self.reset {
            self.frame_count = 0;
            self.reset = false;
        }

        self.current_time = glfw.get_time();

        self.frame_count += 1;

        if self.current_time - self.previous_time >= 1.0 {

            self.previous_time = self.current_time;

            self.reset = true;

            return true;
        }

        false
    }

    pub fn calculate_delta(&mut self, glfw: &glfw::Glfw) -> f64 {
        self.current_delta = glfw.get_time();

        let delta = self.current_delta - self.previous_delta;

        self.previous_delta = glfw.get_time();

        delta
    }

    pub fn get_fps(&self) -> i32 {
        self.frame_count
    }
}

pub fn new(glfw: &glfw::Glfw) -> Time {
    let current_time = glfw.get_time();
    Time {
        // timer fields
        previous_time: current_time,
        current_time,
        frame_count: 0,
        reset: false,

        // delta fields
        previous_delta: current_time,
        current_delta: current_time
    }
}