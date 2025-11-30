pub fn part1(input: &str) -> String {
    // Implement the logic for part 1 here
    input.to_string()
}

pub fn part2(input: &str) -> String {
    // Implement the logic for part 2 here
    input.to_string()
}

fn get_input_to_lines() -> Vec<String> {
    let input = include_str!("input.txt");
    input.lines().map(|line| line.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "test input for part 1";
        let expected_output = "expected output for part 1";
        assert_eq!(part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "test input for part 2";
        let expected_output = "expected output for part 2";
        assert_eq!(part2(input), expected_output);
    }
}
