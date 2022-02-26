use std::{
    fs::{
        self,
        File
    },
    io::{Read, BufReader}
};

use image::{ImageBuffer, Rgba};

use crate::helper::helper_functions::{get_path_string, with_path};


pub fn load_resource(path: String) -> String {    
    let resource_result = fs::read_to_string(get_path_string() + &path);    

    match resource_result {
        Ok(data) => data,
        Err(_) => panic!("FAILED TO LOAD: {}!", &path[..]),
    }    
}

// creates a usable image buffer in rgba 8 format
pub fn create_image_buffer(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let image: File = File::open(with_path(path)).expect(&("COULD NOT LOAD IMAGE IN ".to_string() + path));
    let buffered_reader: BufReader<File> = BufReader::new(image);
    image::load(buffered_reader, image::ImageFormat::Png).unwrap().to_rgba8()
}

pub fn load_texture(path: &str) -> Vec<u8>{

    let mut file: File = File::open(get_path_string() + path).expect("COULD NOT LOAD IMAGE!");
    let mut data: Vec<u8> = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("COULD NOT PARSE IMAGE DATA!");

    data
}