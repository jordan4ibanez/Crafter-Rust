use glam::Vec2;


pub struct Chunk {
    position:  Vec2,
    block:     [u32; 32768],
    rotation:  [u8;  32768],
    light:     [u8;  32768],
    heightmap: [u8;  256]
}