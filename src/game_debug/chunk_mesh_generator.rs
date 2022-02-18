
struct MeshUpdate {
    key: String,
    update_neighbors: bool
}

struct ChunkMeshGenerator {
    queue: Vec<MeshUpdate>
}