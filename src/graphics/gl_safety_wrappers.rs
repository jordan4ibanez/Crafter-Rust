
pub fn clear_depth_and_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r,g,b,a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Clear(gl::DEPTH_BUFFER_BIT);
    }
}