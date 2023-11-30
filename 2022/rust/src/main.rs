use traits::PrintableResult;
use crate::day1::{day1_1, day1_2};
use crate::day2::{day2_1, day2_2};
use crate::day3::{day3_1, day3_2};
use crate::day4::{day4_1, day4_2};
use crate::day5::{day5_1, day5_2};
use crate::day6::{day6_1, day6_2};
use crate::day7::{day7_1, day7_2};

mod utils;
mod day1;
mod traits;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!();
    println!("###############################################################");
    println!(" Advent of Code 2022");

    println!();

    day1_1("sample_day1.txt").printable_result("sample day 1.1:");
    day1_2("sample_day1.txt").printable_result("sample day 1.2:");
    day1_1("input_day1.txt").printable_result("day 1.1:");
    day1_2("input_day1.txt").printable_result("day 1.2:");

    println!();

    day2_1("sample_day2.txt").printable_result("sample day 2.1:");
    day2_2("sample_day2.txt").printable_result("sample day 2.2:");
    day2_1("input_day2.txt").printable_result("day 2.1:");
    day2_2("input_day2.txt").printable_result("day 2.2:");

    println!();

    day3_1("sample_day3.txt").printable_result("sample day 3.1:");
    day3_2("sample_day3.txt").printable_result("sample day 3.2:");
    day3_1("input_day3.txt").printable_result("day 3.1:");
    day3_2("input_day3.txt").printable_result("day 3.2:");

    println!();

    day4_1("sample_day4.txt").printable_result("sample day 4.1:");
    day4_2("sample_day4.txt").printable_result("sample day 4.2:");
    day4_1("input_day4.txt").printable_result("day 4.1:");
    day4_2("input_day4.txt").printable_result("day 4.2:");

    println!();

    day5_1("sample_day5.txt").printable_result("sample day 5.1:");
    day5_2("sample_day5.txt").printable_result("sample day 5.2:");
    day5_1("input_day5.txt").printable_result("day 5.1:");
    day5_2("input_day5.txt").printable_result("day 5.2:");

    println!();

    day6_1("sample_day6.txt").printable_result("sample day 6.1:");
    day6_2("sample_day6.txt").printable_result("sample day 6.2:");
    day6_1("input_day6.txt").printable_result("day 6.1:");
    day6_2("input_day6.txt").printable_result("day 6.2:");

    println!();

    day7_1("sample_day7.txt").printable_result("sample day 7.1:");
    day7_2("sample_day7.txt").printable_result("sample day 7.2:");
    day7_1("input_day7.txt").printable_result("day 7.1:");
    day7_2("input_day7.txt").printable_result("day 7.2:");
}
