use glfw::{
    Glfw,
    Window,
    Monitor,
    VidMode
};

pub struct WindowVariables {
    full_screen: bool
}

impl WindowVariables {
    pub fn new() -> WindowVariables {
        WindowVariables {
            full_screen: false
        }
    }

    pub fn set_full_screen(&mut self, full_screen: bool) {
        self.full_screen = full_screen;
    }

    pub fn get_full_screen(&self) -> bool {
        self.full_screen
    }
}

pub fn toggle_full_screen(glfw: &mut Glfw, window: &mut Window, window_variables: &mut WindowVariables){
    glfw.with_primary_monitor(| glfw, m: Option<&Monitor> | {

        // must unwrap the safety chain
        let monitor_reference: &Monitor = m.unwrap();
        let video_mode_option: Option<VidMode> = monitor_reference.get_video_mode();
        let video_mode: VidMode = video_mode_option.unwrap();
        let monitor_size: (u32, u32) = (video_mode.width, video_mode.height);
        
        // windowed
        if window_variables.get_full_screen() {            
            window.set_monitor(
                glfw::WindowMode::Windowed, 
                (monitor_size.0 as i32 - (monitor_size.0 as i32 / 2)) / 2,
                (monitor_size.1 as i32 - (monitor_size.1 as i32 / 2)) / 2,
                monitor_size.0 / 2,
                monitor_size.1 / 2,
                Some(video_mode.refresh_rate)
            );
        // full screen
        } else {
            window.set_monitor(
                glfw::WindowMode::FullScreen(&monitor_reference), 
                video_mode.width as i32 / 2, 
                video_mode.height as i32 / 2, 
                video_mode.width,
                video_mode.height, 
                Some(video_mode.refresh_rate)
            );
        }

        // invert
        window_variables.set_full_screen(!window_variables.get_full_screen());

        println!("F11 FULL SCREEN NEEDS TO POLL THE SETTINGS!");
        glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
    });    
}