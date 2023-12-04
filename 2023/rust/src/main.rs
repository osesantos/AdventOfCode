use crate::{day1::{day1_1, day1_2}, day2::{day2_1, day2_2}, day3::{day3_1, day3_2}, day4::{day4_1, day4_2}, utils::{get_input_sample, get_input}};

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("--------------------------");
    println!("   Advent of Code 2023");
    println!("");
    println!("day1.1 sample - {0}", day1_1(&get_input_sample("day1_1")));
    println!("day1.1        - {0}", day1_1(&get_input("day1")));
    println!("day1.2 sample - {0}", day1_2(&get_input_sample("day1_2")));
    println!("day1.2        - {0}", day1_2(&get_input("day1")));
    println!("");
    println!("day2.1 sample - {0}", day2_1(&get_input_sample("day2")));
    println!("day2.1        - {0}", day2_1(&get_input("day2")));
    println!("day2.2 sample - {0}", day2_2(&get_input_sample("day2")));
    println!("day2.2        - {0}", day2_2(&get_input("day2")));
    println!("");
    println!("day3.1 sample - {0}", day3_1(&get_input_sample("day3")));
    println!("day3.1        - {0}", day3_1(&get_input("day3")));
    println!("day3.2 sample - {0}", day3_2(&get_input_sample("day3")));
    println!("day3.2        - {0}", day3_2(&get_input("day3")));
    println!("");
    println!("day4.1 sample - {0}", day4_1(&get_input_sample("day4")));
    println!("day4.1        - {0}", day4_1(&get_input("day4")));
    println!("day4.2 sample - {0}", day4_2(&get_input_sample("day4")));
    println!("day4.2        - {0}", day4_2(&get_input("day4")));
    println!("");
}
