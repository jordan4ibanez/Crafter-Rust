use std::thread;

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
    face_east, add_block
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

    // dry run to get capacities

    let mut pos_count = 0;
    let mut indice_count = 0;
    let mut texture_coord_count = 0;
    let mut colors_count = 0;

    let mut debug_array: [bool; 32768] = [false; 32768];

    for i in 0..32768 {

        debug_array[i] = randy.gen::<f32>() > 0.5;

        if debug_array[i] {
            for _ in 0..6 {
                dry_run(&mut pos_count, &mut indice_count, &mut texture_coord_count, &mut colors_count)
            }
        }
    }
    
    let mut positions: Vec<f32> = vec![0.0; pos_count as usize];

    let mut indices: Vec<i32> = vec![0; indice_count as usize];

    let mut texture_coordinates: Vec<f32> = vec![0.0; texture_coord_count as usize];

    // this is the light attrib in crafter
    let mut colors: Vec<f32> = vec![0.0; colors_count as usize];


    // println!("CALCULATED: {}", pos_count);


    // create the counters
    let mut pos_count: i32 = 0;
    let mut indice_count: i32 = 0;
    let mut texture_count: i32 = 0;
    let mut color_count: i32 = 0;

    // this part is EXTREMELY important, this allows all the vertex points to link together
    let mut face_count: i32 = 0;

    for i in 0..32768 {

        if debug_array[i as usize] {

            
            let light = randy.gen::<f32>();
            let (x,y,z) = index_to_pos(&i);

            add_block(
                &mut positions,
                &mut indices,
                &mut texture_coordinates,
                &mut colors,
        
                &mut pos_count,
                &mut indice_count,
                &mut texture_count,
                &mut color_count,
                &mut face_count,
        
                x,
                y,
                z,
                light
            );

        }
    }

    let returning_mesh: Mesh = mesh::new(
        positions,
        colors,
        indices,
        texture_coordinates,
        texture
    );

    returning_mesh
}
