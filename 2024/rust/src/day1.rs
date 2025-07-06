use std::usize;

use crate::utils::get_input_lines;

pub fn part1(lines: Vec<String>) -> usize {
    let (list1, list2) = parse_lines_to_list(lines);
    let mut distances: Vec<usize> = Vec::new();

    let mut list1_sorted = list1.clone();
    let mut list2_sorted = list2.clone();
    list1_sorted.sort();
    list2_sorted.sort();

    let len = list1_sorted.len();
    for i in 0..len {
        distances.push(subtract(list1_sorted[i], list2_sorted[i]).unwrap());
    }

    distances.iter().sum()
}

pub fn part2() {}

fn parse_lines_to_list(lines: Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();
    lines.iter().for_each(|l| {
        let parts: Vec<&str> = l.split(' ').into_iter().filter(|x| !x.is_empty()).collect();
        list1.push(parts.get(0).unwrap().parse::<usize>().unwrap());
        list2.push(parts.get(1).unwrap().parse::<usize>().unwrap());
    });

    (list1, list2)
}

fn subtract(first: usize, second: usize) -> Result<usize, String> {
    if first <= second {
        Ok(second - first)
    }
    else {
        Err(format!("unable to subtract: first: {}, second: {}", first, second))
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let parsed_lines = lines.lines().map(|l| l.to_string()).collect();

        assert_eq!(part1(parsed_lines), 11);
    }

    #[test]
    fn test_part1_input() {
        let result = part1(get_input_lines("1"));
        println!("{}", result);
        assert_eq!(1, 1)
    }

    #[test]
    fn test_part2() {
        // Add your test logic for part2 here
        assert_eq!(2 * 2, 4); // Example assertion
    }
}
