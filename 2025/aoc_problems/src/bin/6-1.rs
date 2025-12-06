use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Read each line into a vector
 *    - Split on whitespace and trim to get each entry
 *    Store each vector inside a larger vector
 * 2. Pop first item from each vector and apply the operation
 * 3. Add answer to problem to running total
 * 4. Repeat until Vectors are empty
 */

fn main() {
    let mut count = 0;
    let mut problem_vectors: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/6.txt") {
        for line in lines.map_while(Result::ok) {
            let problem_strings = line.split_whitespace().map(|s| s.to_string()).collect();
            problem_vectors.push(problem_strings);
        }
    }
    let mut operations = problem_vectors.pop().expect("Read input wrong");

    for _ in 0..operations.len() {
        let mut numbers_to_add: Vec<i64> = Vec::new();
        for row in &mut problem_vectors {
            let number = row
                .pop()
                .expect("invalid string")
                .parse::<i64>()
                .expect("invalid number");
            numbers_to_add.push(number);
        }
        let operator = operations.pop().expect("invalid string");
        count += compute_math_problem(operator, &numbers_to_add);
    }
    println!("Final count: {}", count);
}

fn compute_math_problem(operator: String, numbers_to_add: &Vec<i64>) -> i64 {
    match operator.as_str() {
        "+" => numbers_to_add.iter().fold(0, |acc, &num| acc + num),
        "-" => numbers_to_add.iter().fold(0, |acc, &num| acc - num),
        "*" => numbers_to_add.iter().fold(1, |acc, &num| acc * num),
        _ => 0,
    }
}
