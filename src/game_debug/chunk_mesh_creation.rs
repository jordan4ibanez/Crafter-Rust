use crate::graphics::{mesh::{Mesh, self}, texture::{self, Texture}};


pub fn create_chunk_mesh(path: &str) -> Mesh {
    
    // this is the light attrib in crafter
    let colors: Vec<f32> = vec![
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    let indices: Vec<i32> = vec![
        // Front face
        0, 1, 3, 3, 1, 2,
        // Top Face
        4, 0, 3, 5, 4, 3,
        // Right face
        3, 2, 7, 5, 3, 7,
        // Left face
        6, 1, 0, 6, 0, 4,
        // Bottom face
        2, 1, 6, 2, 6, 7,
        // Back face
        7, 6, 4, 7, 4, 5,
    ];

    let positions: Vec<f32> = vec![        
        // VO
        -0.5,  0.5,  0.5,
        // V1
        -0.5, -0.5,  0.5,
        // V2
        0.5,  -0.5,  0.5,
        // V3
        0.5,   0.5,  0.5,
        // V4
        -0.5,  0.5, -0.5,
        // V5
        0.5,   0.5, -0.5,
        // V6
        -0.5, -0.5, -0.5,
        // V7
        0.5,  -0.5, -0.5,
    ];

    let texture_coordinates: Vec<f32> = vec![
        1.0, 1.0, //bottom right
        0.0, 1.0, //bottom left
        0.0, 0.0, //top left
        1.0, 0.0, //top right


        1.0, 1.0, //bottom right
        0.0, 1.0, //bottom left
        0.0, 0.0, //top left
        1.0, 0.0, //top right
        
    ];

    let this_texture: Texture = texture::new(path.to_string() + "/textures/debug.png");

    let returning_mesh: Mesh = mesh::new(
        positions,
        colors,
        indices,
        texture_coordinates,
        this_texture
    );

    returning_mesh
}
