use std::{
    fs::{
        self,
        File
    },
    io::Read
};

pub fn get_path_string() -> String {
    std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn with_path(path: &str) -> String {
    get_path_string() + path
}

pub fn load_resource(path: String) -> String {    
    let resource_result = fs::read_to_string(get_path_string() + &path);    

    match resource_result {
        Ok(data) => data,
        Err(_) => panic!("FAILED TO LOAD: {}!", &path[..]),
    }    
}

pub fn load_texture(path: &str) -> Vec<u8>{

    let mut file: File = File::open(get_path_string() + path).expect("COULD NOT LOAD IMAGE!");
    let mut data: Vec<u8> = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("COULD NOT PARSE IMAGE DATA!");

    data
}