use aoc_problems::read_file;

/*
 * Algorithm:
 * 1. Break input into a list of each of the comma-separated id ranges
 * 2. Loop through range (inclusive of start/end values)
 * 3. Check for IDs with repeated digits
 *   - Can skip all odd numbers
 *   - Split even numbers in half and compare the two strings
 *   - Add IDs that match to a running sum
 */

fn main() {
    let mut sum: i64 = 0;
    let file_contents = read_file("./inputs/2.txt").expect("Error reading file");
    let id_ranges = file_contents.split(',');
    for id_range in id_ranges {
        let split_id_range: Vec<&str> = id_range.split('-').collect();
        let starting_id = split_id_range[0]
            .trim()
            .parse::<i64>()
            .expect("Improper ID range");
        let ending_id = split_id_range[1]
            .trim()
            .parse::<i64>()
            .expect("Improper ID range");
        for id in starting_id..=ending_id {
            if repeated_digits(&id.to_string()) {
                sum += id;
                println!("Adding {} to sum. Sum: {}", id, sum);
            }
        }
    }
    println!("Final sum: {}", sum)
}

fn repeated_digits(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }
    let middle_index = id.len() / 2;
    let (first_half, second_half) = id.split_at(middle_index);
    if first_half == second_half {
        return true;
    }
    return false;
}
