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
}

pub struct BlockShape {
    shape: Vec<f32>
}

impl BlockShape {
    // shape is a basic container for a vector of floats
    pub fn new(shape: Vec<f32>) -> Self {
        
        if shape.len() % 6 != 0 {
            panic!("BLOCK SHAPE IS NOT EVEN!");
        }

        BlockShape {
            shape,
        }
    }
}


pub struct BlockComponentSystem {
    id: Vec<u32>,
    name: Vec<String>,
    texture: Vec<Vec<String>>,
    shape: Vec<Option<BlockShape>>, // this will be replaced with block shape or vec of f32
    draw_type: Vec<DrawType>,
    mapping: Vec<Vec<AtlasTextureMap>>
}

pub enum DrawType {
    None,
    Normal
}

impl BlockComponentSystem {
    pub fn new() -> Self {
        let mut component_system = BlockComponentSystem {
            id: Vec::new(),
            name: Vec::new(),
            texture: Vec::new(),
            shape: Vec::new(),
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
        shape: Option<BlockShape>,
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

        self.shape.push(shape);

        self.draw_type.push(draw_type);

        self.mapping.push(mapping);

    }
}