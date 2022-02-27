use std::borrow::Borrow;


// holds precalculated data
pub struct AtlasTextureMap {
    min_x: f32,
    min_y: f32,

    max_x: f32,
    max_y: f32
}

impl AtlasTextureMap {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        AtlasTextureMap {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn get_as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.min_x, self.min_y, self.max_x, self.max_y)
    }
}

pub struct BlockBox {
    block_box: Vec<f32>
}

impl BlockBox {
    // block_box is a basic container for a vector of floats
    pub fn new(block_box: Vec<f32>) -> Self {
        
        // this is a double check in case lua misses it - perhaps someone manually inserted into table?
        if block_box.len() < 6 || block_box.len() % 6 != 0 {
            panic!("BLOCK BOX IS NOT EVEN!");
        }

        BlockBox {
            block_box,
        }
    }

    pub fn get(&self) -> &Vec<f32> {
        &self.block_box
    }
}


pub struct BlockComponentSystem {
    id: Vec<u32>,
    name: Vec<String>,
    texture: Vec<Vec<String>>,
    block_box: Vec<Option<BlockBox>>, // this will be replaced with block shape or vec of f32
    draw_type: Vec<DrawType>,
    mapping: Vec<Vec<AtlasTextureMap>>
}

pub enum DrawType {
    None,
    Normal,
    BlockBox
}

impl BlockComponentSystem {
    pub fn new() -> Self {
        let mut component_system = BlockComponentSystem {
            id: Vec::new(),
            name: Vec::new(),
            texture: Vec::new(),
            block_box: Vec::new(),
            draw_type: Vec::new(),
            mapping: Vec::new()
        };

        // built in definition for air
        component_system.register_block("air".to_string(), vec![], None, DrawType::None, vec![]);

        component_system
    }
    
    pub fn register_block(
        &mut self,
        name: String,
        mut textures: Vec<String>,
        block_box: Option<BlockBox>,
        draw_type: DrawType,
        mapping: Vec<AtlasTextureMap>
    ) {

        self.id.push(self.id.len() as u32);

        self.name.push(name.clone());

        // fill the vector with unknown texture
        while textures.len() < 6 {
            textures.push(String::from("unkown.png"));
        }

        self.texture.push(textures);

        self.block_box.push(block_box);

        self.draw_type.push(draw_type);

        self.mapping.push(mapping);

    }
}