use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use crate::{
    chunk_mesh_procedure::chunk_mesh_boilerplate::dry_run,
    world::{
        world::World
    }, graphics::mesh_component_system::MeshComponentSystem, blocks::block_component_system::BlockComponentSystem
};

use super::chunk_mesh_boilerplate::{
    add_block
};

// this is procedurally generated

/*
positions,
colors,
indices,
texture_coordinates,
this_texture
*/


// Convertes u16 1D position into (i8,i8,i8) 3D tuple position
fn index_to_pos(i: usize) -> (usize,usize,usize) {
    (i / 2048,
    (i % 2048) % 128,
    (i % 2048) / 128)
}

// Converts x,y,z (i8) 3D position into u16 1D position.
pub fn pos_to_index ( x: usize, y: usize, z: usize ) -> usize {
    (x * 2048) + (z * 128) + y
}






// borrow the entire world
pub fn create_chunk_mesh(bcs: &BlockComponentSystem, mcs: &mut MeshComponentSystem, world: &World,pos_x: i32, pos_z: i32, texture_id: u32) -> Option<u32> {      

    // dry run to get capacities

    let block_vector_option: Option<&[u32]> = world.get_chunk_blocks_slice(pos_x, pos_z);

    match block_vector_option {
        Some(_) => (),
        None => return None,
    }


    let float_count: AtomicUsize = AtomicUsize::new(0);
    let indices_count: AtomicUsize = AtomicUsize::new(0);

    let chunk: &[u32] = block_vector_option.unwrap();

    let neighbor_plus_x_option: Option<&[u32]> = world.get_chunk_blocks_slice(pos_x + 1, pos_z);
    let neighbor_minus_x_option: Option<&[u32]> = world.get_chunk_blocks_slice(pos_x - 1, pos_z);

    let neighbor_plus_z_option: Option<&[u32]> = world.get_chunk_blocks_slice(pos_x ,pos_z + 1);
    let neighbor_minus_z_option: Option<&[u32]> = world.get_chunk_blocks_slice(pos_x, pos_z - 1);


    chunk.par_iter().enumerate().for_each( | ( index, value ) | {

        // if it does not equal air
        if *value != 0 {

            let (x,y,z) = index_to_pos(index);
            
            // internal
            if x + 1 <= 15 && chunk[pos_to_index(x + 1, y, z)] == 0 {
                dry_run(&float_count, &indices_count)
            }
            if x >= 1 && chunk[pos_to_index(x - 1, y, z)] == 0 {
                dry_run(&float_count, &indices_count)
            }

            if y == 127 || (y < 127 && chunk[pos_to_index(x, y + 1, z)] == 0) {
                dry_run(&float_count, &indices_count)
            }
            if y > 0 && y - 1 >= 1 && chunk[pos_to_index(x, y - 1, z)] == 0 {
                dry_run(&float_count, &indices_count)
            }

            if z + 1 <= 15 && chunk[pos_to_index(x, y, z + 1)] == 0 {
                dry_run(&float_count, &indices_count)
            }
            if z >= 1 && chunk[pos_to_index(x, y, z - 1)] == 0 {
                dry_run(&float_count, &indices_count)
            }

            // external

            // x
            if x == 0 {
                match neighbor_minus_x_option {
                    Some(neighbor_minus_x) => {
                        if neighbor_minus_x[pos_to_index(15, y, z)] == 0 {
                            dry_run(&float_count, &indices_count);
                        }
                    },
                    None => (),
                }
            }
            if x == 15 {
                match neighbor_plus_x_option {
                    Some(neighbor_plus_x) => {
                        if neighbor_plus_x[pos_to_index(0, y, z)] == 0 {
                            dry_run(&float_count, &indices_count);
                        }
                    },
                    None => (),
                }
            }

            // z
            if z == 0 {
                match neighbor_minus_z_option {
                    Some(neighbor_minus_z) => {
                        if neighbor_minus_z[pos_to_index(x, y, 15)] == 0 {
                            dry_run(&float_count, &indices_count);
                        }
                    },
                    None => (),
                }
            }
            if z == 15 {
                match neighbor_plus_z_option {
                    Some(neighbor_plus_z) => {
                        if neighbor_plus_z[pos_to_index(x, y, 0)] == 0 {
                            dry_run(&float_count, &indices_count);
                        }
                    },
                    None => (),
                }
            }
        }
    });
    
    // end dry run

    // prevent crashing
    if float_count.load(Ordering::Relaxed) == 0 {
        return None;
    }

    // println!("CALCULATED: {}", pos_count);

    // create the vectors with predetermined size
    let mut float_data: Vec<f32> = vec![0.0; float_count.load(Ordering::Relaxed)];
    let mut indices_data: Vec<u32> = vec![0; indices_count.load(Ordering::Relaxed)];


    // reset the counters
    let mut new_float_count = 0;
    let mut new_indices_count = 0;

    // this part is EXTREMELY important, this allows all the vertex points to link together
    let mut face_count: usize = 0;

    chunk.iter().enumerate().for_each(|(index, value)| {

        // let block_id: u32 = chunk[pos_to_index(x, y, z)];

        // if it does not equal air
        if *value != 0 {

            let (x,y,z) = index_to_pos(index);

            let light = 16.0/16.0;
            
            let mut x_plus = x + 1 <= 15 && chunk[pos_to_index(x + 1, y, z)] == 0;
            let mut x_minus =    x >= 1  && chunk[pos_to_index(x - 1, y, z)] == 0;

            let y_plus = y == 127 || (y < 127 && chunk[pos_to_index(x, y + 1, z)] == 0);
            let y_minus = y > 0 && y - 1 >= 1 && chunk[pos_to_index(x, y - 1, z)] == 0;

            let mut z_plus = z + 1 <= 15 && chunk[pos_to_index(x, y, z + 1)] == 0;
            let mut z_minus =    z >= 1  && chunk[pos_to_index(x, y, z - 1)] == 0;

            // x
            if x == 0 {
                match neighbor_minus_x_option {
                    Some(neighbor_minus_x) => {
                        if neighbor_minus_x[pos_to_index(15, y, z)] == 0 {
                            x_minus = true;
                        }
                    },
                    None => (),
                }
            }
            if x == 15 {
                match neighbor_plus_x_option {
                    Some(neighbor_plus_x) => {                        
                        if neighbor_plus_x[pos_to_index(0, y, z)] == 0 {
                            x_plus = true;
                        }
                    },
                    None => (),
                }
            }

            // z
            if z == 0 {
                match neighbor_minus_z_option {
                    Some(neighbor_minus_z) => {
                        if neighbor_minus_z[pos_to_index(x, y, 15)] == 0 {
                            z_minus = true;
                        }
                    },
                    None => (),
                }
            }
            if z == 15 {
                match neighbor_plus_z_option {
                    Some(neighbor_plus_z) => {
                        if neighbor_plus_z[pos_to_index(x, y, 0)] == 0 {
                            z_plus = true;
                        }
                    },
                    None => (),
                }
            }

            if x_plus || x_minus || y_plus || y_minus || z_plus || z_minus {
                add_block(
                    bcs.get_mapping(*value),
                    &mut float_data,
                    &mut indices_data,

                    &mut new_float_count,
                    &mut face_count,
                    &mut new_indices_count,

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
    });

    let returning_mesh: u32 = mcs.new_mesh(float_data, indices_data, texture_id);

    Some(returning_mesh)
}
