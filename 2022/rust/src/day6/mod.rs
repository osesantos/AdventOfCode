use std::collections::HashSet;
use std::str::Chars;
use crate::utils::get_file_lines;

// The start of a packet is a sequence of "four characters that are all different".
// The returning String is a nth of the last char of a sequence
pub fn day6_1(file_name: &str) -> String {
    let binding = get_file_lines(file_name);
    let line = binding.first().unwrap();
    let chars = line.chars();
    let mut index: usize = 0;
    chars.clone().enumerate().for_each(|(i, c)| {
        if i > 2 && index == 0 {
            let first = get_chars_nth(i - 3, chars.clone());
            let second = get_chars_nth(i - 2, chars.clone());
            let third = get_chars_nth(i - 1, chars.clone());
            let forth = get_chars_nth(i, chars.clone());
            if are_they_different(first, second, third, forth) {
                index = i + 1;
            }
        }
    });

    index.to_string()
}

fn get_chars_nth(n: usize, chars: Chars) -> char {
    chars.clone().nth(n).unwrap()
}

fn get_chars_nth_back(n: usize, chars: Chars) -> char {
    chars.clone().nth_back(n).unwrap()
}

fn are_they_different(first: char, second: char, third: char, forth: char) -> bool {
    let chars: Vec<char> = vec![first, second, third, forth];
    let unique_chars: HashSet<_> = chars.iter().cloned().collect();
    unique_chars.len() == chars.len()
}

fn are_chars_different(chars: Vec<char>) -> bool {
    let unique_chars: HashSet<_> = chars.iter().cloned().collect();
    unique_chars.len() == chars.len()
}

pub fn day6_2(file_name: &str) -> String {
    let binding = get_file_lines(file_name);
    let line = binding.first().unwrap();
    let chars = line.chars();
    let mut index: usize = 0;
    chars.clone().enumerate().for_each(|(i, c)| {
        if i > 12 && index == 0 {
            let char_vec: Vec<char> = chars.clone().collect();
            let slice = char_vec[i-13..i].to_vec();
            if are_chars_different(slice) {
                index = i + 1
            }
        }
    });

    index.to_string()
}
