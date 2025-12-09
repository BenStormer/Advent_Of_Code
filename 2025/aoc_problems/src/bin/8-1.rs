use aoc_problems::read_lines;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};

/*
 * Algorithm:
 * 1. Read input line-by-line and store each line in a Vector of tuples
 * 2. Construct a graph as adjacency matrix where we connect coordinates with
 *    the distance between other coordinates as value
 *    - Write function for 3d distance (sqrt of each point's difference squared)
 * 3. Use adjacency matrix to create a BTree HashMap with <(coord1, coord2), distance>
 *    to sort them
 *    - Adjacency matrix step could be skipped, but I am assuming it will be
 *      useful for part 2
 * 4. Create a HashMap of groups (coordinate, groupNumber)
 *    - For each coord in the BTree HashMap, look up coordinates in the group HashMap.
 *       - If one of the coordinates is already in a group, add the other one to
 *         the same group
 *       - If neither coordinate is in a group, create a new group with them
 * 6. Repeat until all coordinates are sorted in groups
 * 7. Count the number of coordinates in each group and store in new BTree
 *    Set (since it is sorted)
 * 8. Multiply the # of coordinates in the 3 largest groups
 */

const NUM_CONNECTIONS: usize = 1000;
const TOP_N_MEMBER_COUNTS: usize = 3;

type Coordinate = (i64, i64, i64);

fn main() {
    let mut coordinates: Vec<(i64, i64, i64)> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/8.txt") {
        for line in lines.map_while(Result::ok) {
            let coordinate_vector: Vec<i64> = line
                .split(',')
                .map(|s| s.parse::<i64>().expect("Invalid input"))
                .collect();
            let coordinate = (
                coordinate_vector[0],
                coordinate_vector[1],
                coordinate_vector[2],
            );
            coordinates.push(coordinate);
        }
    }

    let mut adjacency_matrix: Vec<Vec<i64>> = vec![vec![0; coordinates.len()]; coordinates.len()];
    let mut distance_hashmap: BTreeMap<i64, (Coordinate, Coordinate)> = BTreeMap::new();
    for coordinate_1_index in 0..coordinates.len() {
        for coordinate_2_index in (coordinate_1_index + 1)..coordinates.len() {
            let coordinate_1 = coordinates[coordinate_1_index];
            let coordinate_2 = coordinates[coordinate_2_index];
            let distance = calculate_distance(coordinate_1, coordinate_2);

            // Construct adjacency matrix
            adjacency_matrix[coordinate_1_index][coordinate_2_index] = distance;

            // Construct sorted Hashmap (distance as key to sort by it)
            distance_hashmap.insert(distance, (coordinate_1, coordinate_2));
        }
    }

    // Calculate group pairings for all coordinates (coord: group_number)
    let mut group_hashmap: HashMap<Coordinate, i64> = HashMap::new();
    let mut group_count = 0;
    for (_distance, coordinates) in distance_hashmap.iter().take(NUM_CONNECTIONS) {
        let coordinate_1 = coordinates.0;
        let coordinate_2 = coordinates.1;

        // Check if coordinates are already in group_hashmap
        if group_hashmap.contains_key(&coordinate_1) && group_hashmap.contains_key(&coordinate_2) {
            let coordinate_1_group = group_hashmap
                .get(&coordinate_1)
                .expect("Invalid group number");
            let coordinate_2_group = group_hashmap
                .get(&coordinate_2)
                .expect("Invalid group number");
            if coordinate_1_group == coordinate_2_group {
                continue;
            } else {
                // Combine groups for group 1 and group 2
                combine_groups(*coordinate_1_group, *coordinate_2_group, &mut group_hashmap);
            }
        } else if group_hashmap.contains_key(&coordinate_1) {
            let coordinate_1_group = group_hashmap
                .get(&coordinate_1)
                .expect("Invalid group number");
            group_hashmap.insert(coordinate_2, *coordinate_1_group);
        } else if group_hashmap.contains_key(&coordinate_2) {
            let coordinate_2_group = group_hashmap
                .get(&coordinate_2)
                .expect("Invalid group number");
            group_hashmap.insert(coordinate_1, *coordinate_2_group);
        } else {
            group_count += 1;
            group_hashmap.insert(coordinate_1, group_count);
            group_hashmap.insert(coordinate_2, group_count);
        }
    }

    // Count the number of members for each group
    let mut group_member_counts: BTreeMap<Reverse<i64>, i64> = BTreeMap::new();
    for group_number in 1..=group_count {
        let member_count = group_hashmap
            .values()
            .filter(|&&val| val == group_number)
            .count() as i64;
        group_member_counts.insert(Reverse(member_count), group_number);
    }

    // Get top 3 member_counts and multiply
    let top_n_member_counts: Vec<&i64> = group_member_counts
        .iter()
        .take(TOP_N_MEMBER_COUNTS)
        .map(|(Reverse(member_count), _group_number)| member_count)
        .collect();
    println!("Top N Member Counts: {:?}", top_n_member_counts);

    let final_answer: i64 = top_n_member_counts.iter().map(|&&x| x).product();
    println!("Final Answer: {}", final_answer);
}

fn calculate_distance(coordinate_1: Coordinate, coordinate_2: Coordinate) -> i64 {
    let x_diff = coordinate_1.0 - coordinate_2.0;
    let y_diff = coordinate_1.1 - coordinate_2.1;
    let z_diff = coordinate_1.2 - coordinate_2.2;
    let distance = (x_diff * x_diff) + (y_diff * y_diff) + (z_diff * z_diff);

    // Convert the distance to an i64 so we can use it as a key in BTreeMap.
    // Rust does not implement the Ord trait (total ordering) for floats.
    // I'm just hoping that there are no values that end up the same in our AoC input.
    // To be truly thorough, we could use a crate like ordered_float to allow this
    // or create our own type to handle ordering floats
    return distance as i64;
}

fn combine_groups(group_1: i64, group_2: i64, group_hashmap: &mut HashMap<Coordinate, i64>) {
    for (_coordinate, group_number) in group_hashmap.iter_mut() {
        if *group_number == group_2 {
            *group_number = group_1;
        }
    }
}
