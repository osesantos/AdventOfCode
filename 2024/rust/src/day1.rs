use std::usize;

use crate::utils::get_input_lines;

pub fn part1(lines: Vec<String>) -> usize {
    let (mut list1, mut list2) = parse_lines_to_list(lines);
    list1.sort();
    list2.sort();

    list1.iter().enumerate().map(|(i, x)| x.abs_diff(list2[i])).sum()
}

pub fn part2(lines: Vec<String>) -> usize {
    let (list1, list2) = parse_lines_to_list(lines);

    list1.iter().map(|i| list2.iter().filter(|x| **x == *i).count() * i).sum()
}

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
        println!("result: {}", part1(get_input_lines("1")));
        assert_eq!(1, 1)
    }

    #[test]
    fn test_part2() {
        let lines: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let parsed_lines = lines.lines().map(|l| l.to_string()).collect();

        assert_eq!(part2(parsed_lines), 31);
    }

    #[test]
    fn test_part2_input() {
        println!("result: {}", part2(get_input_lines("1")));
        assert_eq!(1, 1)
    }
}
