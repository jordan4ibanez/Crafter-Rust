use std::{collections::{HashMap, hash_map::Values}};

use glam::{Vec3, Vec2};

use crate::graphics::mesh::MeshComponentSystem;

use super::chunk::{Chunk, self};

pub struct World {
    map: HashMap<String, Chunk>
}

impl World {

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    // adds a chunk to the map - returns success
    pub fn add(&mut self, chunk: Chunk) -> bool {

        let key: String = chunk.get_key();

        if !self.map.contains_key(&key) {
            self.map.insert(key, chunk);
            return true;
        }
        
        false
    }

    pub fn clean_up(&mut self, mcs: &mut MeshComponentSystem){
        self.map.values_mut().into_iter().for_each( | chunk: &mut Chunk | {
            match chunk.get_mesh_id() {
                Some(mesh_id) => mcs.delete(*mesh_id, false),
                None => (),
            }
        });
    }

    pub fn set_chunk_mesh(&mut self, mcs: &mut MeshComponentSystem, key: String, mesh_id: u32) {
        let chunk_option: Option<&mut Chunk> = self.map.get_mut(&key);
        
        // does the chunk exist?
        match chunk_option {
            Some(chunk) => chunk.set_mesh(mcs, mesh_id),
            None => mcs.delete(mesh_id, false),
        }
    }
    
    // returns a map iterator
    pub fn iter_map(&self) -> Values<'_, String, chunk::Chunk> {
        self.map.values().into_iter()
    }
    
    pub fn iter_map_sorted(&self, camera_pos: Vec3) -> Vec<&Chunk> {

        let camera_pos_2d: Vec2 = Vec2::new(camera_pos.x, camera_pos.z);

        let mut sorted_vec: Vec<&Chunk> = Vec::from_iter(self.map.values());

        sorted_vec.sort_by(|chunk_1, chunk_2 |{
            chunk_1.get_pos().as_vec2()
                        .distance(camera_pos_2d)
                        .partial_cmp(
                            &chunk_2.get_pos().as_vec2().distance(camera_pos_2d)
                        ).unwrap()
        });

        sorted_vec
    }

    // removes a chunk from the world
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    // gets a chunk
    pub fn get_chunk(&self, key: String) -> Option<&Chunk> {
        self.map.get(&key)
    }

    // gets a mutable chunk
    pub fn get_chunk_mut(&mut self, key: String) -> &mut Chunk {
        self.map.get_mut(&key).unwrap()
    }
}