// only 12 red cubes, 13 green cubes, and 14 blue cubes
// sum of the IDs of the games that are possible with thi config ^

use std::usize;

#[derive(Debug)]
struct Set {
    n_red: usize,
    n_green: usize,
    n_blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>
}

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

pub fn day2_1(_input: &Vec<String>) -> u32 {
    let games = parse_game(_input);
    games.into_iter().filter(|g| is_game_valid(g)).map(|g| g.id as u32).sum()
}

fn is_game_valid(game: &Game) -> bool {
    let mut is_valid = true;
    let sets = &game.sets;
    sets.into_iter().for_each(|s| {
        if s.n_red > RED || s.n_green > GREEN || s.n_blue > BLUE {
            is_valid = false;
        }
    });
    is_valid
}

fn parse_game(input: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![]; 
    input.into_iter().for_each(|line| {
        let game_id = line.split(":").nth(0).unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let sets = line.split(":").last().unwrap();
        let parsed_sets = parse_set(sets.to_string());
        games.push( Game{
            id: game_id,
            sets: parsed_sets
        });
    });
    games

}

// input e.g. " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn parse_set(input: String) -> Vec<Set> {
    let sets = input.split(";");
    let mut vec_sets: Vec<Set> = vec![];
    sets.for_each(|set| {
        let cubes = set.split(",");
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;
        cubes.for_each(|cube| {
            let elem = cube.trim().split(" ");
            if elem.clone().last().unwrap().contains("red") {
                reds = elem.clone().nth(0).unwrap().parse::<usize>().unwrap();
            }
            if elem.clone().last().unwrap().contains("green") {
                greens = elem.clone().nth(0).unwrap().parse::<usize>().unwrap();
            }
            if elem.clone().last().unwrap().contains("blue") {
                blues = elem.clone().nth(0).unwrap().parse::<usize>().unwrap();
            }
        });
        vec_sets.push(Set { n_red: reds, n_green: greens, n_blue: blues })
    });
    vec_sets
}

pub fn day2_2(_input: &Vec<String>) -> u32 {
    0
}
