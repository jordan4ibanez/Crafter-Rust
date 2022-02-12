use std::fs;

pub fn load_resource(path: String) -> String {

    let resource_result = fs::read_to_string(&path);    

    match resource_result {
        Ok(data) => data,
        Err(_) => panic!("FAILED TO LOAD: {}!", &path[..]),
    }    
}