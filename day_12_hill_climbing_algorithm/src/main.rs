use std::fs;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;

const INPUT_FILE_PATH: &str = "input";

const MIN_HEIGHT_CHAR: char = 'a';
const MAX_HEIGHT_CHAR: char = 'z';
const START_POINT_CHAR: char = 'S';
const END_POINT_CHAR: char = 'E';
const MAXIMUM_DISTANCE: i32 = 10_000_000;


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let mut area_matrix: Vec<Vec<char>> = build_area_matrix(&input);
    let (start, end): (usize, usize) = find_start_and_end_nodes(&mut area_matrix);
    let graph: DiGraph<usize, ()> = build_unidirected_graph(&area_matrix);

    // Part 1
    let minimum_distance = compute_minimum_distance(&graph, start, end);

    // Part 2
    let starts: Vec<usize> = get_possible_starting_points(&area_matrix);
    let minimum_lowest: i32 = compute_minimum_distance_multiple(&graph, &starts, end);

    println!("The shortest path length is: {minimum_distance}");
    println!("The minimum distance from any '{MIN_HEIGHT_CHAR}' is: {minimum_lowest}");
}


fn read_input_file(file_path: &str) -> String {
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


fn build_area_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(
        |line| line.chars().collect::<Vec<char>>()
    ).collect::<Vec<Vec<char>>>()
}


fn find_start_and_end_nodes(matrix: &mut Vec<Vec<char>>) -> (usize, usize) {
    let columns: usize = matrix[0].len();
    let matrix_flat: Vec<char> = matrix.iter().flatten().map(|c| *c).collect();

    // Start
    let start: usize = matrix_flat.iter().position(|x| *x == START_POINT_CHAR).unwrap();
    matrix[start / columns][start % columns] = MIN_HEIGHT_CHAR;

    // End
    let end: usize = matrix_flat.iter().position(|x| *x == END_POINT_CHAR).unwrap();
    matrix[end / columns][end % columns] = MAX_HEIGHT_CHAR;

    (start, end)
}


fn build_unidirected_graph(matrix: &Vec<Vec<char>>) -> DiGraph<usize, ()> {
    let rows: usize = matrix.len();
    let columns: usize = matrix[0].len();

    let nodes_matrix: Vec<Vec<usize>> = (0..rows).map(
        |r| (0..columns).map(|c| r * columns + c).collect::<Vec<usize>>()
    ).collect::<Vec<Vec<usize>>>();

    let mut graph: DiGraph<usize, ()> = DiGraph::<usize, ()>::new();
    let nodes = nodes_matrix.iter().map(
        |r| r.iter().map(|c| graph.add_node(*c)).collect::<Vec<NodeIndex>>()
    ).collect::<Vec<Vec<NodeIndex>>>();

    for i in 0..rows {
        for j in 0..columns {
            let neighbors: Vec<(usize, usize)> = vec![
                (i as i64 + 1, j as i64),
                (i as i64 - 1, j as i64),
                (i as i64, j as i64 + 1),
                (i as i64, j as i64 - 1),
            ].iter().filter(
                |(x, y)|
                *x >= 0 && *x < rows as i64 && *y >= 0 && *y < columns as i64
            ).map(
                |(x, y)|
                (*x as usize, *y as usize)
            ).filter(
                |(x, y)|
                matrix[*x][*y] as u8 <= matrix[i][j] as u8 + 1
            ).collect::<Vec<(usize, usize)>>();

            for (x, y) in neighbors {
                let node_x: NodeIndex = nodes[i][j];
                let node_y: NodeIndex = nodes[x][y];
                graph.add_edge(node_x, node_y, ());
            }
        }
    }

    graph
}


fn compute_minimum_distance(
    graph: &DiGraph<usize, ()>, start: usize, end: usize
) -> i32 {
    *dijkstra(
        graph, NodeIndex::new(start), Some(NodeIndex::new(end)), |_| 1
    ).get(&NodeIndex::new(end)).unwrap_or(&MAXIMUM_DISTANCE)
}


fn get_possible_starting_points(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    matrix.iter().flatten().enumerate()
        .filter(|(_, c)| **c == MIN_HEIGHT_CHAR)
        .map(|(i, _)| i).collect::<Vec<usize>>()
}


fn compute_minimum_distance_multiple(
    graph: &DiGraph<usize, ()>, starts: &Vec<usize>, end: usize
) -> i32 {
    starts.iter().map(
        |start| compute_minimum_distance(&graph, *start, end)
    ).min().unwrap()
}
