use aoc_problems::read_lines;

fn main() {
    if let Ok(lines) = read_lines("../inputs/1.txt") {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

/*
 * Plan:
 * 1. Keep counter of number of times we end on 0
 * 2. Keep tracker of current value
 * 3. Custom functions for L + R where wraparound is handled properly
 *   - L: subtract values and wraparound where 0 -> 99
 *   - R: add values and wraparound where 99 -> 0
 * 4. Return final count of times we end on 0
 */
