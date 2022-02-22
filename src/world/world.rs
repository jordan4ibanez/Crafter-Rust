use std::{
    collections::{
        hash_map::Values
    }, slice::Iter, iter::Zip, vec
};

use glam::{Vec3, Vec2};

use crate::graphics::mesh_component_system::MeshComponentSystem;


pub struct World {

    position_x: Vec<i32>,
    position_z: Vec<i32>,
    block:      Vec<Vec<u32>>,
    rotation:   Vec<Vec<u8>>,
    light:      Vec<Vec<u8>>,
    heightmap:  Vec<Vec<u8>>,

    // chunks NEED to have data, but their mesh COULD not be generated yet
    mesh_id:    Vec<Option<u32>>,

    sorted_chunks: Vec<(Option<u32>, (i32, i32))>
}

impl World {

    pub fn initialize() -> Self {
        Self {
            position_x: Vec::new(),
            position_z: Vec::new(),
            block:      Vec::new(),
            rotation:   Vec::new(),
            light:      Vec::new(),
            heightmap:  Vec::new(),

            mesh_id:    Vec::new(),
            sorted_chunks: Vec::new()
        }
    }


    pub fn remove_chunk(&mut self, x: i32, z: i32) -> bool {
        match self.get_index(x, z) {
            Some(index) => {
                self.position_x.remove(index);
                self.position_z.remove(index);
            },
            None => return false,
        }

        true
    }
    
    // this is how we get the id - let's walk through it
    fn get_index(&self, x: i32, z: i32) -> Option<usize> {

        // this match statement tells us if we found anything
        match 
        self.position_x
            // iterate all X positions
            .iter()
            // zip them into a simple tuple (x, z)
            .zip(&self.position_z)
            // enumerate that tuple (index, (x,z)), this allows us to work with the index
            .enumerate()
            // we filter all chunks by x and z, then get the first value if found as an option
            .find(| value | {
                *value.1.0 == x && *value.1.1 == z
            }) {
            // now we finally match to see if we found it, zipped_value (index, (x, z)) where .0 is the index
            Some(zipped_value) => return Some(zipped_value.0),
            None => return None,
        }
    }

    pub fn get_pos_from_index(&self, index: usize) -> (i32, i32) {
        (self.position_x[index], self.position_z[index])
    }

    // adds a chunk to the map - returns success
    pub fn add_chunk(&mut self, x: i32, z: i32) -> bool {

        match self.get_index(x, z) {
            Some(_) => {
                return false;
            }
            None => {
                self.position_x.push(x);
                self.position_z.push(z);

                self.block.push(vec![0; 32768]);
                self.rotation.push(vec![0; 32768]);
                self.light.push(vec![0; 32768]);
                self.heightmap.push(vec![0; 256]);
                self.mesh_id.push(None);

                return true;
            }
        }
    }

    pub fn clean_up(&mut self, mcs: &mut MeshComponentSystem){
        self.mesh_id.iter().for_each( | this_mesh_option: &Option<u32> | {
            match this_mesh_option {
                Some(mesh_id) => mcs.delete_mesh(*mesh_id, false),
                None => (),
            }
        });
    }


    fn delete_old_mesh(&mut self, mcs: &mut MeshComponentSystem, index: usize) {
        match self.mesh_id[index] {
            Some(mesh_id) => mcs.delete_mesh(mesh_id, false),
            None => ()
        }
    }

    pub fn set_chunk_mesh(&mut self, mcs: &mut MeshComponentSystem, x: i32, z: i32, mesh_id: u32) {
        // does the chunk exist?
        match self.get_index(x, z) {

            Some(index) => {
                // clean up old mesh
                self.delete_old_mesh(mcs, index);
                // apply new mesh
                self.mesh_id[index] = Some(mesh_id);
            },
            // chunk does not exist, this mesh must be deleted
            None => mcs.delete_mesh(mesh_id, false),
        }
    }
    
    // returns a map iterator
    pub fn iter_map(&self) -> Zip<Zip<Iter<i32>, Iter<i32>>, Iter<Option<u32>>> {
        self.position_x.iter().zip(self.position_z.iter()).zip(self.mesh_id.iter())
    }

    // returns a map iterator
    pub fn sort_map(&mut self, camera_pos: Vec3) {

        // println!("UPDATED CHUNK ORDERING!");

        let mut index = 0;

        self.sorted_chunks.clear();

        for x in self.position_x.iter() {                        
            self.sorted_chunks.push((self.mesh_id[index], (*x, self.position_z[index])));
            index += 1;
        }

        let camera_pos_2d: Vec2 = Vec2::new(camera_pos.x, camera_pos.z);

        self.sorted_chunks.sort_by(|chunk_1, chunk_2| {
            let chunk_worker_vector_1: Vec2 = Vec2::new(
                chunk_1.1.0 as f32 * 16.0,
                chunk_1.1.1 as f32 * 16.0
            );
            let chunk_worker_vector_2: Vec2 = Vec2::new(
                chunk_2.1.0 as f32 * 16.0,
                chunk_2.1.1 as f32 * 16.0
            );
            chunk_worker_vector_2.distance(camera_pos_2d).partial_cmp(&chunk_worker_vector_1.distance(camera_pos_2d)).unwrap()
        });

        //self.sorted_chunks

        //self.position_x.iter().zip(self.position_z.iter()).zip(self.mesh_id.iter())
    }

    pub fn get_map_sorted(&self) -> &Vec<(Option<u32>, (i32, i32))> {
        &self.sorted_chunks
    }

    // returns the vector block data - mutably
    pub fn get_chunk_blocks_mut(&mut self, x: i32, z: i32) -> Option<&mut Vec<u32>> {
        match self.get_index(x, z) {
            Some(index) => return Some(&mut self.block[index]),
            None => None,
        }
    }

    // returns the vector block data - immutably
    pub fn get_chunk_blocks(&self, x: i32, z: i32) -> Option<&Vec<u32>> {
        match self.get_index(x, z) {
            Some(index) => return Some(&self.block[index]),
            None => None,
        }
    }
    
    /*
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
    */

    // removes a chunk from the world
    pub fn remove(&mut self, x: i32, z: i32) {
        match self.get_index(x, z) {
            Some(index) => {
                self.position_x.remove(index);
                self.position_z.remove(index);
                self.block.remove(index);
                self.rotation.remove(index);
                self.light.remove(index);
                self.heightmap.remove(index);
                self.mesh_id.remove(index);
            },
            None => ()
        }
    }

    // gets a chunk
    /*
    pub fn get_chunk(&self, key: String) -> Option<&Chunk> {
        self.map.get(&key)
    }

    // gets a mutable chunk
    pub fn get_chunk_mut(&mut self, key: String) -> &mut Chunk {
        self.map.get_mut(&key).unwrap()
    }
    */
}