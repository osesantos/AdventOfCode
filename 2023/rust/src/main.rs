use crate::{
    day1::{day1_1, day1_2}, 
    day2::{day2_1, day2_2}, 
    day3::{day3_1, day3_2}, 
    day4::{day4_1, day4_2}, 
    day5::{day5_1, day5_2_2}, 
    day6::{day6_1, day6_2}, 
    day7::{day7_1, day7_2}, 
    utils::{get_input_sample, get_input, get_input_str, get_input_sample_str}
};

use std::env;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();
    // 0 -> app
    // 1 -> day (e.g. "day1")

    println!("--------------------------");
    println!("   Advent of Code 2023");
    if args.len() == 1 {
        println!("");
        println!("Please add a day, eg. \"cargo run -- day1\"");
        return
    }
    let day = &args[1];
    if day == "day1" {
        println!("");
        println!("day1.1 sample - {0}", day1_1(&get_input_sample("day1_1")));
        println!("day1.1        - {0}", day1_1(&get_input("day1")));
        println!("day1.2 sample - {0}", day1_2(&get_input_sample("day1_2")));
        println!("day1.2        - {0}", day1_2(&get_input("day1")));
        println!("");
    }
    if day == "day2" {
        println!("");
        println!("day2.1 sample - {0}", day2_1(&get_input_sample("day2")));
        println!("day2.1        - {0}", day2_1(&get_input("day2")));
        println!("day2.2 sample - {0}", day2_2(&get_input_sample("day2")));
        println!("day2.2        - {0}", day2_2(&get_input("day2")));
        println!("");
    }
    if day == "day3" {
        println!("");
        println!("day3.1 sample - {0}", day3_1(&get_input_sample("day3")));
        println!("day3.1        - {0}", day3_1(&get_input("day3")));
        println!("day3.2 sample - {0}", day3_2(&get_input_sample("day3")));
        println!("day3.2        - {0}", day3_2(&get_input("day3")));
        println!("");
    }
    if day == "day4" {
        println!("");
        println!("day4.1 sample - {0}", day4_1(&get_input_sample("day4")));
        println!("day4.1        - {0}", day4_1(&get_input("day4")));
        println!("day4.2 sample - {0}", day4_2(&get_input_sample("day4")));
        println!("day4.2        - {0}", day4_2(&get_input("day4")));
        println!("");
    }
    if day == "day5" {
        println!("");
        println!("day5.1 sample - {0}", day5_1(&get_input_sample("day5")));
        println!("day5.1        - {0}", day5_1(&get_input("day5")));
        //println!("day5.2 sample - {0}", day5_2(&get_input_sample("day5")));
        //println!("day5.2        - {0}", day5_2(&get_input("day5")));
        println!("day5.2 sample - {0}", day5_2_2(&get_input_sample_str("day5")));
        println!("day5.2        - {0}", day5_2_2(&get_input_str("day5")));
        println!("");
    }
    if day == "day6" {
        println!("");
        println!("day6.1 sample - {0}", day6_1(&get_input_sample("day6")));
        println!("day6.1        - {0}", day6_1(&get_input("day6")));
        println!("day6.2 sample - {0}", day6_2(&get_input_sample("day6")));
        println!("day6.2        - {0}", day6_2(&get_input("day6")));
        println!("");
    }
    if day == "day7" {
        println!("");
        println!("day7.1 sample - {0}", day7_1(&get_input_sample("day7")));
        println!("day7.1        - {0}", day7_1(&get_input("day7")));
        println!("day7.2 sample - {0}", day7_2(&get_input_sample("day7")));
        println!("day7.2        - {0}", day7_2(&get_input("day7")));
        println!("");
    }
}
