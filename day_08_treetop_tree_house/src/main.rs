use std::fs;

const INPUT_FILE_PATH: &str = "input";


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let heights_matrix: Vec<Vec<u8>> = parse_matrix(input);
    let highest_scenic_score: usize = get_highest_scenic_score(&heights_matrix);
    let visibility_matrix: Vec<Vec<u8>> = compute_visibility_matrix(heights_matrix);
    let visible_trees: u64 = sum_matrix_elements(&visibility_matrix) as u64;

    println!("Number of visible trees: {visible_trees}");
    println!("Highest scenic score possible: {highest_scenic_score}");
}


fn read_input_file(file_path: &str) -> String {
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn parse_matrix(input: String) -> Vec<Vec<u8>> {
    input.lines().map(
        |l| l.chars().map(
            |c| c.to_digit(10).unwrap() as u8
        ).collect::<Vec<u8>>()
    ).collect::<Vec<Vec<u8>>>()
}


fn compute_visibility_matrix(mut matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n_rows: usize = matrix.len();
    let n_columns: usize = matrix[0].len();

    let mut visibility_matrix: Vec<Vec<u8>> = vec![vec![0; n_columns]; n_rows];

    // Left to right
    update_visibility_matrix_rows(&matrix, &mut visibility_matrix);

    // Right to left
    visibility_matrix = reverse_matrix_rows(&visibility_matrix);
    matrix = reverse_matrix_rows(&matrix);
    update_visibility_matrix_rows(&matrix, &mut visibility_matrix);

    // Down to up
    visibility_matrix = transpose_matrix(&visibility_matrix);
    matrix = transpose_matrix(&matrix);
    update_visibility_matrix_rows(&matrix, &mut visibility_matrix);

    // Up to down
    visibility_matrix = reverse_matrix_rows(&visibility_matrix);
    matrix = reverse_matrix_rows(&matrix);
    update_visibility_matrix_rows(&matrix, &mut visibility_matrix);

    visibility_matrix
}


fn update_visibility_matrix_rows(
    matrix: &Vec<Vec<u8>>, visibility_matrix: &mut Vec<Vec<u8>>
) -> () {
    matrix.iter().zip(visibility_matrix.iter_mut()).for_each(
        |(row_matrix, row_visibility)| {
            row_matrix.iter().zip(row_visibility.iter_mut()).fold(
                -1,
                |max_value, (elem_matrix, elem_visibility)| {
                    if (*elem_matrix as i32) > max_value {
                        *elem_visibility = 1;
                        return *elem_matrix as i32;
                    }

                    max_value
                }
            );
        }
    );
}


fn reverse_matrix_rows(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    matrix.iter().map(
        |v| v.iter().map(|e| *e).rev().collect::<Vec<u8>>()
    ).collect::<Vec<Vec<u8>>>()
}


fn transpose_matrix(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n_columns: usize = matrix[0].len();

    (0..n_columns).map(
        |i| matrix.iter().map(|v| v[i]).collect::<Vec<u8>>()
    ).collect::<Vec<Vec<u8>>>()
}


fn sum_matrix_elements(matrix: &Vec<Vec<u8>>) -> i64 {
    matrix.iter().fold(0,|sum, row| sum + row.iter().map(|x| *x as i64).sum::<i64>())
}


fn get_highest_scenic_score(matrix: &Vec<Vec<u8>>) -> usize {
    let n_rows: usize = matrix.len();
    let n_columns: usize = matrix[0].len();

    (1..n_rows - 1).fold(
        0,
        |highest_score, i| {
            let highest_score_line: usize = (1..n_columns - 1).fold(
                0,
                |highest_score_line, j| {
                    let tree_size: u8 = matrix[i][j];

                    let left: usize = matrix[i][..j].iter().rev()
                        .position(|&x| x >= tree_size)
                        .unwrap_or(matrix[i][..j].len() - 1) + 1;
                    let right: usize = matrix[i][j + 1..].iter()
                        .position(|&x| x >= tree_size)
                        .unwrap_or(matrix[i][j + 1..].len() - 1) + 1;
                    let up: usize = (0..i).map(|k| matrix[k][j]).rev()
                        .position(|x| x >= tree_size)
                        .unwrap_or((0..i).len() - 1) + 1;
                    let down: usize = (i + 1..n_rows).map(|k| matrix[k][j])
                        .position(|x| x >= tree_size)
                        .unwrap_or((i + 1..n_rows).len() - 1) + 1;

                    let scenic_score: usize = left * right * up * down;

                    if scenic_score > highest_score_line {
                        return scenic_score;
                    }

                    highest_score_line
                }
            );

            if highest_score_line > highest_score {
                return highest_score_line;
            };

            highest_score
        }
    )
}
