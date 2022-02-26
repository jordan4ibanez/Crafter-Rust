use std::{fs::File, str::from_utf8, io::Read};

use crate::helper::helper_functions::with_path;

// loads a lua file within the root directory of the program
pub fn load_lua_file(path: &str) -> String {
    let mut file: File = File::open(with_path(path)).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let text_str: &str = from_utf8(&buffer).unwrap();

    let test_string = text_str.clone().to_string().to_owned();

    test_string
}