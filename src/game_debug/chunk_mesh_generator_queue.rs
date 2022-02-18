
// mesh updates hold data on which chunks should be updated
struct MeshUpdate {
    key: String,
    update_neighbors: bool
}

struct ChunkMeshGenerator {
    queue: Vec<MeshUpdate>
}