pub struct BlockComponentSystem {
    id: Vec<u32>,
    name: Vec<String>,
    texture: Vec<Vec<String>>,
    shape: Vec<Vec<f32>>, // this will be replaced with block shape or vec of f32
    draw_type: Vec<u32>
}

impl BlockComponentSystem {

    pub fn new() -> Self {
        BlockComponentSystem {
            id: Vec::new(),
            name: Vec::new(),
            texture: Vec::new(),
            shape: Vec::new(),
            draw_type: Vec::new(),
        }
    }
    
    pub fn register_block(
        &mut self,
        name: &str,
        mut textures: Vec<String>,
        shape: Vec<f32>,
        draw_type: u32
    ) {
        
        self.id.push(self.id.len() as u32);

        self.name.push(name.to_string());

        // fill the vector with unknown texture
        while textures.len() < 6 {
            textures.push(String::from("unkown.png"));
        }

        self.texture.push(textures);

        // panic if the shape is not evenly done
        if shape.len() % 6 != 0 {
            panic!("BLOCK {} DOES NOT HAVE AN EVEN AMOUNT OF SHAPE! (6 components per shape -x, -y, -z, +x +y +z)", name);
        }

        self.shape.push(shape);

        self.draw_type.push(draw_type);

    }
}