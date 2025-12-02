struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    // invalid examples: 55, 6464, 123123
    // invalid if some sequence of digits repeated twice
    // however, 121212 is valid because 12 is repeated more than twice
    fn is_num_invalid(&self, num: u64) -> bool {
        let num_len = num.to_string().len();
        if num_len % 2 != 0 {
            return false;
        }

        let left_half = num
            .to_string()
            .chars()
            .take(num_len / 2)
            .collect::<String>();
        let right_half = num
            .to_string()
            .chars()
            .skip(num_len / 2)
            .take(num_len / 2)
            .collect::<String>();

        if left_half.len() == right_half.len() && left_half == right_half {
            return true;
        }
        false
    }

    // invalid examples: 55, 6464, 123123, 111, 999, 1010
    // invalid if some sequence of digits repeated twice
    // 121212 is invalid because 12 is repeated more than twice
    // 824824824 is invalid because 824 is repeated more than twice
    // 56565656 is invalid because 56 is repeated more than twice
    fn is_num_invalid_2(&self, num: u64) -> bool {
        let num_str = num.to_string();
        let num_len = num_str.len();

        for seq_len in 1..=(num_len / 2) {
            if num_len % seq_len != 0 {
                continue;
            }

            let mut is_invalid = true;
            let first_seq = &num_str[0..seq_len];

            for i in (0..num_len).step_by(seq_len) {
                let current_seq = &num_str[i..i + seq_len];
                if current_seq != first_seq {
                    is_invalid = false;
                    break;
                }
            }

            if is_invalid {
                return true;
            }
        }

        false
    }

    fn sum_invalid(&self) -> u64 {
        let mut sum = 0;
        for num in self.start..=self.end {
            if self.is_num_invalid(num) {
                sum += num;
            }
        }
        sum
    }

    fn sum_invalid_2(&self) -> u64 {
        let mut sum = 0;
        for num in self.start..=self.end {
            if self.is_num_invalid_2(num) {
                sum += num;
            }
        }
        sum
    }
}

pub fn part1(input: &str) -> String {
    let mut total_sum = 0;

    input.split(',').map(|s| s.trim()).for_each(|range_str| {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        let range = Range::new(start, end);
        total_sum += range.sum_invalid();
    });

    total_sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total_sum = 0;

    input.split(',').map(|s| s.trim()).for_each(|range_str| {
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        let range = Range::new(start, end);
        total_sum += range.sum_invalid_2();
    });

    total_sum.to_string()
}

pub fn get_input() -> &'static str {
    let input = include_str!("input.txt");
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        let expected_output = "1227775554";
        assert_eq!(part1(input), expected_output);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        let expected_output = "4174379265";
        assert_eq!(part2(input), expected_output);
    }
}
