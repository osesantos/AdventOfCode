#[derive(Debug)]
struct Seed {
    seed: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize
}

struct Mapping {
    destination_start: usize,
    source_start: usize,
    len: usize,
}


pub fn day5_1(_input: &Vec<String>) -> u32 {
    let seed_to_soil: Vec<Mapping> = vec![]; 
    let soil_to_fertilizer: Vec<Mapping> = vec![]; 
    let fertilizer_to_water: Vec<Mapping>  = vec![]; 
    let water_to_light: Vec<Mapping>  = vec![]; 
    let light_to_temperature: Vec<Mapping>  = vec![]; 
    let temperature_to_humidity: Vec<Mapping>  = vec![]; 
    let humidity_to_location: Vec<Mapping>  = vec![];





    0
}

fn get_seeds(input: &Vec<String>) -> Vec<Seed> {
    input.iter().nth(0).unwrap().replace("seeds: ", "").split(" ").map(|s| Seed{
        seed: s.parse::<usize>().unwrap(),
        soil: 0,
        fertilizer: 0,
        water: 0,
        light: 0,
        temperature: 0,
        humidity: 0,
        location: 0
    }).collect()
}

pub fn day5_2(_input: &Vec<String>) -> u32 {
    0
}
