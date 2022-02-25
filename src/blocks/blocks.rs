pub struct BlockComponentSystem {
    id: Vec<u32>,
    texture: Vec<String>,
    shape: Vec<f32>,
    draw_type: Vec<u32>,

}

impl BlockComponentSystem {

    pub fn new() -> Self {
        BlockComponentSystem {
            id: Vec::new(),
            texture: Vec::new(),
            shape: Vec::new(),
            draw_type: Vec::new(),
        }
    }
    
}