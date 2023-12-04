
#[derive(Debug)]
struct Card {
    id: usize,
    winning_nums: Vec<usize>,
    nums: Vec<usize>,
    points: usize,
}

pub fn day4_1(_input: &Vec<String>) -> u32 {
    _input.iter().map(|line| parse_line(line).points as u32).sum()
}

fn parse_line(_line: &String) -> Card {
    let id_nums: Vec<String> = _line.split(":").map(|e| e.to_string()).collect();
    let new_card = Card {
        id: id_nums[0].to_string().replace("Card ", "").trim().parse::<usize>().unwrap(),
        winning_nums: id_nums[1].to_string().split("|").collect::<Vec<_>>()[0].trim().split(" ").filter(|s| !s.is_empty()).map(|e| e.trim().parse::<usize>().unwrap()).collect(),
        nums: id_nums[1].to_string().split("|").collect::<Vec<_>>()[1].trim().split(" ").filter(|s| !s.is_empty()).map(|e| e.trim().parse::<usize>().unwrap()).collect(),
        points: 0,
    };

    fill_points(new_card)

}

fn fill_points(_card: Card) -> Card {
    let mut points = 0;

    for n in _card.nums.clone() {
        if _card.winning_nums.contains(&n) {
            if points == 0 {
                points = 1;
            } else {
                points = points * 2;
            }
        }
    }

    let mut new_card = _card;
    new_card.points = points;

    new_card
}



pub fn day4_2(_input: &Vec<String>) -> u32 {
    0
}
