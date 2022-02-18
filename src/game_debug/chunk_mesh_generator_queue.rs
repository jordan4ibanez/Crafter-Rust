
// mesh updates hold data on which chunks should be updated
struct MeshUpdate {
    key: String,
    update_neighbors: bool
}

impl MeshUpdate {
    pub fn get_key(&self) -> &String {
        &self.key
    }
    pub fn update_neighbors(&self) -> bool {
        self.update_neighbors
    }
}

struct ChunkMeshGeneratorQueue {
    queue: Vec<MeshUpdate>
}

// this is a lot like Deque in Java - at least the parts that I use
impl ChunkMeshGeneratorQueue {
    pub fn new() -> Self {
        ChunkMeshGeneratorQueue {
            queue: Vec::new(),
        }
    }
    pub fn put(&mut self, x: i32, z: i32, update_neighbors: bool){
        let update: MeshUpdate = MeshUpdate {
            key: x.to_string() + " " + &z.to_string(),
            update_neighbors,
        };
        self.queue.push(update);
    }
    pub fn pop(&mut self) -> Option<MeshUpdate> {
        self.queue.pop()
    }
}