use glam::{
    IVec2
};

use crate::graphics::mesh::Mesh;


pub struct Chunk {
    key: String,
    position:  IVec2,
    block:     [u32; 32768],
    rotation:  [u8;  32768],
    light:     [u8;  32768],
    heightmap: [u8;  256],

    mesh: Option<Mesh>
}

impl Chunk {
    pub fn get_pos(&self) -> IVec2 {
        self.position
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_mesh(&self) -> Option<&Mesh> {
        self.mesh.as_ref()
    }

    pub fn get_mesh_mut(&mut self) -> Option<&mut Mesh> {
        self.mesh.as_mut()
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        match &self.mesh {
            Some(existing_mesh) => existing_mesh.clean_up(false),
            None => self.mesh = Some(mesh),
        }
    }
}


pub fn new(x: i32, y: i32) -> Chunk {
    Chunk {
        key: x.to_string() + " " + &y.to_string(),
        position: IVec2::new(x, y),
        block: [0; 32768],
        rotation: [0; 32768],
        light: [0; 32768],
        heightmap: [0; 256],
        mesh: None
    }
}

/*
pub fn new_with_mesh(x: i32, y: i32, mesh: Mesh) -> Chunk {
    Chunk {
        key: x.to_string() + " " + &y.to_string(),
        position: IVec2::new(x, y),
        block: [0; 32768],
        rotation: [0; 32768],
        light: [0; 32768],
        heightmap: [0; 256],
        mesh: Some(mesh)
    }
}
*/