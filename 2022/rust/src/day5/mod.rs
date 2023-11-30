use crate::utils::get_file_lines;

struct Movement {
    quantity: i32,
    start_stack: i32,
    end_stack: i32,
}

pub fn day5_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let movements: Vec<_> = lines.iter().map(|l| l.movement_parser().unwrap_or(Movement {
        quantity: 0,
        start_stack: 0,
        end_stack: 0,
    })).into_iter().filter(|m| m.quantity != 0).collect();

    let mut stacks = convert_to_list(&lines);
    let executed_stack = execute(movements, &mut stacks);

    get_top_stacks(executed_stack)
}

fn get_top_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut s = String::new();

    stacks.iter().for_each(|r| {
        let new_s = r.first().unwrap().clone().to_string();
        s = format!("{}{}", s, new_s);
    });

    s
}

fn execute(movements: Vec<Movement>, stacks: &mut Vec<Vec<char>>) -> &Vec<Vec<char>> {
    movements.iter().for_each(|m| {
        let mut counter = 0;
        while counter < m.quantity {
            counter = counter + 1;
            let e = stacks[m.start_stack as usize - 1].remove(0).clone();
            stacks[m.end_stack as usize - 1].insert(0, e);
        }
    });

    return stacks;
}

fn convert_to_list(lines: &Vec<String>) -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = string_to_matrix(lines);
    transpose(matrix)
}

fn string_to_matrix(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();

    for line in lines {
        if line == "" {
            return matrix;
        }
        let newline = line.replace("["," ").replace("]", " ");
        let row: Vec<char> = newline.chars().collect();
        matrix.push(row);
    }

    matrix
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed_matrix = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }

    let cleaned: Vec<Vec<char>> = transposed_matrix.iter().filter(|a|{
        let length = a.len();
        let n_spaces = a.iter().filter(|b| b == &&' ').cloned().collect::<Vec<char>>();
        return length != n_spaces.len();
    }).cloned().collect();

    let cleaned_cleaned = cleaned.iter().map(|a| {
        let mut row = a.clone();
        row.retain(|b| b.clone() != ' ');
        return row;
    }).collect();

    return cleaned_cleaned;
}

trait MovementExtensions {
    fn movement_parser(&self) -> Option<Movement>;
}
impl MovementExtensions for &String {
    fn movement_parser(&self) -> Option<Movement> {
        if !self.starts_with("move") {
            return None;
        }
        let raw: Vec<_>= self.split(" ").collect();
        return Some(Movement {
            quantity: raw[1].parse::<i32>().unwrap(),
            start_stack: raw[3].parse::<i32>().unwrap(),
            end_stack: raw[5].parse::<i32>().unwrap(),
        });
    }
}

pub fn day5_2(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let movements: Vec<_> = lines.iter().map(|l| l.movement_parser().unwrap_or(Movement {
        quantity: 0,
        start_stack: 0,
        end_stack: 0,
    })).into_iter().filter(|m| m.quantity != 0).collect();

    let mut stacks = convert_to_list(&lines);
    let executed_stack = execute9001(movements, &mut stacks);

    get_top_stacks(executed_stack)
}

fn execute9001(movements: Vec<Movement>, stacks: &mut Vec<Vec<char>>) -> &Vec<Vec<char>> {
    movements.iter().for_each(|m| {
        let mut collected = vec![];
        let mut counter = 0;
        while counter < m.quantity {
            counter = counter + 1;
            let e = stacks[m.start_stack as usize - 1].remove(0).clone();
            collected.insert(0, e);
        }
        collected.reserve(0);
        collected.iter().for_each(|c| {
            stacks[m.end_stack as usize - 1].insert(0, *c);
        });
    });

    return stacks;
}
