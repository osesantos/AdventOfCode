use std::sync::{Arc, Mutex};
use std::thread;


fn get_instruction_position(instructions: Vec<char>, i_instruction: usize) -> usize {
    if i_instruction == instructions.len() - 1 {
        return 0;
    }
    i_instruction + 1
}

fn get_instructions(lines: Vec<String>) -> Vec<char> {
    lines.into_iter().nth(0).unwrap().to_string().chars().collect()
}

pub fn day8_1(input: &Vec<String>) -> u32 {
    let instructions = get_instructions(input.clone());
    let mut counter = 0;

    let mut next_label: String = "AAA".to_string();
    let mut next_instruction_i = 0;

    while next_label != "ZZZ" {
        let instruction = instructions.clone().into_iter().nth(next_instruction_i).unwrap();
        let node = input.clone().into_iter().find(|n| n.contains(&(next_label.to_string() + " ="))).unwrap();
        let left_node_label = node.clone().split(" = ").into_iter().nth(1).unwrap().to_string().replace("(", "").replace(")", "").split(", ").into_iter().nth(0).unwrap().trim().to_string();
        let right_node_label = node.clone().split(" = ").into_iter().nth(1).unwrap().to_string().replace("(", "").replace(")", "").split(", ").into_iter().nth(1).unwrap().trim().to_string();

        if instruction == 'L' {
            next_label = left_node_label.to_string();
        } else {
            next_label = right_node_label.to_string();
        }

        next_instruction_i = get_instruction_position(instructions.clone(), next_instruction_i.clone());
        counter = counter + 1;
    }
    counter
}

fn get_left_label(node: String) -> String {
    node.clone().split(" = ").into_iter().nth(1).unwrap().to_string().replace("(", "").replace(")", "").split(", ").into_iter().nth(0).unwrap().trim().to_string()
}

fn get_right_label(node: String) -> String {
    node.clone().split(" = ").into_iter().nth(1).unwrap().to_string().replace("(", "").replace(")", "").split(", ").into_iter().nth(1).unwrap().trim().to_string()
}

fn get_node_label(node: String) -> String {
    node.clone().split(" = ").into_iter().nth(0).unwrap().trim().to_string()
}

// end when all the end nodes have XXZ
// starting nodes are the same len as end nodes
pub fn day8_2(input: Vec<String>) -> u32 {
    let counter = Arc::new(Mutex::new(0));
    let counter_z_nodes = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    let active_nodes: Arc<Vec<String>> = Arc::from(input.clone().into_iter().filter(|l| l.contains("A =")).collect());

    for node in active_nodes.to_vec() {
        let counter = Arc::clone(&counter);
        let counter_z_nodes = Arc::clone(&counter_z_nodes);
        let instructions: Arc<Vec<char>> = Arc::from(get_instructions(input.clone()));
        let inner_node: Arc<String> = Arc::from(node.clone());
        let lines: Arc<Vec<String>> = Arc::from(input.clone());

        let handle = thread::spawn(move || {
            let mut next_instruction_i = 1;
            let mut next_node: String = inner_node.to_string();
            println!("[{inner_node}] Starting a new thread...");
            while get_node_label(next_node.clone()).chars().last().unwrap() != 'Z' {
                let instruction = instructions.to_vec().into_iter().nth(next_instruction_i).unwrap();

                println!("[{inner_node}] instruction {instruction}");
                if instruction == 'L' {
                    let left_node = lines.to_vec().into_iter().find(|n| n.contains(&(get_left_label(next_node.clone().to_string()).to_string() + " ="))).unwrap();
                    next_node = left_node;
                } else {
                    let right_node = lines.to_vec().into_iter().find(|n| n.contains(&(get_right_label(next_node.clone().to_string()).to_string() + " ="))).unwrap();
                    next_node = right_node;
                }
                println!("[{inner_node}] next node is {next_node}");

                next_instruction_i = get_instruction_position(instructions.to_vec(), next_instruction_i.clone());
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("[{inner_node}] counter {num}");
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = *counter.lock().unwrap(); x
}
