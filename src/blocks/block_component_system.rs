
// holds precalculated data
#[derive(Debug)]
pub struct AtlasTextureMap {
    min_x: f32,
    min_y: f32,

    max_x: f32,
    max_y: f32,

    rotation: u8,
    flip: u8
}

impl AtlasTextureMap {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32, rotation: u8, flip: u8) -> Self {
        AtlasTextureMap {
            min_x,
            min_y,
            max_x,
            max_y,
            rotation,
            flip
        }
    }

    pub fn get_as_tuple(&self) -> (f32, f32, f32, f32, u8) {
        (self.min_x, self.min_y, self.max_x, self.max_y, self.rotation)
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
    draw_type: Vec<DrawType>,
    texture: Vec<Vec<String>>,
    block_box: Vec<Option<BlockBox>>,
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
            draw_type: Vec::new(),
            texture: Vec::new(),
            block_box: Vec::new(),
            mapping: Vec::new()
        };

        // built in definition for air
        component_system.register_block(String::from("air"), DrawType::None, vec![], None, vec![]);

        component_system
    }
    
    pub fn register_block(
        &mut self,
        name: String,
        draw_type: DrawType,
        mut textures: Vec<String>,
        block_box: Option<BlockBox>,
        mapping: Vec<AtlasTextureMap>
    ) {
        println!("{} is ID: {}", &name, self.id.len());

        self.id.push(self.id.len() as u32);

        self.name.push(name.clone());

        self.draw_type.push(draw_type);

        // fill the vector with unknown texture
        while textures.len() < 6 {
            textures.push(String::from("unknown.png"));
        }

        self.texture.push(textures);

        self.block_box.push(block_box);

        self.mapping.push(mapping);
    }

    pub fn get_mapping(&self, id: u32) -> &Vec<AtlasTextureMap> {
        self.mapping.get(id as usize).unwrap()
    }

    pub fn get_id_of(&self, name: String) -> u32 {
        self.name
            .iter()
            .enumerate()
            .find(|test|{
                test.1.eq(&name)
            })
            .expect("TRIED TO GET ID OF NON-EXISTENT BLOCK!")
            .0 as u32
    }
}