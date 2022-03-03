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

    pub fn get_min(&self) -> u8 {
        self.min
    }

    pub fn get_max(&self) -> u8 {
        self.max
    }
}

pub struct NoiseParams {
    min: f32,
    max: f32,
    frequency: f32
}

impl NoiseParams {
    pub fn new(min: f32, max: f32, frequency: f32) -> Self {
        Self {
            min,
            max,
            frequency
        }
    }
    pub fn get(&self) -> (f32, f32, f32) {
        (self.min, self.max, self.frequency)
    }

    pub fn get_min(&self) -> f32 {
        self.min
    }

    pub fn get_max(&self) -> f32 {
        self.max
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn in_range(&self, noise_calculation: f32) -> bool {
        noise_calculation >= self.min && noise_calculation <= self.max
    }
}


pub struct HeatParams {
    min: f32,
    max: f32,
}

impl HeatParams {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }
    pub fn get(&self) -> (f32, f32) {
        (self.min, self.max)
    }

    pub fn get_min(&self) -> f32 {
        self.min
    }

    pub fn get_max(&self) -> f32 {
        self.max
    }

    pub fn in_range(&self, noise_calculation: f32) -> bool {
        noise_calculation >= self.min && noise_calculation <= self.max
    }
}

pub struct BiomeOres {
    // held as block ID
    size: usize,
    ores: Vec<u32>,
    depth: Vec<LayerDepth>,
    heat: Vec<HeatParams>,
    frequency: Vec<f32>,
}

impl BiomeOres {
    pub fn new() -> Self {
        Self {
            size: 0,
            ores: Vec::new(),
            depth: Vec::new(),
            heat: Vec::new(),
            frequency: Vec::new(),
        }
    }

    pub fn register_ore(&mut self, id: u32, depth: LayerDepth, heat: HeatParams, frequency: f32) {
        self.size += 1;
        self.ores.push(id);
        self.depth.push(depth);
        self.heat.push(heat);
        self.frequency.push(frequency);
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_ore(&self, index: usize) -> (&u32, &LayerDepth, &HeatParams, &f32) {
        (&self.ores[index], &self.depth[index], &self.heat[index], &self.frequency[index])
    }
}


// the gcs holds all biome data exclusively
pub struct GenerationComponentSystem {

    id: Vec<u32>,

    game_mod: Vec<String>,

    name: Vec<String>,

    top_layer: Vec<u32>,
    top_layer_depth: Vec<LayerDepth>,

    bottom_layer: Vec<u32>,
    bottom_layer_depth: Vec<LayerDepth>,

    stone_layer: Vec<u32>,

    biome_ores: Vec<Option<BiomeOres>>,

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
            game_mod: Vec::new(),
            name: Vec::new(),
            top_layer: Vec::new(),
            top_layer_depth: Vec::new(),
            bottom_layer: Vec::new(),
            bottom_layer_depth: Vec::new(),
            stone_layer: Vec::new(),
            biome_ores: Vec::new(),
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

        game_mod: String,

        top_layer: u32,
        top_layer_depth: LayerDepth,

        bottom_layer: u32,
        bottom_layer_depth: LayerDepth,

        stone_layer: u32,

        biome_ores: Option<BiomeOres>,

        terrain_noise_multiplier: u8,

        terrain_frequency: f32,

        caves: bool,

        cave_heat: NoiseParams,

        rain: bool,

        snow: bool

    ){

        println!("BIOME: {} IS ID: {}",name,self.id.len() as u32);

        self.id.push(self.id.len() as u32);

        self.game_mod.push(game_mod);

        self.name.push(name);

        self.top_layer.push(top_layer);
        self.top_layer_depth.push(top_layer_depth);

        self.bottom_layer.push(bottom_layer);
        self.bottom_layer_depth.push(bottom_layer_depth);

        self.stone_layer.push(stone_layer);

        self.biome_ores.push(biome_ores);

        self.terrain_noise_multiplier.push(terrain_noise_multiplier);

        self.terrain_frequency.push(terrain_frequency);

        self.caves.push(caves);

        self.cave_heat.push(cave_heat);

        self.rain.push(rain);

        self.snow.push(snow);
    }


    /*
    id: Vec<u32>,
    name: Vec<String>,
    top_layer: Vec<u32>,
    top_layer_depth: Vec<LayerDepth>,
    bottom_layer: Vec<u32>,
    bottom_layer_depth: Vec<LayerDepth>,
    stone_layer: Vec<u32>,
    terrain_noise_multiplier: Vec<u8>,
    terrain_frequency: Vec<f32>,
    caves: Vec<bool>,
    cave_heat: Vec<NoiseParams>,
    rain: Vec<bool>,
    snow: Vec<bool>
             */

    // this is debug
    // in production this will search by heatmap of 2D
    // this is also a mess
    pub fn get(&self, id: usize) -> (&String, u32, &LayerDepth, u32, &LayerDepth, u32, &Option<BiomeOres>, u8, f32, bool, &NoiseParams, bool, bool) {
        (
            &self.name[id],

            self.top_layer[id],
            &self.top_layer_depth[id],

            self.bottom_layer[id],
            &self.bottom_layer_depth[id],

            self.stone_layer[id],

            &self.biome_ores[id],

            self.terrain_noise_multiplier[id],

            self.terrain_frequency[id],

            self.caves[id],

            &self.cave_heat[id],

            self.rain[id],

            self.snow[id],
        )
    }
    
}