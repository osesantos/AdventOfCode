use std::fs;

pub fn get_input(_day: &str) -> Vec<String> {
    let _input_file = "src/inputs/".to_string() + _day + ".txt";
    return fs::read_to_string(_input_file).expect("Could not read the path").lines().map(|s| s.to_string()).collect()
}

pub fn get_input_sample(_day: &str) -> Vec<String> {
    let _input_file = "src/inputs/".to_string() + _day + "_sample.txt";
    return fs::read_to_string(_input_file).expect("Could not read the path").lines().map(|s| s.to_string()).collect()
}

