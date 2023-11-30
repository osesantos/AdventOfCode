use crate::utils::get_file_lines;

/// A - Rock, B - Paper, C - Scissors
/// X - Rock, Y - Paper, Z - Scissors
///
/// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
///
/// The score for a single round is the score for the shape you selected,
/// (1 for Rock, 2 for Paper, and 3 for Scissors)
/// plus the score for the outcome of the round.
/// (0 if you lost, 3 if the round was a draw, and 6 if you won)
///
/// Total score is the sum of your scores for each round.
///
pub fn day2_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let scores: Vec<Vec<&str>> = lines.iter().map(|l| l.split(" ").collect()).collect();

    let t_score: Vec<i32> = scores.into_iter().map(|v| round_score(&v[0].to_string(), &v[1].to_string())).collect();

    total_score(&t_score).to_string()
}

fn total_score(scores: &Vec<i32>) -> i32 {
    scores.iter().sum()
}

fn round_score(opponent_choice: &String, my_choice: &String) -> i32 {
    parse_my_choice(my_choice) + parse_match_result(opponent_choice, my_choice)
}

fn parse_my_choice(choice: &String) -> i32 {
    match choice.as_str() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    }
}

fn parse_match_result(o_c: &String, m_c: &String) -> i32 {
    if (o_c == "A" && m_c == "Z") || (o_c == "C" && m_c == "Y") || (o_c == "B" && m_c == "X") {
        0
    } else if (o_c == "A" && m_c == "X") || (o_c == "B" && m_c == "Y") || (o_c == "C" && m_c == "Z") {
        3
    } else {
        6
    }
}

/// X means you need to lose,
/// Y means you need to end the round in a draw, and
/// Z means you need to win.
///
pub fn day2_2(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let scores: Vec<Vec<&str>> = lines.iter().map(|l| l.split(" ").collect()).collect();

    let t_score: Vec<i32> = scores
        .into_iter()
        .map(|v|
            round_score(&v[0].to_string(), &obtain_my_choice(&v[0].to_string(), &v[1].to_string())))
        .collect();

    total_score(&t_score).to_string()
}

fn obtain_my_choice(o_c: &String, outcome: &String) -> String {
    if o_c == "A" {
        match outcome.as_str() {
            "X" => "Z".to_string(),
            "Y" => "X".to_string(),
            _ => "Y".to_string()
        }
    } else if o_c == "B" {
        match outcome.as_str() {
            "X" => "X".to_string(),
            "Y" => "Y".to_string(),
            _ => "Z".to_string()
        }
    } else {
        match outcome.as_str() {
            "X" => "Y".to_string(),
            "Y" => "Z".to_string(),
            _ => "X".to_string()
        }
    }
}
