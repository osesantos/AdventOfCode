use regex::Regex;

#[derive(Debug)]
struct Hand {
    label: String,
    bid: u32,
    rank: u32,
}

impl Clone for Hand {
    fn clone(&self) -> Self {
        // Create a new instance of Card with the same values
        Hand {
            label: self.label.clone(),
            bid: self.bid.clone(),
            rank: self.rank.clone(),
        }
    }
}

const CARDS: [&char;13] = [&'A', &'K', &'Q', &'J', &'T', &'9', &'8', &'7', &'6', &'5', &'4', &'3', &'2'];

fn is_five_of_kind(hand: String) -> bool {
    let re = Regex::new(r"^(A{5}|K{5}|Q{5}|J{5}|T{5}|9{5}|8{5}|7{5}|6{5}|5{5}|4{5}|3{5}|2{5})$").unwrap();
    re.is_match(&hand)
}

// need to support QQQJQ and T5555
fn is_four_of_kind(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut found = false;

    for c in CARDS {
        let filtered_len: Vec<char> = original_hand.clone().filter(|oc| oc == c).collect();
        if filtered_len.len() == 4 {
            found = true;
            break;
        }
    }

    found
}

// need to support QQQJA and T55J5 and dont consider KK677
fn is_three_of_kind(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut found = false;

    for c in CARDS {
        let filtered_len: Vec<char> = original_hand.clone().filter(|oc| oc == c).collect();
        if filtered_len.len() == 3 {
            found = true;
            break;
        }
    }

    found
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
    let mut found = false;

    for c in CARDS {
        let filtered_len: Vec<char> = original_hand.clone().filter(|oc| oc == c).collect();
        if filtered_len.len() == 2 {
            found = true;
            break;
        }
    }

    found
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

// 0 -> lowest rank
// 1 -> high rank
fn order_by_rank(a: Hand, b:Hand) -> std::cmp::Ordering {
    let mut result: std::cmp::Ordering = std::cmp::Ordering::Equal;
    if a.label.len() != b.label.len() {
        panic!("Can not order, len are different.");
    }
    let range = 0..a.label.len();
    for i in range {
        if CARDS.iter().position(|&&c| c == a.label.chars().nth(i).unwrap()) == CARDS.iter().position(|&&c| c == b.label.chars().nth(i).unwrap()) {
            continue;
        }
        if CARDS.iter().position(|&&c| c == a.label.chars().nth(i).unwrap()) > CARDS.iter().position(|&&c| c == b.label.chars().nth(i).unwrap()) {
            result = std::cmp::Ordering::Greater;
            break;
        }
        else {
            result = std::cmp::Ordering::Less;
            break;
        }
    }

    result
}

// five = hands.len -> high = 1
// but if high is empty then one is ranked 1, if five is empty then four is ranked 5
fn rank_hands(hands: &Vec<Hand>) -> Vec<Hand> {
    let mut hands_with_rank: Vec<Hand> = vec![];

    // highest rank value
    let mut five: Vec<Hand> = Vec::new();
    let mut four: Vec<Hand> = Vec::new();
    let mut full: Vec<Hand> = Vec::new();
    let mut three: Vec<Hand> = Vec::new();
    let mut two: Vec<Hand> = Vec::new();
    let mut one: Vec<Hand> = Vec::new();
    // lowest rank value
    let mut high: Vec<Hand> = Vec::new();

    for hand in hands {
        let hand_label = &hand.label;
        if is_five_of_kind(hand_label.to_string()) {
            //println!("Ranking {} as five", hand_label);
            five.push(hand.clone());
        }
        if is_four_of_kind(hand_label.to_string()) {
            //println!("Ranking {} as four", hand_label);
            four.push(hand.clone());
        }
        if is_full_house(hand_label.to_string()) {
            //println!("Ranking {} as full", hand_label);
            full.push(hand.clone());
        }
        if is_three_of_kind(hand_label.to_string()) && !is_full_house(hand_label.to_string()) {
            //println!("Ranking {} as three", hand_label);
            three.push(hand.clone());
        }
        if is_two_pair(hand_label.to_string()) && !is_full_house(hand_label.to_string()) {
            //println!("Ranking {} as two", hand_label);
            two.push(hand.clone());
        }
        if is_one_pair(hand_label.to_string()) && !is_two_pair(hand_label.to_string()) {
            //println!("Ranking {} as one", hand_label);
            one.push(hand.clone());
        }
        if is_high_card(hand_label.to_string()) {
            //println!("Ranking {} as high", hand_label);
            high.push(hand.clone());
        }
    }
    
    five.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));
    four.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));
    full.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));
    three.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));
    two.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));
    one.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));
    high.sort_by(|a, b| order_by_rank(a.clone(), b.clone()));

    let mut order_q: Vec<Hand> = Vec::new();
    order_q.append(&mut five);
    order_q.append(&mut four);
    order_q.append(&mut full);
    order_q.append(&mut three);
    order_q.append(&mut two);
    order_q.append(&mut one);
    order_q.append(&mut high);


    let mut point_value = order_q.len();
    for e in order_q.clone() {
        hands_with_rank.push(Hand{
            label: e.label.clone(),
            bid: e.bid.clone(),
            rank: point_value as u32
        });
        point_value = point_value - 1;
        //println!("{}", e.label);
    }


    hands_with_rank
}

// J is the Joker, and J is the lowest card
pub fn day7_1(input: &Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = input.into_iter().map(|l| parse_line(&l)).collect();
    hands = rank_hands(&hands);

    hands.into_iter().map(|h| h.bid * h.rank).sum()
}

// --------------------------------------------------------------------------------------------
// Part 2

// J is the Joker, and J is the lowest card
pub fn day7_2(input: &Vec<String>) -> u32 {
    let hands = rank_hands_2(&input);

    hands.into_iter().map(|h| h.bid * h.rank).sum()
}

const CARDS_2: [&char;13] = [&'A', &'K', &'Q', &'T', &'9', &'8', &'7', &'6', &'5', &'4', &'3', &'2', &'J'];

fn is_five_of_kind_2(hand: String) -> bool {
    let re = Regex::new(r"^(A{5}|K{5}|Q{5}|J{5}|T{5}|9{5}|8{5}|7{5}|6{5}|5{5}|4{5}|3{5}|2{5})$").unwrap();
    re.is_match(&hand)
}

// need to support QQQJQ and T5555
fn is_four_of_kind_2(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut found = false;

    for c in CARDS_2 {
        let filtered_len: Vec<char> = original_hand.clone().filter(|oc| oc == c || oc == &'J').collect();
        if filtered_len.len() == 4 {
            found = true;
            break;
        }
    }

    found
}

// need to support QQQJA and T55J5 and dont consider KK677
fn is_three_of_kind_2(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut found = false;

    for c in CARDS_2 {
        let filtered_len: Vec<char> = original_hand.clone().filter(|oc| oc == c || oc == &'J').collect();
        if filtered_len.len() == 3 {
            found = true;
            break;
        }
    }

    found
}

fn is_full_house_2(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut pairs: Vec<(char, usize)> = Vec::new();

    for c in original_hand {
        let filtered_l: Vec<(char, usize)> = pairs.clone().into_iter().filter(|x| x.0 == c || x.0 == 'J').collect();
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

fn is_two_pair_2(hand: String) -> bool {
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

fn is_one_pair_2(hand: String) -> bool {
    let original_hand = hand.chars(); 
    let mut found = false;

    for c in CARDS_2 {
        let filtered_len: Vec<char> = original_hand.clone().filter(|oc| oc == c || oc == &'J').collect();
        if filtered_len.len() == 2 {
            found = true;
            break;
        }
    }

    found
}

// All cards are distinct
fn is_high_card_2(hand: String) -> bool {
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


// 0 -> lowest rank
// 1 -> high rank
fn order_by_rank_2(a: Hand, b:Hand) -> std::cmp::Ordering {
    let mut result: std::cmp::Ordering = std::cmp::Ordering::Equal;
    if a.label.len() != b.label.len() {
        panic!("Can't order, lens are different.");
    }
    println!("-----------------------------------");

    let range = 0..a.label.len();
    //let a_sorted = order_hand(a.clone());
    //let b_sorted = order_hand(b.clone());
    println!("a: {0}, b: {1}", a.label, b.label);

    for i in range {
        let a_char = a.label.chars().nth(i).unwrap();
        let b_char = b.label.chars().nth(i).unwrap();

        println!("Comparing a {0} with b {1}", a_char, b_char);


        if CARDS_2.iter().position(|&&c| c == a_char) == CARDS_2.iter().position(|&&c| c == b_char) {
            continue;
        }

        if CARDS_2.iter().position(|&&c| c == a_char) > CARDS_2.iter().position(|&&c| c == b_char) {
            result = std::cmp::Ordering::Greater;
            println!("a is Greater");
            break;
        }
        else {
            println!("a is Less");
            result = std::cmp::Ordering::Less;
            break;
        }
    }

    result
}

fn order_hand(h: Hand) -> Hand {
    let mut result: Hand = h.clone();

    let mut chars: Vec<char> = result.label.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    result.label = String::from_iter(chars);

    result
}

// five = hands.len -> high = 1
// but if high is empty then one is ranked 1, if five is empty then four is ranked 5
fn rank_hands_2(input: &Vec<String>) -> Vec<Hand> {
    let hands: Vec<Hand> = input.into_iter().map(|l| parse_line(&l)).collect();
    let mut hands_with_rank: Vec<Hand> = vec![];

    // highest rank value
    let mut five: Vec<Hand> = Vec::new();
    let mut four: Vec<Hand> = Vec::new();
    let mut full: Vec<Hand> = Vec::new();
    let mut three: Vec<Hand> = Vec::new();
    let mut two: Vec<Hand> = Vec::new();
    let mut one: Vec<Hand> = Vec::new();
    // lowest rank value
    let mut high: Vec<Hand> = Vec::new();

    for hand in hands {
        let hand_label = &hand.label;
        if is_five_of_kind_2(hand_label.to_string()) {
            println!("Ranking {} as five", hand_label);
            five.push(hand.clone());
        }
        if is_four_of_kind_2(hand_label.to_string()) 
        && !is_five_of_kind_2(hand_label.to_string()) {
            println!("Ranking {} as four", hand_label);
            four.push(hand.clone());
        }
        if is_full_house_2(hand_label.to_string()) {
            println!("Ranking {} as full", hand_label);
            full.push(hand.clone());
        }
        if is_three_of_kind_2(hand_label.to_string()) 
        && !is_full_house_2(hand_label.to_string()) 
        && !is_four_of_kind_2(hand_label.to_string()) {
            println!("Ranking {} as three", hand_label);
            three.push(hand.clone());
        }
        if is_two_pair_2(hand_label.to_string()) 
        && !is_full_house_2(hand_label.to_string()) 
        && !is_four_of_kind_2(hand_label.to_string()) 
        && !is_three_of_kind_2(hand_label.to_string()){
            println!("Ranking {} as two", hand_label);
            two.push(hand.clone());
        }
        if is_one_pair_2(hand_label.to_string()) 
        && !is_two_pair_2(hand_label.to_string())
        && !is_full_house_2(hand_label.to_string()) 
        && !is_four_of_kind_2(hand_label.to_string()) 
        && !is_three_of_kind_2(hand_label.to_string()){
            println!("Ranking {} as one", hand_label);
            one.push(hand.clone());
        }
        if is_high_card_2(hand_label.to_string()) {
            println!("Ranking {} as high", hand_label);
            high.push(hand.clone());
        }
    }
    
    five.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));
    four.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));
    full.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));
    three.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));
    two.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));
    one.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));
    high.sort_by(|a, b| order_by_rank_2(a.clone(), b.clone()));

    let mut order_q: Vec<Hand> = Vec::new();
    order_q.append(&mut five);
    order_q.append(&mut four);
    order_q.append(&mut full);
    order_q.append(&mut three);
    order_q.append(&mut two);
    order_q.append(&mut one);
    order_q.append(&mut high);


    let mut point_value = order_q.len();
    for e in order_q.clone() {
        hands_with_rank.push(Hand{
            label: e.label.clone(),
            bid: e.bid.clone(),
            rank: point_value as u32
        });
        println!("hand: {0}, rank: {1}", e.label, point_value);
        point_value = point_value - 1;
    }


    hands_with_rank
}

