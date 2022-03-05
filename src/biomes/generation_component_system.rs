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
    scale: f32,
    frequency: f32,
}

impl NoiseParams {
    pub fn new(min: f32, max: f32, scale: f32, frequency: f32) -> Self {
        Self {
            min,
            max,
            scale,
            frequency
        }
    }
    pub fn get(&self) -> (f32, f32, f32, f32) {
        (self.min, self.max, self.scale, self.frequency)
    }

    pub fn get_min(&self) -> f32 {
        self.min
    }

    pub fn get_max(&self) -> f32 {
        self.max
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
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
    heat: Vec<NoiseParams>,
    frequency: Vec<f32>,
    scale: Vec<f32>
}

impl BiomeOres {
    pub fn new() -> Self {
        Self {
            size: 0,
            ores: Vec::new(),
            depth: Vec::new(),
            heat: Vec::new(),
            frequency: Vec::new(),
            scale: Vec::new()
        }
    }

    pub fn register_ore(&mut self, id: u32, depth: LayerDepth, heat: NoiseParams, frequency: f32, scale: f32) {
        self.size += 1;
        self.ores.push(id);
        self.depth.push(depth);
        self.heat.push(heat);
        self.frequency.push(frequency);
        self.scale.push(scale);
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_ore(&self, index: usize) -> (u32, &LayerDepth, &NoiseParams, f32, f32) {
        (self.ores[index], &self.depth[index], &self.heat[index], self.frequency[index], self.scale[index])
    }
}


// the gcs holds all biome data exclusively
pub struct GenerationComponentSystem {

    id: Vec<u32>,

    biome_noise_params: Vec<NoiseParams>,

    // how high or low the terrain can fluctuate
    terrain_height_flux: Vec<u8>,

    game_mod: Vec<String>,

    name: Vec<String>,

    top_layer: Vec<u32>,
    top_layer_depth: Vec<LayerDepth>,

    bottom_layer: Vec<u32>,
    bottom_layer_depth: Vec<LayerDepth>,

    stone_layer: Vec<u32>,

    bedrock_layer: Vec<u32>,

    biome_ores: Vec<Option<BiomeOres>>,

    // defines if there is cave generation
    caves: Vec<bool>,

    // minimum and maximum noise for a cave to be carved
    cave_noise_params: Vec<NoiseParams>,

    // defines if there is rain
    rain: Vec<bool>,

    // defines if there is snow
    snow: Vec<bool>

}

impl GenerationComponentSystem {
    pub fn new() -> Self {
        Self {
            id: Vec::new(),
            biome_noise_params: Vec::new(),
            terrain_height_flux: Vec::new(),
            game_mod: Vec::new(),
            name: Vec::new(),
            top_layer: Vec::new(),
            top_layer_depth: Vec::new(),
            bottom_layer: Vec::new(),
            bottom_layer_depth: Vec::new(),
            stone_layer: Vec::new(),
            bedrock_layer: Vec::new(),
            biome_ores: Vec::new(),
            caves: Vec::new(),
            cave_noise_params: Vec::new(),
            rain: Vec::new(),
            snow: Vec::new(),
        }
    }

    pub fn register_biome(
        &mut self,

        name: String,

        biome_noise_params: NoiseParams,

        terrain_height_flux: u8,

        game_mod: String,

        top_layer: u32,
        top_layer_depth: LayerDepth,

        bottom_layer: u32,
        bottom_layer_depth: LayerDepth,

        stone_layer: u32,

        bedrock_layer: u32,

        biome_ores: Option<BiomeOres>,

        caves: bool,

        cave_noise_params: NoiseParams,

        rain: bool,

        snow: bool

    ){

        println!("BIOME: {} IS ID: {}",name,self.id.len() as u32);

        self.id.push(self.id.len() as u32);

        self.biome_noise_params.push(biome_noise_params);

        self.terrain_height_flux.push(terrain_height_flux);

        self.game_mod.push(game_mod);

        self.name.push(name);

        self.top_layer.push(top_layer);
        self.top_layer_depth.push(top_layer_depth);

        self.bottom_layer.push(bottom_layer);
        self.bottom_layer_depth.push(bottom_layer_depth);

        self.stone_layer.push(stone_layer);

        self.bedrock_layer.push(bedrock_layer);

        self.biome_ores.push(biome_ores);


        self.caves.push(caves);

        self.cave_noise_params.push(cave_noise_params);

        self.rain.push(rain);

        self.snow.push(snow);
    }


    // this is debug
    // in production this will search by heatmap of 2D
    // this is also a mess
    pub fn get(&self, id: usize) -> (&String, &NoiseParams, u8, u32, &LayerDepth, u32, &LayerDepth, u32, u32, &Option<BiomeOres>, bool, &NoiseParams, bool, bool) {
        (
            &self.name[id],

            &self.biome_noise_params[id],

            self.terrain_height_flux[id],

            self.top_layer[id],
            &self.top_layer_depth[id],

            self.bottom_layer[id],
            &self.bottom_layer_depth[id],

            self.stone_layer[id],

            self.bedrock_layer[id],

            &self.biome_ores[id],

            self.caves[id],

            &self.cave_noise_params[id],

            self.rain[id],

            self.snow[id],
        )
    }
    
}