pub struct LayerDepth {
    min: u8,
    max: u8
}

impl LayerDepth {
    pub fn new(min: u8, max: u8) -> Self {
        LayerDepth {
            min,
            max
        }
    }
    pub fn get(&self) -> (u8, u8) {
        (self.min, self.max)
    }
}

pub struct GenerationComponentSystem {
    id: Vec<String>,
    name: Vec<String>,
    top_layer: Vec<u32>

}

impl GenerationComponentSystem {
    
}