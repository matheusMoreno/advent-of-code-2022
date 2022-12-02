use std::fs;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT_FILE_PATH: &str = "input";
const N_OF_ELEMENTS: usize = 3;


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let mut calories_greatest: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut calories_current: usize = 0;

    for line in input.lines() {
        let calories: usize = match line.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                update_heap(&mut calories_greatest, calories_current);
                calories_current = 0;
                0
            },
        };

        calories_current += calories;
    }

    let calories_greatest_sum: usize = calories_greatest.iter().map(|x| x.0).sum();

    println!(
        "The {N_OF_ELEMENTS} elves with the most calories have, \
        in total, {calories_greatest_sum} cal."
    );
}


fn read_input_file(file_path: &str) -> String {
    // Reads the entire file into memory at once; could be a problem
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn update_heap(heap: &mut BinaryHeap<Reverse<usize>>, value: usize) -> () {
    // Code from https://users.rust-lang.org/ forum
    heap.push(Reverse(value));
    if heap.len() > N_OF_ELEMENTS {
        heap.pop();
    }
}
