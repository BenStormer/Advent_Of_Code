use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Read input into vector of (x, y) tuples to represent tiles
 * 2. For each tile, compute the area between it and all other tiles,
 *    and track the max value
 * 3. Return the max value
 */

type Coordinate = (i32, i32);

fn main() {
    let mut coordinates: Vec<Coordinate> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/9.txt") {
        for line in lines.map_while(Result::ok) {
            let coordinate_vector: Vec<i32> = line
                .split(',')
                .map(|s| s.parse::<i32>().expect("Invalid input"))
                .collect();
            let coordinate = (coordinate_vector[0], coordinate_vector[1]);
            coordinates.push(coordinate);
        }
    }

    let mut max_area: i64 = -1;
    for coordinate_1_index in 0..coordinates.len() {
        for coordinate_2_index in (coordinate_1_index + 1)..coordinates.len() {
            let coordinate_1 = coordinates[coordinate_1_index];
            let coordinate_2 = coordinates[coordinate_2_index];
            let area = calculate_area(coordinate_1, coordinate_2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Max Area: {max_area}");
}

fn calculate_area(coordinate_1: Coordinate, coordinate_2: Coordinate) -> i64 {
    let x_length = ((coordinate_1.0 - coordinate_2.0).abs() + 1) as i64;
    let y_length = ((coordinate_1.1 - coordinate_2.1).abs() + 1) as i64;
    return x_length * y_length;
}
