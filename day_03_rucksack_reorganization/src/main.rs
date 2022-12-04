use std::fs;
use std::collections::HashSet;

const INPUT_FILE_PATH: &str = "input";

const LOWERCASE_OFFSET: u32 = 'a' as u32;
const UPPERCASE_OFFSET: u32 = 'A' as u32;
const N_OF_LETTERS: u32 = 26;


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let priorities_sum: u64 = get_priorities_sum(&input);
    let badges_priorities_sum: u64 = get_badges_priorities_sum(&input);

    println!("The sum of priorities is {priorities_sum}.");
    println!("The sum of priorities for the badges is {badges_priorities_sum}.");
}


fn read_input_file(file_path: &str) -> String {
    // Reads the entire file into memory at once; could be a problem
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn get_priorities_sum(input_str: &str) -> u64 {
    let mut priorities_sum: u64 = 0;

    for rusack in input_str.lines() {
        let (compartment_a, compartment_b): (&str, &str) = split_string_in_half(rusack);

        let set_a: HashSet<char> = create_hash_set_of_chars(compartment_a);
        let set_b: HashSet<char> = create_hash_set_of_chars(compartment_b);

        let shared_item: char = get_element_in_common(&set_a, &set_b);
        priorities_sum += get_priority_value(shared_item);
    }

    priorities_sum
}


fn get_badges_priorities_sum(input_str: &str) -> u64 {
    let mut badges_priorities_sum: u64 = 0;

    for rusack_chunk in input_str.lines().collect::<Vec<&str>>().chunks(3) {
        // VERY messy, but better than instantiating every hash map individually
        let intersection_set: HashSet<char> = rusack_chunk.iter()
            .fold(HashSet::new(), |set, x| {
                if set.is_empty() {
                    create_hash_set_of_chars(x)
                } else {
                    get_intersection(&set, &create_hash_set_of_chars(x))
                }
            });

        let shared_item: char = intersection_set.iter().map(|x| *x).next().expect("Error!");
        badges_priorities_sum += get_priority_value(shared_item);
    }

    badges_priorities_sum
}


fn split_string_in_half(string: &str) -> (&str, &str) {
    (&string[..string.len() / 2], &string[string.len() / 2..])
}


fn create_hash_set_of_chars(string: &str) -> HashSet<char> {
    HashSet::from_iter(string.chars())
}


fn get_element_in_common(set_a: &HashSet<char>, set_b: &HashSet<char>) -> char {
    set_a.intersection(&set_b).map(|x| *x).next().expect("Error!")
}


fn get_intersection(set_a: &HashSet<char>, set_b: &HashSet<char>) -> HashSet<char> {
    set_a.intersection(&set_b).map(|x| *x).collect()
}


fn get_priority_value(item: char) -> u64 {
    let item_ascii: u32 = item as u32;

    // We need to do this check because the ASCII values of uppercase letters is
    // actually lesser than the values of lowercases
    if item_ascii >= LOWERCASE_OFFSET {
        return (item_ascii - LOWERCASE_OFFSET + 1).into()
    }

    (item_ascii - UPPERCASE_OFFSET + N_OF_LETTERS + 1).into()
}
