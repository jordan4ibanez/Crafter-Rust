use crate::{
    game_debug::chunk_mesh_boilerplate::dry_run,
    world::{
        chunk::Chunk,
        world::World
    }, graphics::mesh_component_system::MeshComponentSystem
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
fn mini_index_to_pos(i: u16) -> (i8,i8,i8) {
    let mut index :u16 = i.clone();
    let x: i8 = (index / 2048) as i8;
    index = index % 2048;
    let z: i8 = (index / 128) as i8;
    index = index % 128;
    let y: i8 = index as i8;
    (x, y, z)
}

// Converts x,y,z (i8) 3D position into u16 1D position.
pub fn mini_pos_to_index ( x: i8, y: i8, z: i8 ) -> u16 {
    let x_wide: u16 = x as u16;
    let y_wide: u16 = y as u16;
    let z_wide: u16 = z as u16;
    (x_wide * 2048) + (z_wide * 128) + y_wide
}






// borrow the entire world
pub fn create_chunk_mesh(mcs: &mut MeshComponentSystem, world: &World,pos_x: i32, pos_z: i32, texture_id: u32) -> Option<u32> {      

    // dry run to get capacities

    let mut float_count: u32 = 0;
    let mut indices_count: u32 = 0;


    let chunk_option: Option<&Chunk> = world.get_chunk(pos_x.to_string() + " " + &pos_z.to_string());

    match chunk_option {
        Some(_) => (),
        None => return None,
    }

    let chunk: &Chunk = chunk_option.unwrap();

    let neighbor_plus_x_option: Option<&Chunk> = world.get_chunk((pos_x + 1).to_string() + " " + &pos_z.to_string());
    let neighbor_minus_x_option: Option<&Chunk> = world.get_chunk((pos_x - 1).to_string() + " " + &pos_z.to_string());

    let neighbor_plus_z_option: Option<&Chunk> = world.get_chunk(pos_x.to_string() + " " + &(pos_z + 1).to_string());
    let neighbor_minus_z_option: Option<&Chunk> = world.get_chunk(pos_x .to_string() + " " + &(pos_z - 1).to_string());

    /*
    match neighbor_minus_x_option {
        Some(_) => println!("YES NEIGHBOR DOES EXIST"),
        None => println!("NO NEIGHBOR DOES NOT EXIST"),
    }
    */


    // slight performance loss at the expense of readibility

    for i in 0..32768 {

        let (x,y,z) = mini_index_to_pos(i as u16);

        if chunk.get_block(x, y, z) != 0 {
            
            // internal
            if x + 1 <= 15 && chunk.get_block(x + 1, y, z) == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }
            if x - 1 >= 0 && chunk.get_block(x - 1, y, z) == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }

            if y == 127 || (y < 127 && chunk.get_block(x, y + 1, z) == 0) {
                dry_run(&mut float_count, &mut indices_count)
            }
            if y - 1 >= 0 && chunk.get_block(x, y - 1, z) == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }

            if z + 1 <= 15 && chunk.get_block(x, y, z + 1) == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }
            if z - 1 >= 0 && chunk.get_block(x, y, z - 1) == 0 {
                dry_run(&mut float_count, &mut indices_count)
            }

            // external

            // x
            if x == 0 {
                match neighbor_minus_x_option {
                    Some(neighbor_minus_x) => {
                        if neighbor_minus_x.get_block(15, y, z) == 0 {
                            dry_run(&mut float_count, &mut indices_count);
                        }
                    },
                    None => (),
                }
            }
            if x == 15 {
                match neighbor_plus_x_option {
                    Some(neighbor_plus_x) => {
                        if neighbor_plus_x.get_block(0, y, z) == 0 {
                            dry_run(&mut float_count, &mut indices_count);
                        }
                    },
                    None => (),
                }
            }

            // z
            if z == 0 {
                match neighbor_minus_z_option {
                    Some(neighbor_minus_z) => {
                        if neighbor_minus_z.get_block(x, y, 15) == 0 {
                            dry_run(&mut float_count, &mut indices_count);
                        }
                    },
                    None => (),
                }
            }
            if z == 15 {
                match neighbor_plus_z_option {
                    Some(neighbor_plus_z) => {
                        if neighbor_plus_z.get_block(x, y, 0) == 0 {
                            dry_run(&mut float_count, &mut indices_count);
                        }
                    },
                    None => (),
                }
            }
        }
    }
    
    // end dry run

    // prevent crashing
    if float_count == 0 {
        return None;
    }

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

        let (x,y,z) = mini_index_to_pos(i as u16);

        if chunk.get_block(x, y, z) != 0 {

            let light = 16.0/16.0;
            
            let mut x_plus = x + 1 <= 15 && chunk.get_block(x + 1, y, z) == 0;
            let mut x_minus = x - 1 >= 0 && chunk.get_block(x - 1, y, z) == 0;

            let y_plus = y == 127 || (y < 127 && chunk.get_block(x, y + 1, z) == 0);
            let y_minus = y - 1 >= 0 && chunk.get_block(x, y - 1, z) == 0;

            let mut z_plus = z + 1 <= 15 && chunk.get_block(x, y, z + 1) == 0;
            let mut z_minus = z - 1 >= 0 && chunk.get_block(x, y, z - 1) == 0;

            // x
            if x == 0 {
                match neighbor_minus_x_option {
                    Some(neighbor_minus_x) => {
                        if neighbor_minus_x.get_block(15, y, z) == 0 {
                            x_minus = true;
                        }
                    },
                    None => (),
                }
            }
            if x == 15 {
                match neighbor_plus_x_option {
                    Some(neighbor_plus_x) => {
                        if neighbor_plus_x.get_block(0, y, z) == 0 {
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
                        if neighbor_minus_z.get_block(x, y, 15) == 0 {
                            z_minus = true;
                        }
                    },
                    None => (),
                }
            }
            if z == 15 {
                match neighbor_plus_z_option {
                    Some(neighbor_plus_z) => {
                        if neighbor_plus_z.get_block(x, y, 0) == 0 {
                            z_plus = true;
                        }
                    },
                    None => (),
                }
            }

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

    let returning_mesh: u32 = mcs.new_mesh(float_data, indices_data, texture_id);

    Some(returning_mesh)
}
