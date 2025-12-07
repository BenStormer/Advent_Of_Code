use aoc_problems::read_lines;

/*
 * Algorithm:
 * 1. Read each line into a vector
 * 2. Split, trim last line into operations vector
 * 3. For each other line, convert to char array
 * 4. Iterate by index through each char array at once to form cephalopod number
 * 5. Apply operation to the cephalopod numbers
 * 6. Repeat until Vectors are empty
 */

fn main() {
    let mut count = 0;
    let mut problem_vector_strings: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/6.txt") {
        for line in lines.map_while(Result::ok) {
            problem_vector_strings.push(line);
        }
    }
    let operations: Vec<String> = problem_vector_strings
        .pop()
        .expect("Read input wrong")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Convert each line of input into a char array
    let input_line_chars: Vec<Vec<char>> = problem_vector_strings
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // Get length of longest line of input
    let max_input_line_length = input_line_chars
        .iter()
        .map(|input_line| input_line.len())
        .max()
        .unwrap_or(0);

    let mut operator_index = 0; // Track what operator we should use
    let mut numbers_to_add: Vec<i64> = Vec::new(); // Track numbers in this problem
    for char_index in 0..max_input_line_length {
        let mut all_whitespace = true; // If only whitespace chars, move to new problem
        let mut number: String = String::new();

        for input_line in &input_line_chars {
            // Avoid out-of-bounds
            if char_index >= input_line.len() {
                continue;
            }

            let char_found = input_line[char_index];
            if char_found == ' ' {
                continue;
            } else {
                // Add digit to cephalopod number
                all_whitespace = false;
                number.push(char_found);
            }
        }

        // Track numbers we need to add, avoid edgecase of blank number (all whitespace chars)
        if !number.is_empty() {
            numbers_to_add.push(number.parse::<i64>().expect("Invalid number"));
        }

        // Compute the current problem, then move on to the next
        if all_whitespace {
            let operator = &operations[operator_index];
            operator_index += 1;

            count += compute_math_problem(operator, &numbers_to_add);
            numbers_to_add.clear(); // Reset our vector
        }
    }

    // Perform calculation on final set of numbers (all_whitespace flag not triggered)
    let operator = &operations[operator_index];
    count += compute_math_problem(operator, &numbers_to_add);

    println!("Final count: {}", count);
}

fn compute_math_problem(operator: &String, numbers_to_add: &Vec<i64>) -> i64 {
    println!("Performing {} on {:?}", operator, numbers_to_add);
    match operator.as_str() {
        "+" => numbers_to_add.iter().fold(0, |acc, &num| acc + num),
        "-" => numbers_to_add.iter().fold(0, |acc, &num| acc - num),
        "*" => numbers_to_add.iter().fold(1, |acc, &num| acc * num),
        _ => 0,
    }
}
