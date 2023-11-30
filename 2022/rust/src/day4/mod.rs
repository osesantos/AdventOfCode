use crate::utils::get_file_lines;

pub fn day4_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let ranges  = get_ranges(lines);
    let contains_count: Vec<_> = ranges.into_iter().map(|r| fully_contains(r.first().unwrap(), r.last().unwrap())).collect();
    contains_count.iter().filter(|v| **v == true).count().to_string()
}

fn get_ranges(lines: Vec<String>) -> Vec<Vec<(i32, i32)>> {
    return lines
        .iter()
        .map(| l | l
            .split(",")
            .collect::<Vec<_>>()
            .into_iter()
            .map(| r | {
                let split_string = r.split("-").collect::<Vec<_>>();
                return (split_string.first().unwrap().parse::<i32>().unwrap(), split_string.last().unwrap().parse::<i32>().unwrap());
            })
            .collect::<Vec<_>>())
        .collect()
}

fn fully_contains(range1: &(i32, i32), range2: &(i32, i32)) -> bool {
    if range1.0 <= range2.0 && range1.1 >= range2.1 {
        return true;
    }
    if range2.0 <= range1.0 && range2.1 >= range1.1 {
        return true;
    }
    false
}

pub fn day4_2(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let ranges  = get_ranges(lines);
    let contains_count: Vec<_> = ranges.into_iter().map(|r| partial_contains(r.first().unwrap(), r.last().unwrap())).collect();
    contains_count.iter().filter(|v| **v == true).count().to_string()
}

fn partial_contains(range1: &(i32, i32), range2: &(i32, i32)) -> bool {
    let r1: Vec<i32> = (range1.0..=range1.1).collect();
    let r2: Vec<i32> = (range2.0..=range2.1).collect();
    let mut result: bool = false;

    r1.iter().for_each(| a | {
        if r2.contains(a) {
            result = true;
        }
    });

    return result;
}
