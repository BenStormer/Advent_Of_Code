use std::collections::HashSet;

use aoc_problems::read_lines;
/*
 * Algorithm:
 * 1. Create a HashSet of beam_indices
 * 2. Read in input line by line and convert each line to a char vector
 * 3. The index of "S" marks the first beam_index pushed to beam_indices
 * 4. For each row, check the index of every "^" character. If one exists, then
 *    search the beam_indices set for this index
 *     - If it exists, create temporary hashset add i-1 and i+1 to the hashset
 *         - Temporary so we don't have issues with "^" next to eachother on same row
 *     - Increment running count
 */

fn main() {
    let mut count = 0;
    let mut beam_indices: HashSet<i32> = HashSet::new();
    let mut input_lines: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/7.txt") {
        for line in lines.map_while(Result::ok) {
            let line_char_array = line.chars().collect::<Vec<char>>();
            input_lines.push(line_char_array);
        }
    }

    let first_line = input_lines.remove(0);
    let beam_start = first_line
        .iter()
        .position(|&x| x == 'S')
        .expect("Invalid input") as i32;
    beam_indices.insert(beam_start);

    for input_line in input_lines {
        let splitter_indices: Vec<i32> = input_line
            .iter()
            .enumerate()
            .filter(|(_index, val)| **val == '^')
            .map(|(index, _val)| index as i32)
            .collect();

        // See if any beams hit the splitters in this line
        let mut temp_indices: Vec<i32> = Vec::new(); // Keep temp list to avoid two splitters in same line interfering
        for splitter_index in &splitter_indices {
            if beam_indices.contains(&splitter_index) {
                count += 1;
                beam_indices.remove(&splitter_index);
                temp_indices.push(splitter_index - 1);
                temp_indices.push(splitter_index + 1);
            }
        }

        for index in temp_indices.into_iter() {
            beam_indices.insert(index);
        }
    }
    println!("Final count: {}", count);
}
