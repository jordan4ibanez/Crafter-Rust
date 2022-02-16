use std::collections::HashMap;

use glam::{Vec2, IVec2};

use super::chunk::Chunk;

pub struct World {
    map: HashMap<String, Chunk>
}

impl World {

    // adds a chunk to the map - returns success
    pub fn add(&mut self, chunk: Chunk) -> bool {
        let key: String = chunk.get_key();
        if !self.map.contains_key(&key) {
            self.map.insert(key, chunk);
            return true;
        }
        false
    }
    
    // returns a map iterator
    pub fn iter_map(&self) -> std::collections::hash_map::Iter<String, Chunk> {
        self.map.iter()
    }

    // removes a chunk from the world
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

