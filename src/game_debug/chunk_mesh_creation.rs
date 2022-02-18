use crate::{graphics::{
    mesh::{
        Mesh
    },
    texture::{
        Texture
    }
}, game_debug::chunk_mesh_boilerplate::dry_run, world::{chunk::Chunk, world::World}};

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
fn index_to_pos ( i: &u16 ) -> (i32,i32,i32) {
    let mut index :u16 = i.clone();
    let x: i32 = (index / 2048) as i32;
    index = index % 2048;
    let z: i32 = (index / 128) as i32;
    index = index % 128;
    let y: i32 = index as i32;
    (x, y, z)
}

// Converts x,y,z (u8) 3D position into u16 1D position.
pub fn pos_to_index ( x: u8, y: u8, z: u8 ) -> u16 {
    let x_wide: u16 = x.clone().into();
    let y_wide: u16 = y.clone().into();
    let z_wide: u16 = z.clone().into();
    (x_wide * 2048) + (z_wide * 128) + y_wide
}

fn mini_index_to_pos(i: u16) -> (i8,i8,i8) {
    let mut index :u16 = i.clone();
    let x: i8 = (index / 2048) as i8;
    index = index % 2048;
    let z: i8 = (index / 128) as i8;
    index = index % 128;
    let y: i8 = index as i8;
    (x, y, z)
}

pub fn mini_pos_to_index ( x: i8, y: i8, z: i8 ) -> u16 {
    let x_wide: u16 = x as u16;
    let y_wide: u16 = y as u16;
    let z_wide: u16 = z as u16;
    (x_wide * 2048) + (z_wide * 128) + y_wide
}






// borrow the entire world
pub fn create_chunk_mesh(world: &World,pos_x: i32, pos_z: i32, texture: Texture) -> Option<Mesh> {      

    // dry run to get capacities

    let mut float_count: u32 = 0;
    let mut indices_count: u32 = 0;


    let current_chunk_option: Option<&Chunk> = world.get_chunk(pos_x.to_string() + " " + &pos_z.to_string());

    match current_chunk_option {
        Some(_) => (),
        None => return None,
    }

    let chunk_data: &[u32; 32768] = current_chunk_option.unwrap().get_block_aray();


    for i in 0..32768 {
        if chunk_data[i] != 0 {
            let (x,y,z) = mini_index_to_pos(i as u16);
            
            if x + 1 <= 15 && chunk_data[mini_pos_to_index(x + 1, y, z) as usize] == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }
            if x - 1 >= 0 && chunk_data[mini_pos_to_index(x - 1, y, z) as usize] == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }

            if y == 127 || (y < 127 && chunk_data[mini_pos_to_index(x, y + 1, z) as usize] == 0) {
                dry_run(&mut float_count, &mut indices_count)
            }
            if y - 1 >= 0 && chunk_data[mini_pos_to_index(x, y - 1, z) as usize] == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }

            if z + 1 <= 15 && chunk_data[mini_pos_to_index(x, y, z + 1) as usize] == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }
            if z - 1 >= 0 && chunk_data[mini_pos_to_index(x, y, z - 1) as usize] == 0 {
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
        if chunk_data[i] != 0 {

            let light = 1.0;

            let (x,y,z) = mini_index_to_pos(i as u16);
            
            let x_plus = x + 1 <= 15 && chunk_data[mini_pos_to_index(x + 1, y, z) as usize] == 0;
            let x_minus = x - 1 >= 0 && chunk_data[mini_pos_to_index(x - 1, y, z) as usize] == 0;

            let y_plus = y == 127 || (y < 127 && chunk_data[mini_pos_to_index(x, y + 1, z) as usize] == 0);
            let y_minus = y - 1 >= 0 && chunk_data[mini_pos_to_index(x, y - 1, z) as usize] == 0;

            let z_plus = z + 1 <= 15 && chunk_data[mini_pos_to_index(x, y, z + 1) as usize] == 0;
            let z_minus = z - 1 >= 0 && chunk_data[mini_pos_to_index(x, y, z - 1) as usize] == 0;

            if x_plus || x_minus || y_plus || y_minus || z_plus || z_minus {
                add_block(
                    &mut float_data,
                    &mut indices_data,

                    &mut float_count,
                    &mut face_count,
                    &mut indices_count,

                    x_plus,
                    x_minus,
                    y_plus,
                    y_minus,
                    z_plus,
                    z_minus,
            
                    x as f32,
                    y as f32,
                    z as f32,
                    light
                );
            }
        }
    }

    let returning_mesh: Mesh = Mesh::new(
        float_data,
        indices_data,
        texture
    );

    Some(returning_mesh)
}
