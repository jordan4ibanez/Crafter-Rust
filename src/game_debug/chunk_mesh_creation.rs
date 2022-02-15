use crate::graphics::{
    mesh::{
        Mesh,
        self
    },
    texture::{
        self,
        Texture
    }
};

use super::chunk_mesh_boilerplate::{
    self,
    face_up,
    face_down,
    face_south,
    face_north,
    face_west,
    face_east
};

/*
positions,
colors,
indices,
texture_coordinates,
this_texture
*/




pub fn create_chunk_mesh(path: &str) -> Mesh {   

    let mut positions: Vec<f32> = Vec::<f32>::new();

    let mut indices: Vec<i32> = Vec::<i32>::new();

    let mut texture_coordinates: Vec<f32> = Vec::<f32>::new();

    // this is the light attrib in crafter
    let mut colors: Vec<f32> = Vec::<f32>::new();

    // for i in 0..100 {
    // }

    face_up(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, 0.0, 0.0, 0.0);

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
