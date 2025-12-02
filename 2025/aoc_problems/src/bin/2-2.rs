use aoc_problems::read_file;

/*
 * Algorithm:
 * 1. Break input into a list of each of the comma-separated id ranges
 * 2. Loop through range (inclusive of start/end values)
 * 3. Check for IDs with repeated digits
 *   - Loop over each way we can evenly split (% i == 0) into equal parts.
 *     - Ex: 2 equal parts, 3 equal parts, 4 equal parts
 *   - if repeat, then add and break
 *   - if not repeat, go into next split we can do
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
    // Check every way we can do an even split for repeated digits
    for n in 2..=id.len() {
        if id.len() % n != 0 {
            continue;
        }
        let id_chunks = get_id_chunks(id, n);

        // Check that all chunks are the same
        let first_chunk = &id_chunks[0];
        if id_chunks.iter().all(|chunk| chunk == first_chunk) {
            println!("\nID: {}, Chunks: {:?}", id, id_chunks);
            return true;
        }
    }
    return false;
}

// Split a string of length n into num_splits equal chunks
// Ex: "123456789" with num_splits=3 -> ["123", "456", "789"]
fn get_id_chunks(id: &str, num_splits: usize) -> Vec<&str> {
    let chunk_size = id.len() / num_splits;
    let mut chunks: Vec<&str> = Vec::new();
    for i in (0..id.len()).step_by(chunk_size) {
        chunks.push(&id[i..i + chunk_size]);
    }
    return chunks;
}
