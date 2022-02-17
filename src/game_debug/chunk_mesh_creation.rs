use crate::{graphics::{
    mesh::{
        Mesh
    },
    texture::{
        Texture
    }
}, game_debug::chunk_mesh_boilerplate::dry_run, world::chunk::Chunk};

use super::chunk_mesh_boilerplate::{
    add_block
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

    let x: f32 = (index / 2048) as f32;

    index = index % 2048;

    let z: f32 = (index / 128) as f32;

    index = index % 128;

    let y: f32 = index as f32;

    (x as f32, y as f32, z as f32)

}



pub fn create_chunk_mesh(chunk: &Chunk, texture: Texture) -> Mesh {      

    // dry run to get capacities

    let mut float_count: u32 = 0;
    let mut indices_count: u32 = 0;

    let mut debug_array: [u32; 32768] = *chunk.get_block_aray();

    for i in 0..32768 {
        if debug_array[i] != 0 {
            for _ in 0..6 {
                dry_run(&mut float_count, &mut indices_count)
            }
        }
    }
    
    // end dry run


    // println!("CALCULATED: {}", pos_count);

    // create the vectors with predetermined size
    let mut float_data: Vec<f32> = vec![0.0; float_count as usize];
    let mut indices_data: Vec<u32> = vec![0; indices_count as usize];


    // reset the counters
    float_count = 0;
    indices_count = 0;

    // this part is EXTREMELY important, this allows all the vertex points to link together
    let mut face_count: u32 = 0;

    for i in 0..32768 {

        if debug_array[i] != 0 {

            let (x,y,z) = index_to_pos(&(i as u16) as &u16);

            let light = 1.0;

            add_block(
                &mut float_data,
                &mut indices_data,

                &mut float_count,
                &mut face_count,
                &mut indices_count,
        
                x,
                y,
                z,
                light
            );

        }
    }

    let returning_mesh: Mesh = Mesh::new(
        float_data,
        indices_data,
        texture
    );

    returning_mesh
}
