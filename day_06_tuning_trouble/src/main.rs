use std::fs;
use std::collections::HashSet;


const INPUT_FILE_PATH: &str = "input";
const START_OF_PACKET_MARKER_SIZE: usize = 4;
const START_OF_MESSAGE_MARKER_SIZE: usize = 14;


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let first_packet_index: usize = find_first_marker_index(
        &input, START_OF_PACKET_MARKER_SIZE);
    let first_message_index: usize = find_first_marker_index(
        &input, START_OF_MESSAGE_MARKER_SIZE);

    println!(
        "The first start-of-packet marker index is {first_packet_index}; \
        the first start-of-message marker index is {first_message_index}."
    );
}


fn read_input_file(file_path: &str) -> String {
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn find_first_marker_index(datastream: &str, marker_size: usize) -> usize {
    let mut i: usize = 0;

    loop {
        let window: &str = &datastream[i..(i + marker_size)];
        let window_char_set: HashSet<char> = window.chars().collect();

        if window_char_set.len() == marker_size {
            return i + marker_size
        }

        i += 1;
    }
}
