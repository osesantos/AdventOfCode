pub fn day1_1(_input: &Vec<String>) -> u32 {
    _input.into_iter().map(|line| (_first_digit(line) + _last_digit(line).as_str()).parse::<u32>().unwrap()).sum()
}

fn _first_digit(_line: &String) -> String {
    let first_number = _line.chars().into_iter().filter(|c| c.is_numeric()).nth(0);
    first_number.unwrap().to_string()
}

fn _last_digit(_line: &String) -> String {
    let first_number = _line.chars().into_iter().filter(|c| c.is_numeric()).last();
    first_number.unwrap().to_string()
}

pub fn day1_2(_input: &Vec<String>) -> u32 {
    _input.into_iter().map(|line| {
        let parsed_line = parse_line_2(line);
        let s_concat = match (_first_digit(&parsed_line) + _last_digit(&parsed_line).as_str()).parse::<u32>() {
            Ok(parsed_value) => parsed_value,
            Err(_) => {
                // Handle the parsing error, e.g., return a default value or log an error.
                // For now, returning 0 as a default value.
                0
            }
        };
        s_concat
    }).sum()
}

fn parse_line_2(line: &String) -> String {
    let mut new_line = line.clone();

    // replacing one with one1one because of "xtwone3four" it then becames "xtwo2twone1one3four4four"
    // This way it wont break the "twone"
    new_line = new_line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    new_line
}

