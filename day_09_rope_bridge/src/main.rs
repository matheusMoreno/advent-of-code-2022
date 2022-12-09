use std::cmp;
use std::collections::HashSet;
use std::fs;

const INPUT_FILE_PATH: &str = "input";


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let movements: String = generate_move_list(input);
    let tail_positions_two: usize = compute_positions_n_knots(&movements, 2);
    let tail_positions_ten: usize = compute_positions_n_knots(&movements, 10);

    println!("Number of unique tail positions for 2 knots: {tail_positions_two}");
    println!("Number of unique tail positions for 10 knots: {tail_positions_ten}");
}


fn read_input_file(file_path: &str) -> String {
    // Reads the entire file into memory at once; could be a problem
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


struct Point {
    x: i64,
    y: i64,
    history: HashSet<(i64, i64)>,
}

impl Point {
    pub fn new() -> Point {
        Point {x: 0, y: 0, history: HashSet::from([(0, 0)])}
    }

    pub fn is_touching(&self, other: &Point) -> bool {
        (other.x - self.x).abs() <= 1 && (other.y - self.y).abs() <= 1
    }

    pub fn walk(&mut self, direction: char) -> () {
        match direction {
            'R' => { self.x += 1; },
            'L' => { self.x += -1; },
            'U' => { self.y += 1; },
            'D' => { self.y += -1; },
            _ => (),
        }

        self.history.insert((self.x, self.y));
    }

    pub fn follow(&mut self, other_x: i64, other_y: i64) -> () {
        let stub_point: Point = Point { x: other_x, y: other_y, history: HashSet::new() };

        if !self.is_touching(&stub_point) {
            let dist_x: i64 = other_x - self.x;
            let dist_y: i64 = other_y - self.y;

            match (dist_x, dist_y) {
                (0, _) => { self.y += dist_y - dist_y.signum(); },
                (_, 0) => { self.x += dist_x - dist_x.signum(); },
                (_, _) => {
                    let dist_diag: i64 = cmp::max(dist_x.abs(), dist_y.abs()) - 1;
                    self.x += dist_diag * dist_x.signum();
                    self.y += dist_diag * dist_y.signum();
                },
            }

            self.history.insert((self.x, self.y));
        }
    }
}


fn generate_move_list(input: String) -> String {
    input.lines().flat_map(|l| {
            let mut line_chars = l.split(" ");
            let movement: &str = line_chars.next().unwrap();
            let times: usize = line_chars.next().unwrap().parse().expect("Error!");
            movement.repeat(times).chars().collect::<Vec<char>>()
        }
    ).collect::<String>()
}


fn compute_positions_n_knots(movements: &String, n: usize) -> usize {
    let mut knots: Vec<Point> = (0..n).map(|_| Point::new()).collect::<Vec<Point>>();

    for movement in movements.chars() {
        knots[0].walk(movement);
        (1..n).for_each(|i| {
            let knot_previous_x: i64 = knots[i - 1].x;
            let knot_previous_y: i64 = knots[i - 1].y;
            knots[i].follow(knot_previous_x, knot_previous_y);
        })
    }

    knots.last().unwrap().history.len()
}
