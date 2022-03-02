
use bracket_noise::prelude::{FastNoise, NoiseType, Interp};
use rand::{Rng, prelude::ThreadRng};

use crate::blocks::block_component_system::BlockComponentSystem;

use super::generation_component_system::GenerationComponentSystem;

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

fn calculate_depth(
    pos_x: f64,
    pos_z: f64,
    chunk_pos_x: f64,
    chunk_pos_z: f64,
    noise: &mut FastNoise, 
    min: u8,
    max: u8
) -> u32{

    ((noise.get_noise(
        (pos_x + (chunk_pos_x * 16.0)) as f32,
        (pos_z + (chunk_pos_z * 16.0)) as f32
    ).abs() * (max - min) as f32) + min as f32).floor() as u32
}

fn carve_cave(
    pos_x: f64,
    pos_y: f64,
    pos_z: f64,
    chunk_pos_x: f64,
    chunk_pos_z: f64,
    noise: &mut FastNoise, 
) -> f32{
    noise.get_noise3d(
        (pos_x + (chunk_pos_x * 16.0)) as f32,
        pos_y as f32,
        (pos_z + (chunk_pos_z * 16.0)) as f32
    )
}




pub fn gen_biome(
    gcs: &GenerationComponentSystem,
    bcs: &BlockComponentSystem,
    block_data: &mut Vec<u32>,
    pos_x: i32,
    pos_z: i32,
    simplex_noise: &mut FastNoise,
    cellular_noise: &mut FastNoise
) {

    // this is debug
    let (
        name,
        top_layer,
        top_layer_depth,
        bottom_layer,
        bottom_layer_depth,
        stone_layer,
        terrain_noise_multiplier,
        terrain_frequency,
        caves,
        cave_heat,
        rain,
        snow
    ) = gcs.get(0);

    simplex_noise.set_frequency(terrain_frequency);

    // the base height - if noise is always 0 the blocks will always generate to 0
    let base_height = 70.0;

    // the amount of fluctuation the blocks can have from base height
    //let noise_multiplier = 50.0;

    let mut y_height: u32 = calculate_y_height(
        0.0, 
        0.0, 
        pos_x as f64, 
        pos_z as f64,
        simplex_noise,
        base_height,
        terrain_noise_multiplier as f64
    );

    let mut top_layer_depth_random: u32 = calculate_depth(
        0.0, 
        0.0, 
        pos_x as f64, 
        pos_z as f64,
        simplex_noise, 
        top_layer_depth.get_min(),
        top_layer_depth.get_max() + 1
    );
    

    let mut bottom_layer_depth_random: u32 = calculate_depth(
        0.0, 
        0.0, 
        pos_x as f64, 
        pos_z as f64,
        simplex_noise, 
        bottom_layer_depth.get_min(),
        bottom_layer_depth.get_max() + 1
    );    

    // generate unmodified terrain
    for i in 0..32768 {
        let (x,y,z) = index_to_pos(i);

        let y_u32: u32 = y as u32;

        if y_u32 == 0 {
            y_height = calculate_y_height(x, z, pos_x as f64, pos_z as f64, simplex_noise, base_height, terrain_noise_multiplier as f64);

            top_layer_depth_random = calculate_depth(
                x, 
                z, 
                pos_x as f64, 
                pos_z as f64,
                simplex_noise, 
                top_layer_depth.get_min(),
                top_layer_depth.get_max() + 1
            );

            bottom_layer_depth_random = calculate_depth(
                x, 
                z,
                pos_x as f64, 
                pos_z as f64,
                simplex_noise, 
                bottom_layer_depth.get_min(),
                bottom_layer_depth.get_max() + 1
            );
        }

        // only calculate when inside possible parameter
        if y_u32 <= y_height {        
            // top layer
            if y_u32 >= y_height - top_layer_depth_random {
                block_data[i] = top_layer;
            }
            // bottom layer
            else if y_u32 < y_height - top_layer_depth_random &&  y_u32 >= y_height - top_layer_depth_random - bottom_layer_depth_random {
                block_data[i] = bottom_layer;
            }
            // stone layer
            else if y_u32 < y_height - top_layer_depth_random - bottom_layer_depth_random {
                block_data[i] = stone_layer;
            }
        }
    }

    
    // generate caves if defined in biome
    if caves {


        let (cave_heat_min, cave_heat_max, cave_frequency) = cave_heat.get();

        cellular_noise.set_frequency(cave_frequency);

        for i in 0..32768 {
            let (x,y,z) = index_to_pos(i);
            
            // noise blend
            let mut cave_gen_noise: f32 = carve_cave(x, y, z, pos_x as f64, pos_z as f64, cellular_noise);
            cave_gen_noise += carve_cave(x, y, z, pos_x as f64, pos_z as f64, simplex_noise);
            cave_gen_noise /= 2.0;


            // out of bounds of cave noise parameters
            if cave_gen_noise >= cave_heat_min && cave_gen_noise <= cave_heat_max {
                block_data[i] = 0;
            }   
        }
    }    
}