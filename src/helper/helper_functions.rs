// returns the string value of the os path
pub fn get_path_string() -> String {
    std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned()
}
 
// concatinates the os path string the input string slice
pub fn with_path(path: &str) -> String {
    get_path_string() + path
}