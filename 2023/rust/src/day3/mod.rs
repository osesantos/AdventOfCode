use regex::Regex;

#[derive(Debug)]
struct Part {
    line_i: usize,
    i: usize,
    value: u32,
    n: String,
    top: String,
    bottom: String,
    is_valid: bool,
}

pub fn day3_1(_input: &Vec<String>) -> u32 {
    let mut parts: Vec<Part> = vec![];
    _input.into_iter().enumerate().for_each(|(i, _line)|{
        let parts_index_and_n = get_parts_index_and_n(_line.to_string());
        parts_index_and_n.into_iter().for_each(|part| {
            let mut _top = String::new();
            let mut _bottom = String::new();
            if i == 0 {
                _top = "".to_string();
            } else {
                _top = get_top_or_bottom(_input[i-1].to_string(), part.0, part.1.len());
            }
            if i == _input.len() - 1 {
                _bottom = "".to_string();
            } else {
                _bottom = get_top_or_bottom(_input[i+1].to_string(), part.0, part.1.len());
            }

            let _is_valid = is_part_valid(part.1.clone(), _top.clone(), _bottom.clone());

            let new_part = Part { 
                line_i: i, 
                i: part.0, 
                value: get_part_value(part.1.clone()), 
                n: part.1.clone(), 
                top: _top.clone(), 
                bottom: _bottom.clone(), 
                is_valid: _is_valid};
            parts.push(new_part);
        });
    });
    parts.into_iter().filter(|p| p.is_valid).map(|p| p.value).sum()
}

fn is_part_valid(n: String, top: String, bottom: String) -> bool {
    let re = Regex::new(r"[^0-9a-zA-Z.\n]").unwrap();
    if re.find(&n.as_str()).is_some() {
        return true;
    }
    if re.find(&top.as_str()).is_some() {
        return true;
    }
    if re.find(&bottom.as_str()).is_some() {
        return true;
    }

    return false;

} 

fn get_top_or_bottom(_line: String, i: usize, size: usize) -> String {
    if _line.is_empty() {
        return "".to_string();
    }
    _line.chars().skip(i).take(size).collect()
}

fn get_part_value(n: String) -> u32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    re.find(&n.as_str()).expect("Unable to find value from n").as_str().parse::<usize>().unwrap() as u32
}

fn get_parts_index_and_n(_line: String) -> Vec<(usize, String)> {
    let re = Regex::new(r".\d{1,3}.").unwrap();
    let mut result: Vec<(usize, String)> = vec![];
    re.captures_iter(&_line.as_str()).for_each(|g| {
        g.iter().for_each(|m| {
            let mut current_value = (m.unwrap().start(), m.unwrap().as_str().to_string());
            // this is for e.g like "..123.456.."
            if !result.is_empty() && current_value.1.chars().nth(0).unwrap().is_numeric() {
                current_value.0 = current_value.0 - 1;
                current_value.1 = result.last().unwrap().1.chars().last().unwrap().to_string() + current_value.1.as_str();
            }
            result.push(current_value)
        });
    });
    result
}

#[derive(Debug)]
struct Gear {
    astx_i: u32,
    values: Vec<u32>,
}

// Algorithm:
// 1. look for the * by using the find_iter r"\*"
// 2. get the index (i - 1 and i + 1)
// 3. look for numbers in current, top and bottom line r"[0-9]"
// 4. if any number starts or ends in index - 1 and index + 1 add it to gear
// 5. multiply top and down
// 6. sum
//
pub fn day3_2(_input: &Vec<String>) -> u32 {
    let mut gears: Vec<Gear> = vec![];
    _input.into_iter().enumerate().for_each(|(i, _line)|{
        let astx_vec = find_asterixes(_line.to_string());
        let mut top_n_and_i: Vec<((u32, u32), String)>  = vec![];
        let mut n_n_and_i: Vec<((u32, u32), String)>  = vec![];
        let mut bottom_n_and_i: Vec<((u32, u32), String)>  = vec![];


        if i != 0 {
            top_n_and_i = find_numbers_and_indexes(_input[i-1].to_string());
        } 

        if i != _input.len() - 1 {
            bottom_n_and_i = find_numbers_and_indexes(_input[i+1].to_string());
        }

        n_n_and_i = find_numbers_and_indexes(_line.to_string());

        let line_gears: Vec<_> = astx_vec.iter().map(|a| find_gear(*a, top_n_and_i.clone(), n_n_and_i.clone(), bottom_n_and_i.clone())).collect();

        for ele in line_gears {
            gears.push(ele);
        }
    });
    gears.iter().filter(|g| g.values.len() == 2).map(|g| {
        return g.values[0] * g.values[1];
    }).sum()
}

fn find_gear(astx_i: u32, top: Vec<((u32, u32), String)>, n: Vec<((u32, u32), String)>, bottom: Vec<((u32, u32), String)>) -> Gear {
    let mut gear = Gear{astx_i: astx_i, values: vec![]};

    let low_range = astx_i - 1;
    let high_range = astx_i + 2;
    let range = low_range..high_range;

    if !top.is_empty() {
        top.iter().for_each(|e|{
            if range.contains(&e.0.0) || range.contains(&e.0.1) {
                gear.values.push(e.1.parse::<u32>().unwrap())
            }
        });
    }

    if !bottom.is_empty() {
        bottom.iter().for_each(|e|{
            if range.contains(&e.0.0) || range.contains(&e.0.1) {
                gear.values.push(e.1.parse::<u32>().unwrap())
            }
        });
    }

    n.iter().for_each(|e|{
        if range.contains(&e.0.0) || range.contains(&e.0.1) {
            gear.values.push(e.1.parse::<u32>().unwrap())
        }
    });

    if gear.values.len() > 2 {
        println!("[ERROR] Found a gear with more that 2 values: ");
        for ele in gear.values {
            println!("{ele}");
        }
        panic!("Found a gear with more that 2 values");
    }

    gear
}

fn find_numbers_and_indexes(_line: String) -> Vec<((u32, u32), String)> {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    re.find_iter(&_line.as_str()).map(|m| ((m.start() as u32, (m.end()-1) as u32), m.as_str().to_string())).collect()
}

fn find_asterixes(_line: String) -> Vec<u32> {
    let re = Regex::new(r"\*").unwrap();
    re.find_iter(&_line.as_str()).map(|a| a.start() as u32).collect()
}



