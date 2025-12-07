use aoc_problems::read_lines;
use std::cmp::Reverse;
use std::collections::BTreeSet;

/*
 * Algorithm:
 * 1. Read in ID ranges as a BTree Set<(start, end),...>
 *    - B-Tree Set should allow us to keep it in sorted order
 * 2. For each ID, look through B-Tree in reverse order and get the first
 *    entry where ID >= start
 * 3. Check if ID <= end, and if not go to the next entry
 *    - We still have to check the next entry since ID ranges can overlap.
 *      Ex: If ID = 20, then range 16-19 will not allow it, but 12-30 will
 * 4. If ingredient is fresh, increment count
 */

fn main() {
    let mut count = 0;
    let mut id_ranges: BTreeSet<Reverse<(i64, i64)>> = BTreeSet::new();
    let mut ids: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines("./inputs/5.txt") {
        // Map over all ID ranges
        for line in lines.map_while(Result::ok) {
            if line.contains('-') {
                // Id Range
                id_ranges.insert(Reverse(get_id_range_tuple(line)));
            } else if !line.trim().is_empty() {
                // Id
                ids.push(line.trim().parse::<i64>().expect("Improper ID"));
            }
        }
    }

    // Check if each ID is in a range
    for id in &ids {
        for id_range in &id_ranges {
            if id < &id_range.0.0 {
                continue;
            }

            // Id is >= start of range, <= end of range
            if id <= &id_range.0.1 {
                count += 1;
                break; // Don't double-count ids that fit in multiple ranges
            }
        }
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
