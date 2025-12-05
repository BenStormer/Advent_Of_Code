use aoc_problems::read_lines;
use std::collections::HashSet;

/*
 * Algorithm
 * 1. Read input into a 2d array
 * 2. Loop through array and add the coordinates of every roll to a set
 * 3. Loop through set and for each coordinate, see if it's neighbors are
 *    in the set
 *    - A neighbor being defined as coordinates from
 *      ([i - 1],[j - 1]) to ([i + 1],[j + 1])
 */

const THRESHOLD: i32 = 4;

fn main() {
    let mut count = 0;
    if let Ok(lines) = read_lines("./inputs/4.txt") {
        // Read all chars into a 2D vector
        let mut char_grid: Vec<Vec<char>> = Vec::new();
        for line in lines.map_while(Result::ok) {
            char_grid.push(line.chars().collect());
        }

        // Store all coordinates for locations with paper rools
        let mut paper_coordinates: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..char_grid.len() {
            for j in 0..char_grid[i].len() {
                if char_grid[i][j] == '@' {
                    paper_coordinates.insert((i as i32, j as i32));
                }
            }
        }
        println!("Paper coordinates: {:?}\n", paper_coordinates);

        for coordinate in &paper_coordinates {
            if neighbor_count_below_threshold(&paper_coordinates, *coordinate) {
                println!("Coordinate: {:?} can be accessed", coordinate);
                count += 1;
            }
        }
    }
    println!("\nFinal count: {}", count);
}

fn neighbor_count_below_threshold(
    paper_coordinates: &HashSet<(i32, i32)>,
    (row, col): (i32, i32),
) -> bool {
    let mut paper_neighbors = 0;
    for i in row - 1..=row + 1 {
        for j in col - 1..=col + 1 {
            if i == row && j == col {
                continue;
            }
            if paper_coordinates.contains(&(i, j)) {
                paper_neighbors += 1;
            }
        }
    }
    if paper_neighbors < THRESHOLD {
        return true;
    }
    return false;
}
