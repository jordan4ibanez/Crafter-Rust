use std::{collections::{HashMap, hash_map::Values}, iter::Map};

use glam::{Vec3, Vec3Swizzles};

use crate::graphics::mesh::Mesh;

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

    pub fn clean_up(&mut self){
        self.map.values_mut().into_iter().for_each( | chunk: &mut Chunk | {
            match chunk.get_mesh_mut() {
                Some(mesh) => mesh.clean_up(false),
                None => (),
            }
        });
    }

    pub fn set_chunk_mesh(&mut self, key: String, mesh: Mesh) {
        let chunk_option: Option<&mut Chunk> = self.map.get_mut(&key);
        
        // does the chunk exist?
        match chunk_option {
            Some(chunk) => chunk.set_mesh(mesh),
            None => mesh.clean_up(false),
        }
    }
    
    // returns a map iterator
    pub fn iter_map(&self) -> Values<'_, String, chunk::Chunk> {
        self.map.values().into_iter()
    }
    
    pub fn iter_map_sorted(&self, camera_pos: Vec3) -> Vec<&Chunk> {

        let mut chunk_worker_vector_1: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut chunk_worker_vector_2: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        let mut sorted_vec: Vec<&Chunk> = Vec::from_iter(self.map.values());

        sorted_vec.sort_by(|chunk_1, chunk_2 |{

            chunk_worker_vector_1.x = chunk_1.get_x() as f32 * 16.0;
            // 2d so no Y
            chunk_worker_vector_1.z = chunk_1.get_z() as f32 * 16.0;

            chunk_worker_vector_2.x = chunk_2.get_x() as f32 * 16.0;
            // 2d so no Y
            chunk_worker_vector_2.z = chunk_2.get_z() as f32 * 16.0;

            chunk_worker_vector_2.distance(camera_pos).partial_cmp(&chunk_worker_vector_1.distance(camera_pos)).unwrap()
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