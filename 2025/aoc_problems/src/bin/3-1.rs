use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Break input up line by line
 * 2. We always need a two digit number, so we want to find the highest digit
 *    that isn't in the last spot
 * 3. Keep running "promotion" list of numbers:
 *      - Track the left digit and the right digit, "l" and "r".
 *      - Every time there is a new digit, check if "r" > "l".
 *      - Then, also check if "next" > "r" and shift accordingly
 *      Ex: 358092
 *          l = 3, r = 5, next = 8 (r > l and next > r)
 *          -> l = 5, r = 8, next = 0 (r > l)
 *          -> l = 8, r = 0, next = 9 (next > r)
 *          -> l = 8, r = 9, next = 2 (r > l)
 *          -> l = 9, r = 2
 * 4. Combine l + r into digit (Ex: 92)
 * 5. Add to running sum
 */

fn main() {
    let mut joltage_sum: i32 = 0;
    if let Ok(lines) = read_lines("./inputs/3.txt") {
        for line in lines.map_while(Result::ok) {
            let digits: Vec<char> = line.chars().collect();
            let mut l = digits[0];
            let mut r = digits[1];
            for i in 2..digits.len() {
                if r > l {
                    l = r;
                    r = digits[i];
                } else if digits[i] > r {
                    r = digits[i];
                }
            }
            let combined_digits = format!("{}{}", l, r);
            println!("Line: {}, Max Joltage: {}", line, combined_digits);
            joltage_sum += combined_digits
                .parse::<i32>()
                .expect("Error parsing digits");
        }
    }
    println!("Final joltage: {}", joltage_sum);
}
