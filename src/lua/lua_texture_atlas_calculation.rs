use texture_packer::{Frame, Rect};

use crate::blocks::block_component_system::AtlasTextureMap;

pub fn calculate_atlas_location_normal(
    atlas_width: u32,
    atlas_height: u32,
    frame: &Frame<String>
) -> AtlasTextureMap {
    
    // println!("{:#?}", frame);

    let rectangle_frame: Rect = frame.frame;

    // convert all to f32 so the calculations are readable
    let atlas_width_f32: f32 = atlas_width as f32;
    let atlas_height_f32: f32 = atlas_height as f32;

    // base location (top left) x
    let x_f32: f32 = rectangle_frame.x as f32;
    // base location (top left) y
    let y_f32: f32 = rectangle_frame.y as f32;
    // width of texture
    let w_f32: f32 = rectangle_frame.w as f32;
    // height of texture
    let h_f32: f32 = rectangle_frame.h as f32;


    let min_x: f32 = x_f32 / atlas_width_f32;
    let min_y: f32 = y_f32 / atlas_height_f32;

    let max_x: f32 = (x_f32 + w_f32) / atlas_width_f32;
    let max_y: f32 = (y_f32 + h_f32) / atlas_height_f32;

    AtlasTextureMap::new(
        min_x, 
        min_y, 
        max_x, 
        max_y
    )

    // println!("{:#?}", test);

    // test
}