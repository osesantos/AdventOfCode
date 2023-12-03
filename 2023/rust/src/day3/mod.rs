// any number adjacent to a symbol, even diagonally, is a "part number"
// adjacent means that around the number at least one position has a symbol
// whats the sum of all part numbers?

// eg n: ".52.", "..*.", "...."
// eg n: ".532.", "...*.", "....."
#[derive(Debug)]
struct Part {
    n: u32,
    topn: String,
    bottomn: String
}

pub fn day3_1(input: &Vec<String>) -> u32 {
    let mut parts: Vec<Part> = vec![];
    input.iter().enumerate().for_each(|(i, l)| {
        println!("Mapping line {0}", l);
        let mut line_parts = map_line_to_parts(input.clone(), l.to_string(), i);
        parts.append(&mut line_parts);
    });
    parts.into_iter().map(|p| p.n).sum()
}

fn map_line_to_parts(input: Vec<String>, line: String, i: usize) -> Vec<Part> {
    let mut top_line = String::new();
    let mut bottom_line = String::new();

    if i != 0 {
        top_line = input.clone().into_iter().nth(i - 1).unwrap();
    }

    println!("{0}",input.len());
    if i != (input.len() - 1) {
        bottom_line = input.clone().into_iter().nth(i + 1).unwrap();
    }

    find_possible_parts(line.clone(), top_line, bottom_line)
}

// goes through the line to find any number
fn find_possible_parts(line: String, top_line: String, bottom_line: String) -> Vec<Part> {
    let mut parts_found: Vec<Part> = vec![];
    let mut part_found: bool = false;
    line.chars().into_iter().enumerate().for_each(|(i, c)| {
        if c == '.' && part_found {
            part_found = false;
        }
        if c.is_numeric() && !part_found {
            part_found = true;

            // part
            let part = retrieve_n(line.clone(), i);
            let mut top = String::new();
            let mut bottom = String::new();

            // top
            if !top_line.is_empty() {
                top = retrieve_n(top_line.clone(), i);
            }

            // bottom
            if !bottom_line.is_empty() {
                bottom = retrieve_n(bottom_line.clone(), i);
            }

            if is_part(part.clone(), top.clone(), bottom.clone()) {
                parts_found.push(Part { n: clean_n(part).parse::<u32>().unwrap(), topn: top.to_string(), bottomn: bottom.to_string() })
            }
        }
    });
    parts_found
}

fn clean_n(s: String) -> String {
    let mut new_s = String::new();
    s.chars().for_each(|c| {
        if c.is_numeric() {
            new_s.push(c);
        }
    });
    new_s
}

fn retrieve_n(line: String, i: usize) -> String {
    let mut n = String::new();
    n.push(line.clone().chars().nth(i).unwrap());
    n.push(line.clone().chars().nth(i + 1).unwrap());
    n.push(line.clone().chars().nth(i + 2).unwrap());
    // get char before n
    if i > 0 {
        n.push(line.clone().chars().nth(i - 1).unwrap());
    }
    // get char after n
    println!("{0}, {1}, {2}", i, (i + 3), line.len());
    if (i + 3) < (line.len() - 1) {
        n.push(line.clone().chars().nth(i + 3).unwrap());
    }
    n = n.replace(".", "");

    n
} 

// validates that the char is not numeric or .
fn is_symbol(c: char) -> bool {
    !(c.is_numeric() || c == '.')
}

// accepts the current number whith the left and right char, the top and the bottom
// e.g. n: ".35." topn: "..*." bottomn: "...."
// checks if any "." contains a symbol
fn is_part(n: String, topn: String, bottomn: String) -> bool {
    if n.chars().into_iter().any(|c| is_symbol(c)) {
        return true;
    }
    if topn.chars().into_iter().any(|c| is_symbol(c)) {
        return true;
    }
    if bottomn.chars().into_iter().any(|c| is_symbol(c)) {
        return true;
    }
    false
}

pub fn day3_2(_input: &Vec<String>) -> u32 {
    0
}
