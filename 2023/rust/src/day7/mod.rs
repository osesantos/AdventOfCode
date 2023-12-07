use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
struct Hand {
    label: String,
    bid: u32,
    rank: u32,
}

const CARDS: [&str;13] = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

fn is_five_of_kind(hand: String) -> bool {
    let re = Regex::new(r"^(A{5}|K{5}|Q{5}|J{5}|T{5}|9{5}|8{5}|7{5}|6{5}|5{5}|4{5}|3{5}|2{5})$").unwrap();
    re.is_match(&hand)
}

fn is_four_of_kind(hand: String) -> bool {
    let re = Regex::new(r"^(A{4}|K{4}|Q{4}|J{4}|T{4}|9{4}|8{4}|7{4}|6{4}|5{4}|4{4}|3{4}|2{4})$").unwrap();
    re.is_match(&hand)
}

fn is_three_of_kind(hand: String) -> bool {
    let re = Regex::new(r"^(A{3}|K{3}|Q{3}|J{3}|T{3}|9{3}|8{3}|7{3}|6{3}|5{3}|4{3}|3{3}|2{3})$").unwrap();
    re.is_match(&hand)
}

fn is_two_pair(hand: String) -> bool {
    let mut original_hand = hand.clone().chars(); 
    let mut pairs: Vec<(char, usize)> = Vec::new();

    for c in original_hand {
        let filtered_l: Vec<(char, usize)> = pairs.into_iter().filter(|x| x.0 == c).collect();
        if filtered_l.is_empty() {
            pairs.push((c, 1));
        } else {
            pairs.iter().map(|x| {
                if x.0 != c {
                    return x;
                } else {
                    return (x.0, x.1 + 1);
                }
            });
        }

    }

    // todo 
    false
}

fn is_one_pair(hand: String) -> bool {
    let mut original_hand = hand.clone().chars(); 
    let mut compare_hand: Vec<_> = vec![];

    for c in original_hand {
        // Only add cards that dont already exist in the compare_hand
        if !compare_hand.contains(&c) {
            compare_hand.push(c);
        } 
    }

    // one card missing, means that 2 are equal
    compare_hand.len() == hand.len() - 1
}

// All cards are distinct
fn is_high_card(hand: String) -> bool {
    let mut original_hand = hand.clone().chars(); 
    let mut compare_hand: Vec<_> = vec![];

    for c in original_hand {
        // Only add cards that dont already exist in the compare_hand
        if !compare_hand.contains(&c) {
            compare_hand.push(c);
        } 
    }

    // no card is missing on the list, they are all different
    compare_hand.len() == hand.len()
}

pub fn day7_1(_input: &Vec<String>) -> u32 {
    0
}



pub fn day7_2(_input: &Vec<String>) -> u32 {
    0
}
