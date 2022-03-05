
use opensimplex_noise_rs::OpenSimplexNoise;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};

use crate::SEED;

use super::generation_component_system::GenerationComponentSystem;

// Convertes u16 1D position into (u8,u8,u8) 3D tuple position
fn index_to_pos ( i: usize ) -> (f64,f64,f64) {
    ((i / 2048) as f64, ((i % 2048) % 128) as f64, ((i % 2048) / 128) as f64)
}

fn calculate_y_height(noise_input: f64, base_height: f64, noise_multiplier: f64) -> u32 {
    (noise_input * noise_multiplier) as u32 + base_height as u32
}

fn calculate_depth(
    noise_input: f64,
    min: u8,
    max: u8
) -> u32 {
    ((noise_input.abs() *
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
    // simplex_noise: &mut FastNoise,
    // fractal_noise: &mut FastNoise
) {

    // this is debug
    let (
        _,
        biome_heat_params,
        top_layer,
        top_layer_depth,
        bottom_layer,
        bottom_layer_depth,
        stone_layer,
        bedrock_layer,
        biome_ores_option,
        terrain_noise_multiplier,
        terrain_frequency,
        caves,
        cave_heat,
        rain,
        snow
    ) = gcs.get(0);
    
    // let simplex_noise: &mut FastNoise = &mut *simplex_noise;

    // simplex_noise.set_frequency(terrain_frequency);

    // the base height - if noise is always 0 the blocks will always generate to 0
    let base_height = 90.0;

    // the amount of fluctuation the blocks can have from base height
    //let noise_multiplier = 50.0;

    
    /*
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

    fractal_noise.set_frequency(cave_frequency);
    fractal_noise.set_fractal_octaves(3);
    fractal_noise.set_fractal_type(FractalType::Billow);

    */

    let (cave_min_heat, cave_max_heat, cave_frequency) = cave_heat.get();

    let noise = OpenSimplexNoise::new(Some(SEED as i64));

    // generate unmodified terrain
    block_data.par_iter_mut().enumerate().for_each(| (index, value) | {

        // noise structure
        
        let (mut x, y, mut z) = index_to_pos(index);
        x += pos_x as f64 * 16.0;
        z += pos_z as f64 * 16.0;

        // println!("NOISE: {:?}", perlin.get([x,y,z]));

        

        let y_u32: u32 = y as u32;

        // println!("Noise: {}", &perlin.get([x * pos_x as f64, z * pos_z as f64]));

        // todo: replace scale 1.0 with terrain scale
        let terrain_2d_noise = gen_2d(&noise, x, z, terrain_frequency as f64, 1.0);
        let terrain_3d_noise = gen_3d(&noise, x, y, z, terrain_frequency as f64, 1.0);

        let cave_2d_noise = gen_2d(&noise, x, z, cave_frequency as f64, 1.0);
        let cave_3d_noise = gen_3d(&noise, x, y, z, cave_frequency as f64, 2.0);
                
        let y_height = calculate_y_height(terrain_2d_noise, base_height, terrain_noise_multiplier as f64);
        
        // println!("Y HEIGHT IS: {}, 2d noise is: {}", y_height, noise_2d);

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

                    if terrain_3d_noise > 0.0 {
                        bedrock = true;
                    }

                    //simplex_noise.set_frequency(terrain_frequency);
                }
            }

            if bedrock {

                *value = bedrock_layer;

            } else {

                if caves && (cave_3d_noise >= cave_min_heat as f64 && cave_3d_noise <= cave_max_heat as f64) {
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

    /*
    // generate ores
    match biome_ores_option {
        Some(biome_ores) => {

            for ore_id in 0..biome_ores.get_size() {

                let (block_id, depth, heat, frequency) = biome_ores.get_ore(ore_id);

                fractal_noise.set_frequency(*frequency);
                fractal_noise.set_fractal_octaves(2);
                fractal_noise.set_fractal_type(FractalType::Billow);

                let (min_depth, max_depth) = depth.get();

                let (heat_min, heat_max) = heat.get();

                for i in 0..32768 {
                    
                    let (x,y,z) = index_to_pos(i);

                    let y_u32: u32 = y as u32;

                    // set to 0 for debugging
                    if block_data[i] == stone_layer &&
                        y_u32 >= min_depth as u32 && y_u32 <= max_depth as u32 {
                            let noise_calculation: f32 = calculate_noise(x, y, z, pos_x as f64, pos_z as f64, &fractal_noise);

                            if noise_calculation >= heat_min && noise_calculation <= heat_max {
                                block_data[i] = *block_id;
                            }
                        }
                    }
                }
            }
        None => (),
    };
    */
}