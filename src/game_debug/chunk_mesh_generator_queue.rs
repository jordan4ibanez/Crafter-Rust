
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
    queue: Vec<MeshUpdate>
}

// this is a lot like Deque in Java - at least the parts that I use
impl ChunkMeshGeneratorQueue {
    pub fn new() -> Self {
        ChunkMeshGeneratorQueue {
            queue: Vec::new(),
        }
    }
    // stores an update for all neighbors - does not recursively generate more
    pub fn batch_neighbor_update(&mut self, x: i32, z: i32){
        let update_1: MeshUpdate = MeshUpdate {
            x: x + 1,
            z,
            update_neighbors: false
        };
        let update_2: MeshUpdate = MeshUpdate {
            x: x - 1,
            z,
            update_neighbors: false
        };
        let update_3: MeshUpdate = MeshUpdate {
            x,
            z: z + 1,
            update_neighbors: false
        };
        let update_4: MeshUpdate = MeshUpdate {
            x,
            z: z - 1,
            update_neighbors: false
        };

        self.queue.push(update_1);
        self.queue.push(update_2);
        self.queue.push(update_3);
        self.queue.push(update_4);
    }
    pub fn put(&mut self, x: i32, z: i32, update_neighbors: bool){
        let update: MeshUpdate = MeshUpdate {
            x,
            z,
            update_neighbors,
        };
        self.queue.push(update);
    }
    pub fn pop(&mut self) -> Option<MeshUpdate> {
        self.queue.pop()
    }
}