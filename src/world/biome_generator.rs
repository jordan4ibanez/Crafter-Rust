
use perlin2d::PerlinNoise2D;

use super::chunk::Chunk;

// Convertes u16 1D position into (u8,u8,u8) 3D tuple position
pub fn index_to_pos ( i: &u16 ) -> (f64,f64,f64) {
    let mut index :u16 = i.clone();
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
    perlin: &mut PerlinNoise2D, 
    base_height: f64,
    noise_multiplier: f64,
) -> f64 {

    base_height + (perlin.get_noise(pos_x + (chunk_pos_x * 16.0), pos_z + (chunk_pos_z * 16.0) as f64) * noise_multiplier)

}

pub fn gen_biome(chunk: &mut Chunk, perlin: &mut PerlinNoise2D) {
    // directly working with chunk data
    let chunk_pos = chunk.get_pos();
    let blocks: &mut [u32; 32768] = chunk.get_block_array_mut();

    // the base height - if noise is always 0 the blocks will always generate to 0
    let base_height = 70.0;

    // the amount of fluctuation the blocks can have from base height
    let noise_multiplier = 50.0;

    let mut y_height = calculate_y_height(0.0, 0.0, chunk_pos.x as f64, chunk_pos.y as f64, perlin, base_height, noise_multiplier);

    for i in 0..32768 {
        let (x,y,z) = index_to_pos(&i);

        if y as i8 == 0 {
            y_height = calculate_y_height(x, z, chunk_pos.x as f64, chunk_pos.y as f64, perlin, base_height, noise_multiplier);
        }
        
        if y <= y_height {
            blocks[i as usize] = 1;
        }
    }
}