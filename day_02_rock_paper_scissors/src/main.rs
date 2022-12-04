use std::fs;
use std::str;

const INPUT_FILE_PATH: &str = "input";

const VICTORY_POINTS: usize = 6;
const TIE_POINTS: usize = 3;
const DEFEAT_POINTS: usize = 0;

const ROCK: Shape = Shape { encoding: 'A', beats: 'C', loses: 'B', points: 1 };
const PAPER: Shape = Shape { encoding: 'B', beats: 'A', loses: 'C', points: 2 };
const SCISSORS: Shape = Shape { encoding: 'C', beats: 'B', loses: 'A', points: 3 };


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let mut score: usize = 0;
    for line in input.lines() {
        let (shape_opponent, shape_player): (Shape, Shape) = get_round_shapes(line);
        score += shape_player.points + shape_player.get_outcome_points(&shape_opponent);
    }

    println!("Your total score is {score}.");
}


fn read_input_file(file_path: &str) -> String {
    // Reads the entire file into memory at once; could be a problem
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn get_round_shapes(input_line: &str) -> (Shape, Shape) {
    let shapes: Vec<&str> = input_line.split(" ").collect();
    let opponent_shape: Shape = get_shape_from_encoding(shapes[0]);
    let player_shape: Shape = get_player_shape(&opponent_shape, shapes[1]);
    (opponent_shape, player_shape)
}


struct Shape {
    encoding: char,
    beats: char,
    loses: char,
    points: usize,
}

impl Shape {
    fn get_outcome_points(&self, shape_opponent: &Shape) -> usize {
        match shape_opponent.encoding {
            x if x == self.beats => VICTORY_POINTS,
            x if x == self.encoding => TIE_POINTS,
            _ => DEFEAT_POINTS,
        }
    }
}


fn get_shape_from_encoding(value: &str) -> Shape {
    match value {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS,
        _ => todo!(),
    }
}


fn get_player_shape(opponent_shape: &Shape, strategy: &str) -> Shape {
    // This strategy is from part 2 of the puzzle
    let player_encoding: char = match strategy {
        "X" => opponent_shape.beats,
        "Y" => opponent_shape.encoding,
        "Z" => opponent_shape.loses,
        _ => todo!(),
    };

    get_shape_from_encoding(&player_encoding.to_string())
}
