use aoc_problems::read_lines;
use std::collections::HashMap;

/*
 * Algorithm:
 * 1. Create a HashSet of beam_indices
 * 2. Read in input line by line and convert each line to a char vector
 * 3. The index of "S" marks the first beam_index pushed to beam_indices
 * 4. For each row, check the index of every "^" character. If one exists, then
 *    search the beam_indices set for this index
 *     - If it exists, we need to recursively follow both the i - 1 and the i + 1 path to completion
 *       - A particle can go left or right, so we need to do both and return add 1 for each path
 *       - Use memoization to store any state we have already computed. We can
 *         use a Hashmap to store ((row, beam_index), result)
 */

fn main() {
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

    // Use memoization to prevent excessively long runtime
    let mut memo: HashMap<(usize, i32), i64> = HashMap::new();
    let count = explore_path(0, beam_start, &input_lines, &mut memo);
    println!("Final count: {}", count);
}

fn explore_path(
    current_row: usize,
    beam_index: i32,
    input_lines: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, i32), i64>,
) -> i64 {
    if current_row >= input_lines.len() {
        return 1;
    }

    // Check memo for current state
    let key = (current_row, beam_index);
    if let Some(&cached_value) = memo.get(&key) {
        return cached_value;
    }

    let line = &input_lines[current_row];
    let splitter_indices: Vec<i32> = line
        .iter()
        .enumerate()
        .filter(|(_index, val)| **val == '^')
        .map(|(index, _val)| index as i32)
        .collect();

    // Check left and right paths recursively
    let result = if splitter_indices.contains(&beam_index) {
        explore_path(current_row + 1, beam_index - 1, input_lines, memo)
            + explore_path(current_row + 1, beam_index + 1, input_lines, memo)
    } else {
        explore_path(current_row + 1, beam_index, input_lines, memo)
    };

    // Add new state to the memo
    memo.insert(key, result);
    return result;
}
