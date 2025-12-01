struct Rotation {
    direction: char,
    degrees: i32,
}

impl Rotation {
    fn new(direction: char, degrees: i32) -> Self {
        Rotation { direction, degrees }
    }

    fn apply(&self, current: i32) -> i32 {
        let rotation = match self.direction {
            'L' => current - self.degrees,
            'R' => current + self.degrees,
            _ => current,
        };

        match self.direction {
            'L' | 'R' => (rotation + 100) % 100,
            _ => current,
        }
    }

    fn passes_through_zero(&self, current: i32) -> i32 {
        let mut count = 0;
        let steps = self.degrees / 1; // 1 degree per step
        for step in 1..=steps {
            let intermediate_rotation = match self.direction {
                'L' => (current - step + 100) % 100,
                'R' => (current + step) % 100,
                _ => current,
            };
            if intermediate_rotation == 0 {
                count += 1;
            }
        }
        count
    }
}

pub fn part1(input: Vec<String>) -> String {
    let rotations: Vec<Rotation> = input
        .iter()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let degrees: i32 = line[1..].parse().unwrap();
            Rotation::new(direction, degrees)
        })
        .collect();

    let mut current_rotation = 50;
    let mut zero_counter = 0;
    for rotation in &rotations {
        current_rotation = rotation.apply(current_rotation);
        if current_rotation == 0 {
            zero_counter += 1;
        }
    }

    zero_counter.to_string()
}

pub fn part2(input: Vec<String>) -> String {
    let rotations: Vec<Rotation> = input
        .iter()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let degrees: i32 = line[1..].parse().unwrap();
            Rotation::new(direction, degrees)
        })
        .collect();

    let mut current_rotation = 50;
    let mut zero_counter = 0;
    // Count every time it passes through zero including landing on zero
    for rotation in &rotations {
        zero_counter += rotation.passes_through_zero(current_rotation);
        current_rotation = rotation.apply(current_rotation);
    }

    zero_counter.to_string()
}

pub fn get_input_to_lines() -> Vec<String> {
    let input = include_str!("input.txt");
    input.lines().map(|line| line.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let expected_output = "3";
        let line_input: Vec<String> = input.trim().lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(line_input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let expected_output = "6";
        let line_input: Vec<String> = input.trim().lines().map(|line| line.to_string()).collect();
        assert_eq!(part2(line_input), expected_output);
    }
}
