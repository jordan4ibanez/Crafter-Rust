use glam::{
    IVec2
};

use crate::graphics::mesh_component_system::MeshComponentSystem;


pub struct Chunk {
    
}

impl Chunk {
    /*
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            key: x.to_string() + " " + &y.to_string(),
            position: IVec2::new(x, y),
            block: vec![0; 32768],
            rotation: vec![0; 32768],
            light: vec![0; 32768],
            heightmap: vec![0; 256],
            mesh_id: None
        }
    }

    pub fn get_block(&self, x: i8, y: i8, z: i8) -> u32 {
        self.block[mini_pos_to_index(x, y, z) as usize]
    }

    pub fn get_block_array_mut(&mut self) -> &mut Vec<u32> {
        &mut self.block
    }

    pub fn get_block_array(&self) -> &Vec<u32> {
        &self.block
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_mesh_id(&self) -> Option<&u32> {
        self.mesh_id.as_ref()
    }

    pub fn set_mesh(&mut self, mcs: &mut MeshComponentSystem, mesh_id: u32) {

        match &self.mesh_id {
            Some(existing_mesh) => {
                // delete existing mesh
                mcs.delete_mesh(*existing_mesh, false);
                self.mesh_id = Some(mesh_id);
            },
            None => self.mesh_id = Some(mesh_id),
        }
    }
    */
}

fn mini_index_to_pos(i: u16) -> (i8,i8,i8) {
    let mut index :u16 = i.clone();
    let x: i8 = (index / 2048) as i8;
    index = index % 2048;
    let z: i8 = (index / 128) as i8;
    index = index % 128;
    let y: i8 = index as i8;
    (x, y, z)
}

pub fn mini_pos_to_index ( x: i8, y: i8, z: i8 ) -> u16 {
    let x_wide: u16 = x as u16;
    let y_wide: u16 = y as u16;
    let z_wide: u16 = z as u16;
    (x_wide * 2048) + (z_wide * 128) + y_wide
}