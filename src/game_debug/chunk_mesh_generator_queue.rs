use std::collections::VecDeque;


// mesh updates hold data on which chunks should be updated
pub struct MeshUpdate {
    x: i32,
    z: i32,
    update_neighbors: bool
}

impl MeshUpdate {
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }
    pub fn update_neighbors(&self) -> bool {
        self.update_neighbors
    }
}

pub struct ChunkMeshGeneratorQueue {
    queue: VecDeque<MeshUpdate>
}

// this is a lot like Deque in Java - at least the parts that I use
impl ChunkMeshGeneratorQueue {
    pub fn new() -> Self {
        ChunkMeshGeneratorQueue {
            queue: VecDeque::new(),
        }
    }
    // stores an update for all neighbors - does not recursively generate more
    pub fn batch_neighbor_update(&mut self, x: i32, z: i32){
        self.push_front(x+1, z, false);
        self.push_front(x-1, z, false);
        self.push_front(x, z+1, false);
        self.push_front(x, z-1, false);
    }

    pub fn push_front(&mut self, x: i32, z: i32, update_neighbors: bool){
        let update: MeshUpdate = MeshUpdate {
            x,
            z,
            update_neighbors,
        };
        self.queue.push_front(update);
    }

    pub fn push_back(&mut self, x: i32, z: i32, update_neighbors: bool){
        let update: MeshUpdate = MeshUpdate {
            x,
            z,
            update_neighbors,
        };
        self.queue.push_back(update);
    }

    pub fn pop_front(&mut self) -> Option<MeshUpdate> {
        self.queue.pop_front()
    }
    pub fn pop_back(&mut self) -> Option<MeshUpdate> {
        self.queue.pop_back()
    }
}