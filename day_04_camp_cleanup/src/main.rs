use std::fs;

use regex;

const INPUT_FILE_PATH: &str = "input";
const INPUT_LINE_REGEX: &str = r"(\d+)-(\d+),(\d+)-(\d+)";

fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);
    let line_regex: regex::Regex = regex::Regex::new(INPUT_LINE_REGEX).unwrap();

    let mut n_full_overlaps: u32 = 0;
    let mut n_partial_overlaps: u32 = 0;

    for capture in line_regex.captures_iter(&input) {
        let (interval_a, interval_b): (Interval, Interval) = build_intervals(capture);
        n_full_overlaps += check_full_overlap(&interval_a, &interval_b) as u32;
        n_partial_overlaps += check_partial_overlap(&interval_a, &interval_b) as u32;
    }

    println!("The number of full overlapping intervals is {n_full_overlaps}.");
    println!("The number of partial overlapping intervals is {n_partial_overlaps}.");
}

fn read_input_file(file_path: &str) -> String {
    // Reads the entire file into memory at once; could be a problem
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}

fn build_intervals(capture: regex::Captures) -> (Interval, Interval) {
    (
        Interval {
            start: capture[1].parse().unwrap(),
            end: capture[2].parse().unwrap(),
        },
        Interval {
            start: capture[3].parse().unwrap(),
            end: capture[4].parse().unwrap(),
        },
    )
}

fn check_full_overlap(interval_a: &Interval, interval_b: &Interval) -> bool {
    let start_difference: i32 = interval_a.start as i32 - interval_b.start as i32;
    let end_difference: i32 = interval_a.end as i32 - interval_b.end as i32;

    if (start_difference >= 0) && (end_difference <= 0) {
        return true;
    } else if (start_difference <= 0) && (end_difference >= 0) {
        return true;
    }

    false
}

fn check_partial_overlap(interval_a: &Interval, interval_b: &Interval) -> bool {
    if (interval_a.start > interval_b.end) || (interval_a.end < interval_b.start) {
        return false
    }

    true
}

struct Interval {
    start: u8,
    end: u8,
}
