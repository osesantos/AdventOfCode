pub fn day3() {
    let input = include_str!("input.txt");
    println!("day3.1 - {}", execute_part_1(input));
    println!("day3.2 - {}", execute_part_2(input));
}

fn execute_part_1(input: &str) -> i32 {
    0
}

fn execute_part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod day1 {
    use super::*;

    #[test]
    fn assert_1() {
        assert_eq!(execute_part_1("12"), 2);
        assert_eq!(execute_part_1("14"), 2);
        assert_eq!(execute_part_1("1969"), 654);
        assert_eq!(execute_part_1("100756"), 33583);
    }

    #[test]
    fn assert_2() {
        assert_eq!(execute_part_2("12"), 2);
        assert_eq!(execute_part_2("1969"), 966);
        assert_eq!(execute_part_2("100756"), 50346);
    }
}
