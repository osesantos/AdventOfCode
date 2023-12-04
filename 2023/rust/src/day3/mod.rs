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
    a_value: u32,
    b_value: u32,
}

pub fn day3_2(_input: &Vec<String>) -> u32 {
    let mut gears: Vec<Gear> = vec![];
    _input.into_iter().enumerate().for_each(|(i, _line)|{
        let ast = find_asterixes(_line.to_string());

    });
    0
}

fn find_asterixes(_line: String) -> Vec<usize> {
    let re = Regex::new(r"\*").unwrap();
    re.find_iter(&_line.as_str()).map(|a| a.start()).collect()
}



