use std::usize;

use crate::utils::get_input_lines;

pub fn part1(lines: Vec<String>) -> usize {
    let rules = parse_lines_to_list(lines);

    rules.iter().map(|rule| {
        let mut rule_sorted = rule.clone();
        rule_sorted.sort();

        let mut diffs: Vec<usize> = vec![];

        for (i, x) in rule_sorted.iter().enumerate(){
            if i == 0 {
                continue;
            }

            diffs.push(x.abs_diff(rule_sorted[i - 1]));
        }

        diffs.sort();

        if diffs[3] > 3 {
            1
        }
        else {
            0
        }
    }).sum()
}

pub fn part2(lines: Vec<String>) -> usize {
    0
}

fn parse_lines_to_list(lines: Vec<String>) -> Vec<Vec<usize>> {
    lines.iter().map(|l| l.split(' ').into_iter().map(|x| x.parse::<usize>().unwrap()).collect()).collect()
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let parsed_lines = lines.lines().map(|l| l.to_string()).collect();

        assert_eq!(part1(parsed_lines), 2);
    }

    #[test]
    fn test_part1_input() {
        println!("result: {}", part1(get_input_lines("2")));
        assert_eq!(1, 1)
    }

    #[test]
    fn test_part2() {
        let lines: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let parsed_lines = lines.lines().map(|l| l.to_string()).collect();

        assert_eq!(part2(parsed_lines), 4);
    }

    #[test]
    fn test_part2_input() {
        println!("result: {}", part2(get_input_lines("2")));
        assert_eq!(1, 1)
    }
}
