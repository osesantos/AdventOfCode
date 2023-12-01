use crate::{day1::{day1_1, day1_2}, utils::{get_input_sample, get_input}};

mod utils;
mod day1;

fn main() {
    println!("--------------------------");
    println!("   Advent of Code 2023");
    println!("");
    println!("day1.1 sample - {0}", day1_1(&get_input_sample("day1_1")));
    println!("day1.1        - {0}", day1_1(&get_input("day1")));
    println!("day1.2 sample - {0}", day1_2(&get_input_sample("day1_2")));
    println!("day1.2        - {0}", day1_2(&get_input("day1")));
    println!("");
}
