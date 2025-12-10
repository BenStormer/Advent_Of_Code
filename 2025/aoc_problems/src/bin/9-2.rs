use aoc_problems::read_lines;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/*
 * NOTE: Algorithm is INCORRECT. There is likely an issue in how the bounds are
 *       determined, I was able to draw shapes that are incorrectly mapped.
 *       However, I couldn't come up with the proper solution in time.
 * NOTE 2: Algorithm is somewhat slow, it is recommended to build the release
 *         version and run that (`cargo run --release --bin 9-2`)
 *
 * Algorithm:
 * 1. Read input into vector of (x, y) tuples to represent red tiles
 * 2. For each column in the grid, determine the (y_start, y_end) ranges that
 *    mark the edge of the shape and store in HashMap (col, (y_start, y_end))
 * 3. For each red tile, draw a rectangle with the subsequent red tile and get
 *    the area it forms, if all edges are bound.
 *      - A rectangle is bound if all edge pieces are bound
 *      - Determine if edge piece is bound by looking it up in the bounds
 *        mapping we made
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
            let x_val = coordinate_vector[0];
            let y_val = coordinate_vector[1];

            // Add coordinate to coordinates vector
            let coordinate = (x_val, y_val);
            coordinates.push(coordinate);
        }
    }

    let edge_tiles = get_edge_tiles(&coordinates);
    let column_bounds = get_column_bounds(&edge_tiles);

    let mut max_area: i64 = -1;
    for coordinate_1_index in 0..coordinates.len() {
        println!(
            "Checking coordinate {:?} of {:?}",
            coordinate_1_index + 1,
            coordinates.len()
        );
        for coordinate_2_index in (coordinate_1_index + 1)..coordinates.len() {
            let coordinate_1 = coordinates[coordinate_1_index];
            let coordinate_2 = coordinates[coordinate_2_index];
            // println!("Comparing {:?} and {:?}", coordinate_1, coordinate_2);

            // Check if all edge pieces are bounded
            let x_min = min(coordinate_1.0, coordinate_2.0);
            let x_max = max(coordinate_1.0, coordinate_2.0);
            let y_min = min(coordinate_1.1, coordinate_2.1);
            let y_max = max(coordinate_1.1, coordinate_2.1);

            let mut is_bound = true;
            for x_val in x_min..=x_max {
                if !is_bound {
                    break;
                }
                for y_val in y_min..=y_max {
                    if !is_bound {
                        break;
                    }
                    let coordinate = (x_val, y_val);
                    is_bound = check_if_bound(&coordinate, &column_bounds);
                }
            }
            if !is_bound {
                break;
            }

            let area = calculate_area(coordinate_1, coordinate_2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Max Area: {max_area}");
}

fn get_edge_tiles(coordinates: &Vec<Coordinate>) -> HashSet<Coordinate> {
    let mut edge_tiles: HashSet<Coordinate> = HashSet::new();
    for coordinate_1_index in 0..coordinates.len() {
        let mut coordinate_2_index: usize = 0;
        if coordinate_1_index != coordinates.len() - 1 {
            coordinate_2_index = coordinate_1_index + 1;
        }

        let coordinate_1 = coordinates[coordinate_1_index];
        let coordinate_2 = coordinates[coordinate_2_index];

        // Determine if x_val or y_val is different between the coordinates
        let x1 = coordinate_1.0;
        let x2 = coordinate_2.0;
        let y1 = coordinate_1.1;
        let y2 = coordinate_2.1;
        if x1 - x2 != 0 {
            if x1 < x2 {
                for x_val in x1..=x2 {
                    edge_tiles.insert((x_val, y1));
                }
            } else {
                for x_val in x2..=x1 {
                    edge_tiles.insert((x_val, y1));
                }
            }
        } else {
            if y1 < y2 {
                for y_val in y1..=y2 {
                    edge_tiles.insert((x1, y_val));
                }
            } else {
                for y_val in y2..=y1 {
                    edge_tiles.insert((x1, y_val));
                }
            }
        }
    }
    return edge_tiles;
}

fn get_column_bounds(edge_tiles: &HashSet<Coordinate>) -> BTreeMap<i32, BTreeSet<(i32, i32)>> {
    // For each column in the grid, we want to figure out the y value bounds
    // that denote the shape edges. So, any point inside these bounds is part
    // of the shape. However, there may be multiple bounds per column that we
    // need to account for

    // Get Map of all column indexes and any y_values in them
    let mut unjoined_bounds: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    for tile in edge_tiles {
        let x_val = tile.0;
        let y_val = tile.1;
        if unjoined_bounds.contains_key(&x_val) {
            let y_bounds_set = unjoined_bounds
                .get_mut(&x_val)
                .expect("Invalid y_bounds vector");
            y_bounds_set.insert(y_val);
        } else {
            let mut y_bounds_set: BTreeSet<i32> = BTreeSet::new();
            y_bounds_set.insert(y_val);
            unjoined_bounds.insert(x_val, y_bounds_set);
        }
    }
    println!("Unjoined bounds: {:?}\n", unjoined_bounds);

    // Combine y_bounds to only be ranges and not all edge values
    let mut bounds: BTreeMap<i32, BTreeSet<(i32, i32)>> = BTreeMap::new();
    for (column, y_val_set) in unjoined_bounds.iter() {
        let mut column_bounds: BTreeSet<(i32, i32)> = BTreeSet::new();

        let mut bound_start = i32::MIN;
        let mut last_val = i32::MIN;
        let mut in_vertical_line = false;
        for y_val in y_val_set {
            // We are in vertical row
            if *y_val == last_val + 1 {
                if bound_start == i32::MIN {
                    bound_start = last_val;
                }
                in_vertical_line = true;
                last_val = *y_val;
                continue;
            // We just finished a vertical row
            } else if in_vertical_line {
                column_bounds.insert((bound_start, last_val));
                in_vertical_line = false;
                bound_start = last_val;
                last_val = *y_val;
                continue;
            }

            // We found the first edge to start a bound
            if bound_start == i32::MIN {
                bound_start = *y_val;
                last_val = *y_val;
                continue;
            }

            // We found the closing edge for this bound
            last_val = *y_val;
            column_bounds.insert((bound_start, last_val));
            bound_start = i32::MIN;
        }
        if bound_start != i32::MIN {
            column_bounds.insert((bound_start, last_val));
        }
        bounds.insert(*column, column_bounds);
    }
    println!("Joined bounds: {:?}", bounds);
    return bounds;
}

fn check_if_bound(
    coordinate: &Coordinate,
    column_bounds: &BTreeMap<i32, BTreeSet<(i32, i32)>>,
) -> bool {
    let this_columns_bounds = column_bounds.get(&coordinate.0).expect("Invalid x value");
    for bound_range in this_columns_bounds {
        let lower_bound = bound_range.0;
        let upper_bound = bound_range.1;
        if coordinate.1 >= lower_bound && coordinate.1 <= upper_bound {
            return true;
        }
    }

    return false;
}
fn calculate_area(coordinate_1: Coordinate, coordinate_2: Coordinate) -> i64 {
    let x_length = ((coordinate_1.0 - coordinate_2.0).abs() + 1) as i64;
    let y_length = ((coordinate_1.1 - coordinate_2.1).abs() + 1) as i64;
    return x_length * y_length;
}
