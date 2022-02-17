use std::collections::{HashMap, hash_map::Values};

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
        self.map.values().into_iter().for_each( | chunk: &Chunk | {
            match chunk.get_mesh_mut() {
                Some(mesh) => mesh.clean_up(false),
                None => (),
            }
        });
    }
    
    // returns a map iterator
    pub fn iter_map(&self) -> Values<'_, String, chunk::Chunk> {
        self.map.values().into_iter()
    }

    // removes a chunk from the world
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    // gets a chunk
    pub fn get_chunk(&self, key: String) -> &Option<&Chunk> {
        &self.map.get(&key)
    }

    // gets a mutable chunk
    pub fn get_chunk_mut(&mut self, key: String) -> &mut Chunk {
        self.map.get_mut(&key).unwrap()
    }
}