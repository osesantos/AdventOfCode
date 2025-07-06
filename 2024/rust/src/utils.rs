pub fn get_input_lines(day: &str) -> Vec<String> {
    let input = std::fs::read_to_string(format!("input/day{}.txt", day))
        .expect("Failed to read input file");
    input.lines().map(|s| s.to_string()).collect()
}
