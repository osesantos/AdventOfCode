
#[derive(Debug)]
struct Card {
    id: usize,
    winning_nums: Vec<usize>,
    nums: Vec<usize>,
    points: usize,
    matches: usize,
    cards_own: Vec<Card>,
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
        matches: 0,
        cards_own: vec![]
    };

    fill_points_and_matches(new_card)
}

fn fill_points_and_matches(_card: Card) -> Card {
    let mut points = 0;
    let mut mathes = 0;

    for n in _card.nums.clone() {
        if _card.winning_nums.contains(&n) {
            mathes = mathes + 1;
            if points == 0 {
                points = 1;
            } else {
                points = points * 2;
            }
        }
    }

    let mut new_card = _card;
    new_card.points = points;
    new_card.matches = mathes;

    new_card
}

struct Instance {
    id: usize,
    count: usize,
}

impl Clone for Card {
    fn clone(&self) -> Self {
        // Create a new instance of Card with the same values
        Card {
            id: self.id.clone(),
            winning_nums: self.winning_nums.clone(),
            nums: self.nums.clone(),
            points: self.points.clone(),
            matches: self.matches.clone(),
            cards_own: self.cards_own.clone()
        }
    }
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        Instance { id: self.id.clone(), count: self.count.clone() }
    }
}

fn parse_instance(_line: &String) -> Instance {
    let id_nums: Vec<String> = _line.split(":").map(|e| e.to_string()).collect();
    Instance { 
        id: id_nums[0].to_string().replace("Card ", "").trim().parse::<usize>().unwrap(), 
        count: 1
    }
}

fn recursive_add_cards_own(_card: &Card, _cards: &Vec<Card>) -> Card {
    let mut new_card = _card.clone();
    println!("-----------------");
    println!("Starting to add cards for card {0}", new_card.id);
    println!("matches {0}", _card.matches);

    if _card.matches == 0 {
        println!("Exiting for card {0}", new_card.id);
        return new_card;
    }

    let filtered_cards: Vec<&Card> = _cards
        .iter()
        .filter(|c| c.id > new_card.id && c.id < (new_card.id + new_card.matches + 1))
        .collect();

    // having a stack overflow
    for c in &filtered_cards {
        println!("{}", c.id);
        new_card.cards_own.push(recursive_add_cards_own(c, _cards))
    }

    new_card
}

fn recursive_fill_instance(instances: &Vec<Instance>, card: &Card) -> Vec<Instance> {
    let mut new_instances = instances.clone();

    if card.cards_own.is_empty() {
        return new_instances;
    }

    for c in card.clone().cards_own {
        new_instances = new_instances.iter().filter(|i| i.id == c.id).map(|i| Instance{ id: i.id, count: i.count + 1}).collect();
        new_instances = recursive_fill_instance(&new_instances, card);
    }

    new_instances
}

pub fn day4_2(_input: &Vec<String>) -> u32 {
    let cards: Vec<Card> = _input.iter().map(|line| parse_line(line)).collect();
    let mut instances: Vec<Instance>= _input.iter().map(|line| parse_instance(line)).collect();

    let cards_with_own = cards.iter().map(|c| recursive_add_cards_own(&c, &cards));
    for c in cards_with_own.clone() {
        instances = recursive_fill_instance(&instances, &c);
    }

    instances.iter().map(|i| i.count as u32).sum()
}
