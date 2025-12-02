fn main() {
    println!("Advent of Code!");
    // --- Day 1 ---
    println!(
        "Day 1 Part 1 Result: {}",
        day01::solution::part1(day01::solution::get_input_to_lines())
    );
    println!(
        "Day 1 Part 2 Result: {}",
        day01::solution::part2(day01::solution::get_input_to_lines())
    );
    // --- Day 2 ---
    println!(
        "Day 2 Part 1 Result: {}",
        day02::solution::part1(day02::solution::get_input())
    );
    println!(
        "Day 2 Part 2 Result: {}",
        day02::solution::part2(day02::solution::get_input())
    );
    // --- Day 3 ---
    println!(
        "Day 3 Part 1 Result: {}",
        day03::solution::part1(day03::solution::get_input_to_lines())
    );
    println!(
        "Day 3 Part 2 Result: {}",
        day03::solution::part2(day03::solution::get_input_to_lines())
    );
}

pub mod utils;
// Add day modules here
pub mod day01;
pub mod day02;
pub mod day03;
