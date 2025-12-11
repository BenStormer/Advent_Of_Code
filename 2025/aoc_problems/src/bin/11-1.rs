use std::collections::HashMap;

use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Read in each line into hashmap of (input, [outputs])
 * 2. Find outputs of input: "you"
 * 3. Perform a depth-first search to explore the paths to "out"
 *    - Retain memo for preventing re-traversal of already-visited nodes
 * 4. Every time there is a split in path, increment counter by 1
 * 5. Return final count of possible paths
 */

fn main() {
    let mut devices: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines("./inputs/11.txt") {
        for line in lines.map_while(Result::ok) {
            let split_line: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
            let input = &split_line[0];
            let outputs: Vec<String> = split_line[1..]
                .iter()
                .flat_map(|s| s.split_whitespace())
                .map(|s| s.to_string())
                .collect();
            devices.insert(input.to_string(), outputs);
        }
    }

    let mut explored_paths: HashMap<String, i32> = HashMap::new();
    let path_count = explore_paths("you".to_string(), &devices, &mut explored_paths);
    println!("Final count: {path_count}");
}

fn explore_paths(
    input: String,
    devices: &HashMap<String, Vec<String>>,
    explored_paths: &mut HashMap<String, i32>,
) -> i32 {
    if input == "out" {
        return 1;
    }
    // Check memo if we have already explored all paths
    if explored_paths.contains_key(&input) {
        return *explored_paths.get(&input).expect("Invalid value");
    }

    // Temporary add until we get the actual value
    explored_paths.insert(input.to_string(), 0);

    let empty_vector: &Vec<String> = &Vec::new();
    let paths_to_explore = devices.get(&input).unwrap_or(empty_vector);
    let path_count = paths_to_explore
        .iter()
        .map(|input| explore_paths(input.to_string(), devices, explored_paths))
        .sum();
    explored_paths.insert(input, path_count);

    return path_count;
}
