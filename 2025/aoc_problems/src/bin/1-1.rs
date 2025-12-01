use aoc_problems::read_lines;

fn main() {
    let mut count = 0;
    let mut curr = 50;
    if let Ok(lines) = read_lines("../inputs/1.txt") {
        for line in lines.map_while(Result::ok) {
            if let Some(turn_value) = get_turn_value(&line) {
                // https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
                curr = (curr + turn_value).rem_euclid(100); // wrap around 0-99 range,
                if curr == 0 {
                    count += 1;
                }
                println!("Turn value: {} Current value: {}", turn_value, curr);
            }
        }
    }
    println!("Final Count: {}", count)
}

fn get_turn_value(line: &str) -> Option<i32> {
    let mut chars = line.chars();
    match chars.next() {
        Some(direction) => {
            let magnitude: Option<i32> = chars.as_str().parse().ok();
            match direction {
                'L' => magnitude.map(|m| -1 * m),
                'R' => magnitude,
                _ => None,
            }
        }
        None => None,
    }
}
