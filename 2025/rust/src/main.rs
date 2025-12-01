fn main() {
    println!("Advent of Code!");
    println!(
        "Day 1 Part 1 Result: {}",
        day01::solution::part1(day01::solution::get_input_to_lines())
    );
    println!(
        "Day 1 Part 2 Result: {}",
        day01::solution::part2(day01::solution::get_input_to_lines())
    );
}

pub mod utils;
// Add day modules here
pub mod day01;
