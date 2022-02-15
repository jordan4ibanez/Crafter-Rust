use rand::{prelude::ThreadRng, Rng};

use crate::{graphics::{
    mesh::{
        Mesh,
        self
    },
    texture::{
        self,
        Texture
    }
}, game_debug::chunk_mesh_boilerplate::dry_run};

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

// Convertes u16 1D position into (u8,u8,u8) 3D tuple position
pub fn index_to_pos ( i: &u16 ) -> (f32,f32,f32) {

    let mut index :u16 = i.clone();

    let x: u8 = (index / 2048).try_into().unwrap();

    index = index % 2048;

    let z: u8 = (index / 128).try_into().unwrap();

    index = index % 128;

    let y: u8 = index.try_into().unwrap();

    (x as f32, y as f32, z as f32)

}



pub fn create_chunk_mesh(texture: Texture, randy: &mut ThreadRng) -> Mesh {   

    let mut positions: Vec<f32> = Vec::<f32>::new();

    let mut indices: Vec<i32> = Vec::<i32>::new();

    let mut texture_coordinates: Vec<f32> = Vec::<f32>::new();

    // this is the light attrib in crafter
    let mut colors: Vec<f32> = Vec::<f32>::new();
    

    // dry run to get values

    let mut pos_count = 0;
    let mut indice_count = 0;
    let mut texture_coord_count = 0;
    let mut colors_count = 0;

    for i in 0..32768 {
        for _ in 0..6 {
            dry_run(&mut pos_count, &mut indice_count, &mut texture_coord_count, &mut colors_count)
        }
    }

    println!("CALCULATED: {}", pos_count);

    for i in 0..32768 {

        // if randy.gen::<f32>() > 0.0 {

            
            let light = randy.gen::<f32>();
            let (x,y,z) = index_to_pos(&i);

            
            face_up(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, x, y, z, light);
            
            face_down(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, x, y, z, light);

            face_south(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, x, y, z, light);

            face_north(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, x, y, z, light);

            face_west(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, x, y, z, light);

            face_east(&mut positions, &mut indices, &mut texture_coordinates, &mut colors, x, y, z, light);
        // }
    }

    println!("positions length: {}", positions.len());

    let returning_mesh: Mesh = mesh::new(
        positions,
        colors,
        indices,
        texture_coordinates,
        texture
    );

    returning_mesh
}
