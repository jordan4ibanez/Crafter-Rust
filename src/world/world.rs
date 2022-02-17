use std::collections::HashMap;

use super::chunk::Chunk;

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
        self.map.iter_mut().for_each(| this_chunk |{
            match this_chunk.1.get_mesh_mut() {
                Some(mesh) => mesh.clean_up(false),
                None => (),
            }
        });
    }
    
    // returns a map iterator
    pub fn iter_map(&self) -> std::collections::hash_map::Iter<String, Chunk> {
        self.map.iter()
    }

    // removes a chunk from the world
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    // gets a chunk
    pub fn get_chunk(&self, key: String) -> &Chunk {
        &self.map.get(&key).unwrap()
    }

    // gets a mutable chunk
    pub fn get_chunk_mut(&mut self, key: String) -> &mut Chunk {
        self.map.get_mut(&key).unwrap()
    }
}