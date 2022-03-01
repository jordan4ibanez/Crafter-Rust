pub struct LayerDepth {
    min: u8,
    max: u8
}

impl LayerDepth {
    pub fn new(min: u8, max: u8) -> Self {
        Self {
            min,
            max
        }
    }
    pub fn get(&self) -> (u8, u8) {
        (self.min, self.max)
    }
}

pub struct NoiseParams {
    min: f32,
    max: f32
}

impl NoiseParams {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }
    pub fn get(&self) -> (f32, f32) {
        (self.min, self.max)
    }
}



pub struct GenerationComponentSystem {

    id: Vec<u32>,

    name: Vec<String>,

    top_layer: Vec<u32>,
    top_layer_depth: Vec<LayerDepth>,

    bottom_layer: Vec<u32>,
    bottom_layer_depth: Vec<LayerDepth>,

    stone_layer: Vec<u32>,

    // how high or low the terrain can fluctuate
    terrain_noise_multiplier: Vec<u8>,

    // how often the terrain fluctuates
    terrain_frequency: Vec<f32>,

    // defines if there is cave generation
    caves: Vec<bool>,

    // minimum and maximum noise for a cave to be carved
    cave_heat: Vec<NoiseParams>,

    // defines if there is rain
    rain: Vec<bool>,

    // defines if there is snow
    snow: Vec<bool>

}

impl GenerationComponentSystem {
    pub fn new() -> Self {
        Self {
            id: Vec::new(),
            name: Vec::new(),
            top_layer: Vec::new(),
            top_layer_depth: Vec::new(),
            bottom_layer: Vec::new(),
            bottom_layer_depth: Vec::new(),
            stone_layer: Vec::new(),
            terrain_noise_multiplier: Vec::new(),
            terrain_frequency: Vec::new(),
            caves: Vec::new(),
            cave_heat: Vec::new(),
            rain: Vec::new(),
            snow: Vec::new(),
        }
    }

    pub fn register_biome(
        &mut self,

        name: String,

        top_layer: u32,
        top_layer_depth: LayerDepth,

        bottom_layer: u32,
        bottom_layer_depth: LayerDepth,

        stone_layer: u32,

        terrain_noise_multiplier: u8,

        terrain_frequency: f32,

        caves: bool,

        cave_heat: NoiseParams,

        rain: bool,

        snow: bool

    ){
        self.id.push(self.id.len() as u32 + 1);

        self.name.push(name);

        self.top_layer.push(top_layer);
        self.top_layer_depth.push(top_layer_depth);

        self.bottom_layer.push(bottom_layer);
        self.bottom_layer_depth.push(bottom_layer_depth);

        self.stone_layer.push(stone_layer);

        self.terrain_noise_multiplier.push(terrain_noise_multiplier);

        self.terrain_frequency.push(terrain_frequency);

        self.caves.push(caves);

        self.cave_heat.push(cave_heat);

        self.rain.push(rain);

        self.snow.push(snow);
    }
    
}