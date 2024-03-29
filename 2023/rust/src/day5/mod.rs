use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::process;

#[derive(Debug)]
struct Seed {
    seed: usize,
    len: usize,
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
            len: self.len.clone(),
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
    should_check: bool,
}

impl Clone for Mapping {
    fn clone(&self) -> Self {
        // Create a new instance of Card with the same values
       Mapping {
            destination: self.destination.clone(),
            source: self.source.clone(),
            len: self.len.clone(),
            should_check: self.should_check.clone(),
        }
    }
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
        if !m.should_check {
            break;
        }
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

    // Add a loop to get the location for all the range, and return the lowest one

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
        len: 0,
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
                should_check: false
            };
        }).collect()
}

/// reddit solution :: check edges
/// seeds: 79 14 55 13 -- seeds #79 and #55 should be checked
// seed-to-soil map:
// 50 98 2  -- seed #98 is outside "seeds" ranges
//-- seed #100 is outside "seeds"
// 52 50 48 -- seed #50 is outside "seeds" ranges
//-- seed #98 is outside "seeds"
// soil-to-fertilizer map:
// 0 15 37 -- soil #15 -> seed #15  -> not in the "seeds"
//-- soil #52 -> seed #50  -> not in the "seeds"
// 37 52 2 -- soil #52 -> seed #50  -> not in the "seeds"
//-- soil #54 -> seed #52  -> not in the "seeds"
// 39 0 15 -- soil #0  -> seed #0   -> not in the "seeds"
//-- soil #15  -> seed #15 -> not in the "seeds"
// fertilizer-to-water map:
// 49 53 8 -- fertilizer #53 -> soil #53 -> Seed #51 is in the "seeds" so should be checked
//-- fertilizer #61 -> soil #61 -> Seed #59 is in the "seeds", so should be checked
// ...etc
//
pub fn day5_2(_input: &Vec<String>) -> u32 {
    let p1 = _input.iter().position(|l| l.contains("seed-to-soil")).unwrap();
    let p2 = _input.iter().position(|l| l.contains("soil-to-fertilizer")).unwrap();
    let p3 = _input.iter().position(|l| l.contains("fertilizer-to-water")).unwrap();
    let p4 = _input.iter().position(|l| l.contains("water-to-light")).unwrap();
    let p5 = _input.iter().position(|l| l.contains("light-to-temperature")).unwrap();
    let p6 = _input.iter().position(|l| l.contains("temperature-to-humidity")).unwrap();
    let p7 = _input.iter().position(|l| l.contains("humidity-to-location")).unwrap();

    let seeds = get_seeds_2(&_input);
    let p1_map = get_mapping(&_input, p1, p2);
    let p2_map = get_mapping(&_input, p2, p3);
    let p3_map = get_mapping(&_input, p3, p4);
    let p4_map = get_mapping(&_input, p4, p5);
    let p5_map = get_mapping(&_input, p5, p6);
    let p6_map = get_mapping(&_input, p6, p7);
    let p7_map = get_mapping(&_input, p7, _input.len());

    let seeds_filled: Vec<Seed> = seeds.iter().map(|s| 
        fill_seed_2(
            &s, 
            &set_mapping_check(&p1_map, s.seed, s.seed + s.len), 
            &set_mapping_check(&p2_map, s.seed, s.seed + s.len), 
            &set_mapping_check(&p3_map, s.seed, s.seed + s.len), 
            &set_mapping_check(&p4_map, s.seed, s.seed + s.len), 
            &set_mapping_check(&p5_map, s.seed, s.seed + s.len), 
            &set_mapping_check(&p6_map, s.seed, s.seed + s.len), 
            &set_mapping_check(&p7_map, s.seed, s.seed + s.len), 
        )
    ).collect();

    seeds_filled.clone().into_iter().for_each(|s| println!("seed: {0}, len: {8}, soil: {1}, fert: {2}, wat: {3}, lig: {4}, tem: {5}, hum: {6}, location: {7}", s.seed, s.soil, s.fertilizer, s.water, s.light, s.temperature, s.humidity, s.location, s.len));

    let mut location = seeds_filled.last().unwrap().location;
    for seed in seeds_filled {
        if seed.location < location {
            location = seed.location;
        }
    }

    location as u32
}


fn get_seeds_2(_input: &Vec<String>) -> Vec<Seed> {
    let seeds_raw = _input.iter().nth(0).unwrap().replace("seeds: ", "");
    let seeds_splitted = seeds_raw.split(" ");

    // 0 => start
    // 1 => len
    let mut seeds_pairs: Vec<_> = vec![];
    seeds_splitted.clone().into_iter().enumerate().for_each(|(i, v)| {
        if i % 2 != 0 {
            let start = seeds_splitted.clone().nth(i-1).unwrap().parse::<usize>().unwrap();
            let len = v.parse::<usize>().unwrap();
            seeds_pairs.push((start, len));
        }
    });

    let mut seeds: Vec<Seed> = vec![];

    for pair in seeds_pairs {
        //let mut i = pair.0;
        //while i <= (pair.0 + pair.1.clone()) {
        //seeds.push(Seed{
        //seed: i,
        //soil: 0,
        //fertilizer: 0,
        //water: 0,
        //light: 0,
        //temperature: 0,
        //humidity: 0,
        //location: 0
        //});

        //i = i + 1;
        //}
        seeds.push(Seed { seed: pair.0, len: pair.1, soil: 0, fertilizer: 0, water: 0, light: 0, temperature: 0, humidity: 0, location: 0 })
    }

    seeds

}

fn fill_seed_2(_seed: &Seed, m1: &Vec<Mapping>, m2: &Vec<Mapping>, m3: &Vec<Mapping>, m4: &Vec<Mapping>, m5: &Vec<Mapping>, m6: &Vec<Mapping>, m7: &Vec<Mapping>) -> Seed {
    let mut seed = _seed.to_owned().clone();

    // Loop to get the location for all the range, and return the lowest one

    println!("  Getting ready to iterate, start: {0}, len: {1}", seed.seed, seed.len);

    seed.soil = inner_fill_seed(m1, seed.seed);

    seed.fertilizer = inner_fill_seed(m2, seed.soil);

    seed.water = inner_fill_seed(m3, seed.fertilizer);

    seed.light = inner_fill_seed(m4, seed.water);

    seed.temperature = inner_fill_seed(m5, seed.light);

    seed.humidity = inner_fill_seed(m6, seed.temperature);

    seed.location = inner_fill_seed(m7, seed.humidity);

    seed
}


/// reddit solution :: check edges
/// seeds: 79 14 55 13 -- seeds #79 and #55 should be checked
// seed-to-soil map:
// 50 98 2  -- seed #98 is outside "seeds" ranges
//          -- seed #100 is outside "seeds"
// 52 50 48 -- seed #50 is outside "seeds" ranges
//          -- seed #98 is outside "seeds"
// soil-to-fertilizer map:
// 0 15 37  -- soil #15 -> seed #15  -> not in the "seeds"
//          -- soil #52 -> seed #50  -> not in the "seeds"
// 37 52 2  -- soil #52 -> seed #50  -> not in the "seeds"
//          -- soil #54 -> seed #52  -> not in the "seeds"
// 39 0 15  -- soil #0  -> seed #0   -> not in the "seeds"
//          -- soil #15  -> seed #15 -> not in the "seeds"
// fertilizer-to-water map:
// 49 53 8  -- fertilizer #53 -> soil #53 -> Seed #51 is in the "seeds" so should be checked
//          -- fertilizer #61 -> soil #61 -> Seed #59 is in the "seeds", so should be checked
// ...etc
//
fn set_mapping_check(mappings: &Vec<Mapping>, seed_start: usize, seed_end: usize) -> Vec<Mapping>{
    let mut new_map: Vec<_> = vec![];
    for m in mappings {
        let mut new_m: Mapping = m.clone();
        println!("Mapping ->  source: {0}, source-end: {1}", new_m.source, new_m.source + new_m.len);
        println!("  Seed :: start: {0}, end: {1}", seed_start, seed_end);
        let seed_range = seed_start..seed_end;
        //if new_m.source <= seed_start && seed_end >= (new_m.source + new_m.len) {
        if seed_range.contains(&new_m.source) || seed_range.contains(&(new_m.source + new_m.len)) {
            println!("----- In range!");
            new_m.should_check = true;
        }
        new_map.push(new_m);
    }

    new_map
}

// --------------------------------------------------------------------------------------
//
//
//

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Interval {
    first: i64,
    last: i64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Mapping_2 {
    source: i64,
    destination: i64,
    length: i64,
}

struct Conversion {
    mappings: Vec<Mapping_2>,
}

fn usage() {
    let progname = env::args().next().unwrap();
    println!("usage: {progname} <file>");
    process::exit(1);
}

fn build_map(expect: &str, lineptr: &mut std::str::Lines) -> Conversion {
    let mut conversion_map: Conversion = Conversion { mappings: vec![] };
    match lineptr.next() {
        Some(line) => {
            if !line.starts_with(expect) {
                panic!("unexpected input");
            }
        }
        None => panic!("unexpected EOF"),
    }
    for line in lineptr.by_ref() {
        if line.is_empty() {
            break;
        } else {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            conversion_map.mappings.push(Mapping_2 {
                source: values[1],
                destination: values[0],
                length: values[2],
            });
        }
    }
    conversion_map
}

fn process_this(contents: &str) -> i64 {
    let mut lineptr = contents.lines();
    let mut lowest: i64 = i64::MAX;
    let seeds: Vec<i64> = match lineptr.next() {
        Some(line) => {
            if !line.starts_with("seeds:") {
                panic!("expected seeds");
            }
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        }
        None => panic!("unexpected EOF"),
    };
    lineptr.next().expect("unexpected EOF");

    let conversions: Vec<Conversion> = vec![
        build_map("seed-to-soil", &mut lineptr),
        build_map("soil-to-fertilizer", &mut lineptr),
        build_map("fertilizer-to-water", &mut lineptr),
        build_map("water-to-light", &mut lineptr),
        build_map("light-to-temperature", &mut lineptr),
        build_map("temperature-to-humidity", &mut lineptr),
        build_map("humidity-to-location", &mut lineptr),
    ];

    for i in (0..seeds.len()).step_by(2) {
        let first: i64 = seeds[i];
        let length: i64 = seeds[i + 1];
        let mut ranges: VecDeque<Interval> = VecDeque::from([Interval {
            first,
            last: first + length - 1,
        }]);
        for conversion in conversions.iter() {
            let mut new_ranges: Vec<Interval> = vec![];
            while !ranges.is_empty() {
                let range: Interval = ranges.pop_front().unwrap();
                let mut found: bool = false;
                for mapping in conversion.mappings.iter() {
                    if range.first >= mapping.source && range.last < mapping.source + mapping.length
                    {
                        new_ranges.push(Interval {
                            first: range.first - mapping.source + mapping.destination,
                            last: range.last - mapping.source + mapping.destination,
                        });
                        found = true;
                    } else if range.first < mapping.source
                        && range.last >= mapping.source
                        && range.last < mapping.source + mapping.length
                    {
                        ranges.push_back(Interval {
                            first: range.first,
                            last: mapping.source - 1,
                        });
                        new_ranges.push(Interval {
                            first: mapping.destination,
                            last: mapping.destination + range.last - mapping.source,
                        });
                        found = true;
                    } else if range.first < mapping.source + mapping.length
                        && range.last >= mapping.source + mapping.length
                        && range.first >= mapping.source
                    {
                        ranges.push_back(Interval {
                            first: mapping.source + mapping.length,
                            last: range.last,
                        });
                        new_ranges.push(Interval {
                            first: mapping.destination + range.first - mapping.source,
                            last: mapping.destination + mapping.length - 1,
                        });
                        found = true;
                    } else if range.first < mapping.source
                        && range.last >= mapping.source + mapping.length
                    {
                        ranges.push_back(Interval {
                            first: range.first,
                            last: mapping.source - 1,
                        });
                        new_ranges.push(Interval {
                            first: mapping.destination,
                            last: mapping.destination + mapping.length - 1,
                        });
                        ranges.push_back(Interval {
                            first: mapping.source + mapping.length,
                            last: range.last,
                        });
                        found = true;
                    }
                    if found {
                        break;
                    }
                }
                if !found {
                    new_ranges.push(range);
                }
            }
            ranges = VecDeque::from(new_ranges);
        }
        lowest = cmp::min(lowest, ranges.iter().min_by_key(|r| r.first).unwrap().first);
    }
    lowest
}

pub fn day5_2_2(_input: &str) -> u32 {
    let result = process_this(&_input);
    result as u32
}


