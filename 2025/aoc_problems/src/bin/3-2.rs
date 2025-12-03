use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Break input up line by line
 * 2. We always need a two digit number, so we want to find the highest digit
 *    that isn't in the last spot
 * 3. Keep running "promotion" list of numbers:
 *      - Use an array to track the top 12 indeces
 *      - Every time there is a new digit, check if the rankings need to be shifted left
 *      - Then, also check if "next" > last digit in rankings
 * 4. Add to running sum
 */

const WINDOW_SIZE: usize = 12;

fn main() {
    let mut joltage_sum: i64 = 0;
    if let Ok(lines) = read_lines("./inputs/3.txt") {
        for line in lines.map_while(Result::ok) {
            let digits: Vec<char> = line.chars().collect();
            let mut digit_rankings: Vec<&char> = digits.iter().take(WINDOW_SIZE).collect();

            // Start at digit after first WINDOW_SIZE that are already in rankings
            for i in WINDOW_SIZE..digits.len() {
                // Do the current rankings need to be shifted left?
                for j in 0..(digit_rankings.len() - 1) {
                    if digit_rankings[j] < digit_rankings[j + 1] {
                        digit_rankings.drain(j..j + 1); // https://stackoverflow.com/questions/38227455/how-are-elements-of-a-vector-left-shifted-in-rust
                        digit_rankings.push(&digits[i]);
                        break; // Once shifted, move to next digit
                    }
                }

                // Check if new digit is larger than last digit in current rankings
                let last_index = digit_rankings.len() - 1;
                if &digits[i] > digit_rankings[last_index] {
                    digit_rankings[last_index] = &digits[i];
                }
            }
            let combined_digits: String = digit_rankings.into_iter().collect();
            println!("Line: {}, adding {}", line, combined_digits);
            joltage_sum += combined_digits
                .parse::<i64>()
                .expect("Error parsing digits");
        }
    }
    println!("Final joltage: {}", joltage_sum);
}
