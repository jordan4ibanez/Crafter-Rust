
use bracket_noise::prelude::FastNoise;
use rand::{Rng, prelude::ThreadRng};

use crate::blocks::block_component_system::BlockComponentSystem;

// Convertes u16 1D position into (u8,u8,u8) 3D tuple position
fn index_to_pos ( i: usize ) -> (f64,f64,f64) {
    let mut index :usize = i.clone();
    let x: f64 = (index / 2048) as f64;
    index = index % 2048;
    let z: f64 = (index / 128) as f64;
    index = index % 128;
    let y: f64 = index as f64;
    (x, y, z)
}

fn calculate_y_height(
    pos_x: f64,
    pos_z: f64,
    chunk_pos_x: f64,
    chunk_pos_z: f64,
    noise: &mut FastNoise, 
    base_height: f64,
    noise_multiplier: f64,
) -> u32 {

    (base_height + (
        noise.get_noise(
            (pos_x + (chunk_pos_x * 16.0)) as f32,
            (pos_z + (chunk_pos_z * 16.0)) as f32)
             * noise_multiplier as f32
        ) as f64
    ) as u32
}



pub fn gen_biome(bcs: &BlockComponentSystem, block_data: &mut Vec<u32>, pos_x: i32, pos_z: i32, perlin: &mut FastNoise, rand_option: Option<&mut ThreadRng>) {

    // let dirt: u32 = bcs.get_id_of(String::from("cobble"));
    let grass: u32 = bcs.get_id_of(String::from("grass"));
    let air: u32 = bcs.get_id_of(String::from("air"));
    let dirt: u32 = bcs.get_id_of(String::from("dirt"));

    let top_layer_thickness = 1;
    let bottom_layer_thickness = (3,4);

    // let stone_layer: u32 = bcs.get_id_of(String::from("stone"));
    let noise_cave_gen_mapping: (f32,f32) = (0.45, 0.55);


    let top_layer: u32 = grass;
    let bottom_layer: u32 = dirt;



    // the base height - if noise is always 0 the blocks will always generate to 0
    let base_height = 70.0;

    // the amount of fluctuation the blocks can have from base height
    let noise_multiplier = 50.0;

    let mut y_height: u32 = calculate_y_height(
        0.0, 
        0.0, 
        pos_x as f64, 
        pos_z as f64, perlin
        ,
        base_height,
        noise_multiplier
    );

    for i in 0..32768 {
        let (x,y,z) = index_to_pos(i);

        let y_u32: u32 = y as u32;

        if y_u32 == 0 {
            y_height = calculate_y_height(x, z, pos_x as f64, pos_z as f64, perlin, base_height, noise_multiplier);
        }
        
        if y_u32 == y_height {
            block_data[i] = top_layer;
        } else if y_u32 < y_height {
            block_data[i] = bottom_layer;
        }
    }
}