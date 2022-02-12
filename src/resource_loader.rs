use std::fs;

pub fn load_resource(directory: String) -> String {
    let resource_result = fs::read_to_string(&directory);    

    match resource_result {
        Ok(data) => data,
        Err(_) => panic!("FAILED TO LOAD: {}!", directory),
    }    
}