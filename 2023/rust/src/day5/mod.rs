use std::ops::{Range, RangeInclusive};

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

impl Clone for Seed {
    fn clone(&self) -> Self {
        // Create a new instance of Card with the same values
        Seed {
            seed: self.seed.clone(),
            soil: self.soil.clone(),
            fertilizer: self.fertilizer.clone(),
            water: self.water.clone(),
            light: self.light.clone(),
            temperature: self.temperature.clone(),
            humidity: self.humidity.clone(),
            location: self.location.clone(),
        }
    }
}

struct Mapping {
    destination: usize,
    source: usize,
    len: usize,
}


pub fn day5_1(_input: &Vec<String>) -> u32 {

    let p = _input.iter().position(|l| l.contains("seeds:"));
    let p1 = _input.iter().position(|l| l.contains("seed-to-soil")).unwrap();
    let p2 = _input.iter().position(|l| l.contains("soil-to-fertilizer")).unwrap();
    let p3 = _input.iter().position(|l| l.contains("fertilizer-to-water")).unwrap();
    let p4 = _input.iter().position(|l| l.contains("water-to-light")).unwrap();
    let p5 = _input.iter().position(|l| l.contains("light-to-temperature")).unwrap();
    let p6 = _input.iter().position(|l| l.contains("temperature-to-humidity")).unwrap();
    let p7 = _input.iter().position(|l| l.contains("humidity-to-location")).unwrap();

    let seeds = get_seeds(&_input);
    let p1_map = get_mapping(&_input, p1, p2);
    let p2_map = get_mapping(&_input, p2, p3);
    let p3_map = get_mapping(&_input, p3, p4);
    let p4_map = get_mapping(&_input, p4, p5);
    let p5_map = get_mapping(&_input, p5, p6);
    let p6_map = get_mapping(&_input, p6, p7);
    let p7_map = get_mapping(&_input, p7, _input.len());

    let seeds_filled: Vec<Seed> = seeds.iter().map(|s| fill_seed(&s, &p1_map, &p2_map, &p3_map, &p4_map, &p5_map, &p6_map, &p7_map)).collect();

    //seeds_filled.into_iter().for_each(|s| println!("seed: {0}, soil: {1}, fert: {2}, wat: {3}, lig: {4}, tem: {5}, hum: {6}, location: {7}", s.seed, s.soil, s.fertilizer, s.water, s.light, s.temperature, s.humidity, s.location));

    let mut location = seeds_filled.last().unwrap().location;
    for seed in seeds_filled {
        if seed.location < location {
            location = seed.location;
        }
    }

    location as u32
}

fn get_position(_start: usize, _len: usize, _seed: usize) -> Option<usize> {
    if _seed >= _start && _seed <= (_start + _len) {
        return Some(_seed - _start);
    }
    return None;

}

fn inner_fill_seed(mappings: &Vec<Mapping>, input_value: usize) -> usize {
    let mut r = input_value;
    for m in mappings {
        let i = get_position(m.source, m.len, input_value);
        if i.is_some() {
            r = m.destination + i.unwrap();
            break;
        }
    }
    r
} 


fn fill_seed(_seed: &Seed, m1: &Vec<Mapping>, m2: &Vec<Mapping>, m3: &Vec<Mapping>, m4: &Vec<Mapping>, m5: &Vec<Mapping>, m6: &Vec<Mapping>, m7: &Vec<Mapping>) -> Seed {
    let mut seed = _seed.to_owned().clone();

    seed.soil = inner_fill_seed(m1, seed.seed);

    seed.fertilizer = inner_fill_seed(m2, seed.soil);

    seed.water = inner_fill_seed(m3, seed.fertilizer);

    seed.light = inner_fill_seed(m4, seed.water);

    seed.temperature = inner_fill_seed(m5, seed.light);

    seed.humidity = inner_fill_seed(m6, seed.temperature);

    seed.location = inner_fill_seed(m7, seed.humidity);

    seed
}

fn get_seeds(_input: &Vec<String>) -> Vec<Seed> {
    _input.iter().nth(0).unwrap().replace("seeds: ", "").split(" ").map(|s| Seed{
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

fn get_mapping(_input: &Vec<String>, map_i: usize, next_map_i: usize) -> Vec<Mapping> {
    //println!("--------");
    _input.iter().enumerate().map(|(i, l)| {
        if i < map_i + 1 ||  i > next_map_i - 1 {
            return String::new();
        } else {
            return l.to_string();
        }
    }).into_iter().filter(|l| !l.is_empty()).map(|l| {
            //println!("{}", l);
            let line_split: Vec<&str> = l.split(" ").collect();
            let leng = line_split[2].parse::<usize>().unwrap();
            let dest = line_split[0].parse::<usize>().unwrap();
            let sourc = line_split[1].parse::<usize>().unwrap();
            return Mapping {
                destination: dest,
                source: sourc,
                len: leng,
            };
        }).collect()
}

pub fn day5_2(_input: &Vec<String>) -> u32 {
    0
}
