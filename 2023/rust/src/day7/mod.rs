use std::collections::VecDeque;

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

// need to support QQQJQ and T5555
fn is_four_of_kind(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut compare_hand: Vec<_> = vec![];

    for c in original_hand {
        // Only add cards that dont already exist in the compare_hand
        if !compare_hand.contains(&c) {
            compare_hand.push(c);
        } 
    }

    // 3 cards missing, means that 4 are equal
    compare_hand.len() == hand.len() - 3
}

// need to support QQQJA and T55J5 and dont consider KK677
fn is_three_of_kind(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut compare_hand: Vec<_> = vec![];

    for c in original_hand {
        // Only add cards that dont already exist in the compare_hand
        if !compare_hand.contains(&c) {
            compare_hand.push(c);
        } 
    }

    // 2 cards missing, means that 3 are equal
    compare_hand.len() == hand.len() - 2
}

fn is_full_house(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut pairs: Vec<(char, usize)> = Vec::new();

    for c in original_hand {
        let filtered_l: Vec<(char, usize)> = pairs.clone().into_iter().filter(|x| x.0 == c).collect();
        if filtered_l.is_empty() {
            pairs.push((c, 1));
        } else {
            pairs = pairs.iter().map(|x| {
                if x.0 != c {
                    return (x.0, x.1);
                } else {
                    let new_count = x.1 + 1;
                    return (x.0, new_count);
                }
            }).collect();
        }

    }
    let pairs_final: Vec<_> = pairs.clone().into_iter().filter(|p| p.1 == 2).collect();
    let three_final: Vec<_> = pairs.clone().into_iter().filter(|p| p.1 == 3).collect();

    !pairs_final.is_empty() && !three_final.is_empty()
}

fn is_two_pair(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut pairs: Vec<(char, usize)> = Vec::new();

    for c in original_hand {
        let filtered_l: Vec<(char, usize)> = pairs.clone().into_iter().filter(|x| x.0 == c).collect();
        if filtered_l.is_empty() {
            pairs.push((c, 1));
        } else {
            pairs = pairs.iter().map(|x| {
                if x.0 != c {
                    return (x.0, x.1);
                } else {
                    let new_count = x.1 + 1;
                    return (x.0, new_count);
                }
            }).collect();
        }

    }
    pairs = pairs.into_iter().filter(|p| p.1 >= 2).collect();

    pairs.len() == 2
}

fn is_one_pair(hand: String) -> bool {
    let original_hand = hand.chars(); 
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
    let original_hand = hand.chars(); 
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

fn parse_line(line: &String) -> Hand {
    let raw_split = line.trim().split(" ");
    Hand { label: raw_split.clone().nth(0).unwrap().parse().unwrap(), bid: raw_split.clone().nth(1).unwrap().parse().unwrap(), rank: 0 }
}

// five = hands.len -> high = 1
// but if high is empty then one is ranked 1, if five is empty then four is ranked 5
fn rank_hands(hands: &Vec<Hand>) -> Vec<Hand> {
    let mut hands_with_rank: Vec<Hand> = vec![];

    // highest rank value
    let mut five: VecDeque<&Hand> = VecDeque::new();
    let mut four: VecDeque<&Hand> = VecDeque::new();
    let mut full: VecDeque<&Hand> = VecDeque::new();
    let mut three: VecDeque<&Hand> = VecDeque::new();
    let mut two: VecDeque<&Hand> = VecDeque::new();
    let mut one: VecDeque<&Hand> = VecDeque::new();
    // lowest rank value
    let mut high: VecDeque<&Hand> = VecDeque::new();

    for hand in hands {
        let hand_label = &hand.label;
        if is_five_of_kind(hand_label.to_string()) {
            println!("Ranking {} as five", hand_label);
            five.push_front(hand);
        }
        if is_four_of_kind(hand_label.to_string()) {
            println!("Ranking {} as four", hand_label);
            four.push_front(hand);
        }
        if is_full_house(hand_label.to_string()) {
            println!("Ranking {} as full", hand_label);
            full.push_front(hand);
        }
        if is_three_of_kind(hand_label.to_string()) {
            println!("Ranking {} as three", hand_label);
            three.push_front(hand);
        }
        if is_two_pair(hand_label.to_string()) {
            println!("Ranking {} as two", hand_label);
            two.push_front(hand);
        }
        if is_one_pair(hand_label.to_string()) {
            println!("Ranking {} as one", hand_label);
            one.push_front(hand);
        }
        if is_high_card(hand_label.to_string()) {
            println!("Ranking {} as high", hand_label);
            high.push_front(hand);
        }
    }

    

    hands_with_rank
}


pub fn day7_1(input: &Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = input.into_iter().map(|l| parse_line(&l)).collect();

    hands = rank_hands(&hands);

    0
}



pub fn day7_2(input: &Vec<String>) -> u32 {
    0
}
