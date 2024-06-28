mod day2 {
    pub fn day2() {
        let input = include_str!("input.txt");
        println!("day2.1 - {}", execute_part_1(input));
        println!("day2.2 - {}", execute_part_2(input));
    }

    fn execute_part_1(input: &str) -> i32 {
        0
    }

    fn execute_part_2(input: &str) -> i32 {
        0
    }

    fn add(input: &str, pos1: i32, pos2: i32, pos3: i32) -> String {
        "".to_string()
    }

    fn multiply(input: &str, pos1: i32, pos2: i32, pos3: i32) -> String {
        "".to_string()
    }

    #[cfg(test)]
    mod day2 {
        use super::*;

        #[test]
        fn assert_1() {
            assert_eq!(execute_part_1("1,0,0,0,99"), "2,0,0,0,99");
            assert_eq!(execute_part_1("2,3,0,3,99"), "2,3,0,6,99");
            assert_eq!(execute_part_1("2,4,4,5,99,0"), "2,4,4,5,99,9801");
            assert_eq!(execute_part_1("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
        }

        #[test]
        fn assert_2() {}
    }
}
