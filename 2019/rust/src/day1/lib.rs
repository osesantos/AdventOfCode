mod day1 {
    pub fn day1() {
        let input = include_str!("input.txt");
        println!("day1.1 - {}", execute_part_1(input));
        println!("day1.2 - {}", execute_part_2(input));
    }

    fn execute_part_1(input: &str) -> i32 {
        let input = input
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut sum = 0;

        for i in input.iter() {
            sum += formula(i);
        }

        sum
    }

    fn execute_part_2(input: &str) -> i32 {
        let input = input
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut sum = 0;

        for i in input.iter() {
            sum += recursive_formula(i);
        }

        sum
    }

    fn formula(mass: &i32) -> i32 {
        mass / 3 - 2
    }

    fn recursive_formula(mass: &i32) -> i32 {
        let fuel = formula(mass);

        if fuel <= 0 {
            return 0;
        }

        fuel + recursive_formula(&fuel)
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
}
