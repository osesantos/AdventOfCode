use std::str::Split;
use crate::utils::get_file_lines;

struct TreeEntry {
    name: String,
    size: usize,
    // file size
    children: Vec<TreeEntry>, // not empty if a dir
}

pub fn day7_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let tree: Vec<TreeEntry> = Vec::new();
    for x in lines {
        let result = parse_line(x);
        if !result.is_some() {
            if !x.contains("cd /") && x.contains("cd") {
                let dir = split_string_nth(&x.split(" "), 2);
                // ...
            }
        }
    }


    "".to_string()
}

fn add_child(line: String, dir: String, tree: Vec<TreeEntry>) -> Vec<TreeEntry> {
    let new_tree: Vec<TreeEntry> = Vec::new();
    tree.iter().for_each(|elem| {
        let new_elem: TreeEntry = TreeEntry::new(elem);
        let line_split = line.split(" ");
        if new_elem.name == dir {
           // ...
        }
    });
    return new_tree

}

fn parse_line(line: String) -> Option<TreeEntry> {
    let split_line = line.split(" ");
    let first_elem = split_string_nth(&split_line, 0);
    if first_elem == "$" {
        return None;
    }
    if first_elem == "dir" {
        return Some(TreeEntry {
            name: split_string_nth(&split_line, 1),
            size: 0,
            children: Vec::new(),
        });
    }
    return Some(TreeEntry {
        name: split_string_nth(&split_line, 1),
        size: split_string_nth(&split_line, 0).parse::<usize>().unwrap(),
        children: Vec::new(),
    });
}

fn split_string_nth(line: &Split<&str>, nth: usize) -> String {
    line.clone().nth(nth).unwrap().to_string()
}

pub fn day7_2(file_name: &str) -> String {
    "".to_string()
}
