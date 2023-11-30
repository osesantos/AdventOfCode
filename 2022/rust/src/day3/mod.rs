use crate::utils::get_file_lines;

pub fn day3_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let compartments: Vec<(&str, &str)> = lines.iter().map(|l| l.split_at(l.len() / 2)).collect();
    let dups: Vec<char> = compartments.iter().map(|c| find_dup(&c.0.chars().collect(), &c.1.chars().collect())).collect();
    total_score(&dups.iter().map(|c| calculate_priority(&c)).collect()).to_string()
}

fn total_score(scores: &Vec<i32>) -> i32 {
    scores.iter().sum()
}

fn find_dup(compartment_1: &Vec<char>, compartment_2: &Vec<char>) -> char {
    let mut dup: char = ' ';
    compartment_1.iter().for_each(|c| {
        if compartment_2.iter().find(|&&x| x == *c).is_some() {
            dup = *c;
        }
    });
    dup
}

/// Reduction factor to ASCII
/// uppercase -> 'A' is 65 while the score is 27 -> 38
/// lowercase -> 'a' is 97 while the score is 1 -> 96
///
fn calculate_priority(dup: &char) -> i32 {
    let dup_ascii = *dup as i32;
    if dup.is_ascii_uppercase() {
        dup_ascii - 38
    } else {
        dup_ascii - 96
    }
}

pub fn day3_2(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let teams = lines.chunks(3);
    let badges: Vec<char> = teams.map(|chunk| find_badge(&chunk.to_vec())).collect();

    total_score(&badges.iter().map(|c| calculate_priority(c)).collect()).to_string()
}

fn find_badge(group_sacks: &Vec<String>) -> char {
    let mut badge: char = ' ';
    let sac1 = &group_sacks[0];
    let sac2 = &group_sacks[1];
    let sac3 = &group_sacks[2];

    sac1.chars().into_iter().for_each(|c| {
       if (sac2.chars().into_iter().find(|&x| x == c).is_some()) && (sac3.chars().into_iter().find(|&x| x == c).is_some()) {
           badge = c;
       }
    });
    badge
}
