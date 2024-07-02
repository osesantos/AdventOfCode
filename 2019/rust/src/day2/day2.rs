const END_OPCODE: i32 = 99;
const ADD_OPCODE: i32 = 1;
const MULTIPLY_OPCODE: i32 = 2;

pub fn day2() {
    let input = include_str!("input.txt");
    println!("day2.1 - {}", execute_part_1(input));
    println!("day2.2 - {}", execute_part_2(input));
}

fn execute_part_1(input: &str) -> i32 {
    let mut program = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut i = 0;
    let mut first_i = 0;
    loop {
        let operation = extract_operation(program.clone(), i).unwrap_or_else(|vec| vec);
        let opcode = operation[0];

        program = match opcode {
            // Program halts
            END_OPCODE => break,
            // Add
            ADD_OPCODE => add(program.clone(), operation),
            // Multiply
            MULTIPLY_OPCODE => multiply(program.clone(), operation),
            _ => continue
        };

        i = i + 4;
        first_i = program[0]
    }

    return first_i
}

fn extract_operation(program: Vec<i32>, index: usize) -> Result<Vec<i32>, Vec<i32>> {
    if index + 4 > program.len() {
        Err([program[index]].to_vec())
    } else {
        Ok(program[index..index + 4].to_vec())
    }
}

fn execute_part_2(input: &str) -> i32 {
    0
}

fn add(mut program: Vec<i32>, operation: Vec<i32>) -> Vec<i32> {
    let pos1 = operation[1] as usize;
    let pos2 = operation[2] as usize;
    let pos3 = operation[3] as usize;

    let sum = program[pos1] + program[pos2];

    program[pos3] = sum;

    program.clone()
}

fn multiply(mut program: Vec<i32>, operation: Vec<i32>) -> Vec<i32> {
    let pos1 = operation[1] as usize;
    let pos2 = operation[2] as usize;
    let pos3 = operation[3] as usize;

    let multiply = program[pos1] * program[pos2];

    program[pos3] = multiply;

    program.clone()
}

#[cfg(test)]
mod day2 {
    use super::*;

    #[test]
    fn assert_1() {
        assert_eq!(execute_part_1("1,0,0,0,99"), 2);
        assert_eq!(execute_part_1("2,3,0,3,99"), 2);
        assert_eq!(execute_part_1("2,4,4,5,99,0"), 2);
        assert_eq!(execute_part_1("1,1,1,4,99,5,6,0,99"), 30);
    }

    #[test]
    fn assert_2() {}
}
