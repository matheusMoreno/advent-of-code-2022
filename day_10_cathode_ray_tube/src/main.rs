use std::fs;

const INPUT_FILE_PATH: &str = "input";

const RELEVANT_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let addx_values: Vec<i64> = parse_commands(&input);
    let register_history: Vec<i64> = execute_operations(&addx_values);
    let strengths_sum: i64 = compute_strengths_sum(&register_history);
    let pixel_positions: Vec<String> = compute_pixel_positions(&register_history);

    println!("Sum of strengths: {strengths_sum}");
    print_screen(&pixel_positions);
}


fn read_input_file(file_path: &str) -> String {
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn parse_commands(input: &str) -> Vec<i64> {
    input.lines().flat_map(
        |line| {
            let mut values: Vec<i64> = vec![0, ];

            if line.starts_with("addx") {
                values.push(line.rsplit(" ").next().unwrap().parse().unwrap());
            }

            values
        }
    ).collect::<Vec<i64>>()
}


fn execute_operations(addx_values: &Vec<i64>) -> Vec<i64> {
    let mut register: i64 = 1;
    let mut register_history: Vec<i64> = vec![register];

    for value in addx_values {
        register += value;
        register_history.push(register);
    }

    register_history
}


fn compute_strengths_sum(register_history: &Vec<i64>) -> i64 {
    register_history.iter().enumerate()
        .filter(|(i, _)| RELEVANT_CYCLES.contains(&(i + 1)))
        .fold(0, |sum, (i, value)| sum + (i as i64 + 1) * value)
}


fn compute_pixel_positions(reg_history: &Vec<i64>) -> Vec<String> {
    let mut pixels: Vec<String> = vec![String::new(); SCREEN_HEIGHT];

    for (i, line) in pixels.iter_mut().enumerate() {
        let positions: &[i64] = &reg_history[i * SCREEN_WIDTH..(i + 1) * SCREEN_WIDTH];

        for (pixel_position_usize, center) in positions.iter().enumerate() {
            let pixel_position: i64 = pixel_position_usize as i64;
            if pixel_position >= center - 1 && pixel_position <= center + 1 {
                line.push_str("#");
            } else {
                line.push_str(".");
            }
        }
    }

    pixels
}


fn print_screen(pixel_positions: &Vec<String>) -> () {
    for line in pixel_positions {
        println!("{:?}", line);
    }
}
