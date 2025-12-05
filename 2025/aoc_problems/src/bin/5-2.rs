use aoc_problems::read_lines;
use std::collections::BTreeSet;

/*
 * Algorithm:
 * 1. Read in ID ranges as a BTreeSet Map<(start, end),...>
 *    - B-Tree Set should allow us to keep it in sorted order
 * 2. Combine overlapping ID ranges
 *    - For each ID range, check if the starting value <= last added ending value
 *       - If so, update range to have max(i[1], last_added[1])
 * 3. Go through all ID ranges and calculate i[1] - i[0] + 1 (since inclusive of range)
 *    and add to total count
 */

fn main() {
    let mut count = 0;
    let mut id_ranges: BTreeSet<(i64, i64)> = BTreeSet::new();

    if let Ok(lines) = read_lines("./inputs/5.txt") {
        // Map over all ID ranges
        for line in lines.map_while(Result::ok) {
            if line.contains('-') {
                id_ranges.insert(get_id_range_tuple(line));
            }
        }
    }

    // Combine overlapping ID ranges
    let mut combined_ranges: Vec<(i64, i64)> = Vec::new();
    combined_ranges.push(id_ranges.pop_first().expect("Empty ID Range list"));
    for id_range in id_ranges {
        // Get last value (highest starting value) of overlapping ID range list
        if let Some(last_id_range) = combined_ranges.last_mut() {
            let current_max = &last_id_range.1;
            if id_range.0 <= *current_max {
                // Update last item with new range
                last_id_range.1 = std::cmp::max(id_range.1, *current_max);
            } else {
                combined_ranges.push(id_range);
            }
        }
    }

    // Calculate IDs in all ranges
    for combined_id_range in combined_ranges {
        count += combined_id_range.1 - combined_id_range.0 + 1;
    }

    println!("Final count: {count}")
}

fn get_id_range_tuple(line: String) -> (i64, i64) {
    let split_id_range: Vec<&str> = line.split('-').collect();
    let starting_id = split_id_range[0]
        .trim()
        .parse::<i64>()
        .expect("Improper ID range");
    let ending_id = split_id_range[1]
        .trim()
        .parse::<i64>()
        .expect("Improper ID range");
    return (starting_id, ending_id);
}
