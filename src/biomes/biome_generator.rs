
use opensimplex_noise_rs::OpenSimplexNoise;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};

use crate::SEED;

use super::generation_component_system::GenerationComponentSystem;

// Convertes u16 1D position into (u8,u8,u8) 3D tuple position
fn index_to_pos ( i: usize ) -> (f64,f64,f64) {
    ((i / 2048) as f64, ((i % 2048) % 128) as f64, ((i % 2048) / 128) as f64)
}

fn calculate_y_height(noise_input: f64, base_height: f64, terrain_height_flux: f64) -> u32 {
    ((noise_input * terrain_height_flux) as i32 + base_height as i32) as u32
}

fn calculate_depth(
    noise_input: f64,
    min: u8,
    max: u8
) -> u32 {
    ((noise_input *
    (max - min) as f64)
    + min as f64)
    .floor()
    as u32
}

fn gen_3d(noise: &OpenSimplexNoise, x: f64, y: f64, z: f64, frequency: f64, scale: f64) -> f64 {
    noise.eval_3d(x * frequency, y * frequency, z * frequency) * scale
}

fn gen_2d(noise: &OpenSimplexNoise, x: f64, z: f64, frequency: f64, scale: f64) -> f64 {
    noise.eval_2d(x * frequency, z * frequency) * scale
}


pub fn gen_biome(
    gcs: &GenerationComponentSystem,
    block_data: &mut Vec<u32>,
    pos_x: i32,
    pos_z: i32,
) {

    // this is debug
    let (
        _,
        biome_noise_params,
        terrain_height_flux,
        top_layer,
        top_layer_depth,
        bottom_layer,
        bottom_layer_depth,
        stone_layer,
        bedrock_layer,
        biome_ores_option,
        caves,
        cave_noise_params,
        rain,
        snow
    ) = gcs.get(0);
    
    // let simplex_noise: &mut FastNoise = &mut *simplex_noise;

    // simplex_noise.set_frequency(terrain_frequency);

    // the base height - if noise is always 0 the blocks will always generate to 0
    let base_height = 90.0;

    // the amount of fluctuation the blocks can have from base height
    //let noise_multiplier = 50.0;


    let (biome_heat_min, biome_heat_max, biome_scale, biome_frequency) = biome_noise_params.get();

    let (cave_heat_min, cave_heat_max, cave_scale, cave_frequency) = cave_noise_params.get();

    // noise structure
    let noise = OpenSimplexNoise::new(Some(SEED as i64));

    // generate unmodified terrain
    block_data.par_iter_mut().enumerate().for_each(| (index, value) | {

        let (mut x, y, mut z) = index_to_pos(index);

        x += pos_x as f64 * 16.0;
        z += pos_z as f64 * 16.0;        

        let y_u32: u32 = y as u32;

        let terrain_2d_noise = gen_2d(&noise, x, z, biome_frequency as f64, biome_scale as f64);
        let terrain_3d_noise = gen_3d(&noise, x, y, z, biome_frequency as f64, biome_scale as f64);

        let cave_3d_noise = gen_3d(&noise, x, y, z, cave_frequency as f64, cave_scale as f64);
                
        let y_height = calculate_y_height(terrain_2d_noise, base_height, terrain_height_flux as f64);

        let bedrock_3d_noise = gen_3d(&noise, x, y, z, 1.5, 0.2);
        

        let top_layer_depth_random = calculate_depth(
            terrain_2d_noise,
            top_layer_depth.get_min(),
            top_layer_depth.get_max() + 1
        );

        let bottom_layer_depth_random = calculate_depth(
            terrain_2d_noise,
            bottom_layer_depth.get_min(),
            bottom_layer_depth.get_max() + 1
        );

        // only calculate when inside possible parameter
        if y_u32 <= y_height {

            let mut bedrock = false;

            if y_u32 <= 2 {
                if y_u32 == 0 {

                    bedrock = true;

                } else {

                    if bedrock_3d_noise > 0.0 {
                        bedrock = true;
                    }

                    //simplex_noise.set_frequency(terrain_frequency);
                }
            }

            if bedrock {

                *value = bedrock_layer;

            } else {

                if caves && (cave_3d_noise >= cave_heat_min as f64 && cave_3d_noise <= cave_heat_max as f64) {
                    *value = 0;
                } else {
                    // top layer
                    if y_u32 >= y_height - top_layer_depth_random {
                        *value = top_layer;
                    }
                    // bottom layer
                    else if y_u32 < y_height - top_layer_depth_random &&  y_u32 >= y_height - top_layer_depth_random - bottom_layer_depth_random {
                        *value = bottom_layer;
                    }
                    // stone layer
                    else if y_u32 < y_height - top_layer_depth_random - bottom_layer_depth_random {
                        *value = stone_layer;
                    }
                }
            }
        }
    });


    // generate ores
    match biome_ores_option {
        Some(biome_ores) => {

            for ore_id in 0..biome_ores.get_size() {

                let (block_id, depth, heat, frequency, scale) = biome_ores.get_ore(ore_id);

                let (min_depth, max_depth) = depth.get();

                let (heat_min, heat_max, _, _) = heat.get();

                block_data.par_iter_mut().enumerate().for_each(| (index, value) | {
                    
                    let (mut x,y,mut z) = index_to_pos(index);

                    x += pos_x as f64 * 16.0;
                    z += pos_z as f64 * 16.0; 

                    let y_u32: u32 = y as u32;

                    // set to 0 for debugging
                    if *value == stone_layer &&

                        y_u32 >= min_depth as u32 && y_u32 <= max_depth as u32 {

                            let ore_noise: f64 = gen_3d(&noise, x, y, z, frequency as f64, scale as f64);

                            if ore_noise >= heat_min as f64 && ore_noise <= heat_max as f64 {
                                *value = block_id;
                            }
                    }
                })
            }
        },
        None => (),
    };
}