use aoc_problems::read_lines;

fn main() {
    let mut count = 0;
    let mut curr = 50;
    if let Ok(lines) = read_lines("./inputs/1.txt") {
        for line in lines.map_while(Result::ok) {
            if let Some(turn_value) = get_turn_value(&line) {
                println!(
                    "Current value: {}, Turn value: {}, Count: {}",
                    curr, turn_value, count
                );

                if turn_value > 0 {
                    count += (curr + turn_value) / 100;
                } else if turn_value < 0 {
                    let amount = -turn_value;

                    if curr == 0 {
                        // Edge case: If we start at 0, we don't count that
                        //            inital turn since we aren't "crossing" 0
                        count += amount / 100;
                    } else if amount >= curr {
                        // We first hit 0 after 'curr' steps, then every 100 steps after
                        count += 1 + (amount - curr) / 100;
                    }
                }

                // https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
                curr = (curr + turn_value).rem_euclid(100);
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
