use std::collections::HashMap;

use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Read in each line into hashmap of (input, [outputs])
 * 2. Find outputs of input: "svr"
 * 3. Perform a depth-first search to explore the paths to "out"
 *    - Retain memo for preventing re-traversal of already-visited nodes
 *    - Also in this memo, store if we have already seen "dac" or "fft"
 *    - If we see cache hit with all matching values, return early with cached
 *      value
 *      - If the seen_dac or seen_fft values don't match cache, continue
 *        traversal
 * 4. If final value == "out" and we have seen "dac" and "fft", increment path
 *    counter
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

    // HashMap is {input: (seen_dac?, seen_fft?, path_counts)}
    let mut explored_paths: HashMap<String, (bool, bool, i64)> = HashMap::new();
    let path_count = explore_paths(
        "svr".to_string(),
        &devices,
        &mut explored_paths,
        false,
        false,
    );
    println!("Final count: {path_count}");
}

fn explore_paths(
    input: String,
    devices: &HashMap<String, Vec<String>>,
    explored_paths: &mut HashMap<String, (bool, bool, i64)>,
    mut seen_dac: bool,
    mut seen_fft: bool,
) -> i64 {
    if input == "out" {
        if seen_dac && seen_fft {
            return 1;
        } else {
            return 0;
        }
    } else if input == "dac" {
        seen_dac = true;
    } else if input == "fft" {
        seen_fft = true;
    }

    // Check memo if we have already explored all paths for this input
    if explored_paths.contains_key(&input) {
        let cache_entry = *explored_paths.get(&input).expect("Invalid value");

        // Only return early if all cache values match, otherwise this path is
        // technically unique
        if cache_entry.0 == seen_dac && cache_entry.1 == seen_fft {
            return cache_entry.2;
        }
    }

    // Sum path_counts from recursive DFS traversal
    let empty_vector: &Vec<String> = &Vec::new(); // Avoid borrowing error
    let paths_to_explore = devices.get(&input).unwrap_or(empty_vector);
    let path_count: i64 = paths_to_explore
        .iter()
        .map(|input| {
            explore_paths(
                input.to_string(),
                devices,
                explored_paths,
                seen_dac,
                seen_fft,
            )
        })
        .sum();

    // Update memo
    explored_paths.insert(input, (seen_dac, seen_fft, path_count));

    return path_count;
}
