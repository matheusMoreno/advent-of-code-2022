use std::fs;

use regex::Regex;

const INPUT_FILE_PATH: &str = "input";
const CRATE_REGEX: &str = r"\[(\w)\]";
const MOVE_ACTION_REGEX: &str = r"move (\d+) from (\d+) to (\d+)";
const STACKS_NUMBER_LINE_REGEX: &str = r"( \d+ )+";

const CRATE_TEXT_LEN: usize = 3;


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut first_line: bool = true;

    for line in input.lines() {
        if is_stacks_number_line(line) {
            break
        }

        if first_line {
            stacks = generate_stacks_vector(line);
            first_line = false;
        }

        add_crates(&mut stacks, line);
    }

    // Reverse stack order so that .pop() and .push() works
    for i in 0..stacks.len() {
        stacks[i].reverse();
    }

    input.lines().for_each(|l| move_crates_collectively(&mut stacks, l));
    let top_crates: String = get_top_crates(&stacks);

    println!("The sequence of top crates is {top_crates}.");
}


fn read_input_file(file_path: &str) -> String {
    // Reads the entire file into memory at once; could be a problem
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn generate_stacks_vector(first_line: &str) -> Vec<Vec<String>> {
    // Each crate occupies 3 spaces, and one space separates each crate
    let n_stacks: usize = (first_line.len() + 1) / 4;
    (0..n_stacks).map(|_| Vec::new()).collect::<Vec<Vec<String>>>()
}


fn is_stacks_number_line(line: &str) -> bool {
    let number_line_regex: Regex = Regex::new(STACKS_NUMBER_LINE_REGEX).unwrap();
    match number_line_regex.captures(line) {
        Some(_) => true,
        None => false,
    }
}


fn add_crates(stacks: &mut Vec<Vec<String>>, line: &str) -> () {
    let n_stacks: usize = stacks.len();
    let crate_regex: Regex = Regex::new(CRATE_REGEX).unwrap();

    for i in 0..n_stacks {
        let start: usize = i * (CRATE_TEXT_LEN + 1);
        let end: usize = start + CRATE_TEXT_LEN;

        match crate_regex.captures(&line[start..end]) {
            Some(capture) => stacks[i].push(String::from(&capture[1])),
            None => continue,
        };
    }
}


#[allow(dead_code)]
fn move_crates_individually(stacks: &mut Vec<Vec<String>>, line: &str) -> () {
    let (n, from, to): (usize, usize, usize) = match parse_move_line(line) {
        Some(value) => value,
        None => return,
    };

    for _ in 0..n {
        let crate_name: String = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(crate_name);
    }
}


fn move_crates_collectively(stacks: &mut Vec<Vec<String>>, line: &str) -> () {
    let (n, from, to): (usize, usize, usize) = match parse_move_line(line) {
        Some(value) => value,
        None => return,
    };

    let from_index: usize = stacks[from - 1].len() - n;
    let mut from_clone: Vec<String> = vec![String::new(); n];

    from_clone.clone_from_slice(&stacks[from - 1][from_index..]);
    stacks[to - 1].append(&mut from_clone);
    stacks[from - 1].truncate(from_index);
}


fn get_top_crates(stacks: &Vec<Vec<String>>) -> String {
    let mut top_crates: String = String::new();

    for stack in stacks {
        top_crates.push_str(stack.last().unwrap_or(&String::new()));
    }

    top_crates
}


fn parse_move_line(line: &str) -> Option<(usize, usize, usize)> {
    let move_regex: Regex = Regex::new(MOVE_ACTION_REGEX).unwrap();

    match move_regex.captures(line) {
        Some(capture) => Some((
            capture[1].parse::<usize>().unwrap(),
            capture[2].parse::<usize>().unwrap(),
            capture[3].parse::<usize>().unwrap(),
        )),
        None => None,
    }
}
