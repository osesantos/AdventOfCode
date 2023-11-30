
use std::fs;

pub fn get_file_lines(_file_name: &str) -> Vec<String> {
    return fs::read_to_string("src/inputs/".to_string() + _file_name).expect("Could not read the path").lines().map(|s| s.to_string()).collect()
}

