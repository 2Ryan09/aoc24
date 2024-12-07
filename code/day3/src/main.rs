use std::fs;
use std::path::Path;
use std::time::Instant;

use regex::Regex;

static INPUT_FILE: &str = "src/input/input.txt";

fn main() {
    let input_path = Path::new(INPUT_FILE);
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    // Track speed of execution, ignoring file read time
    let start_time = Instant::now();

    let mul_expression = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let digits_expression = Regex::new(r"\d{1,3}").unwrap();

    let mut total_mul_result: i32 = 0;
    let mut vector_match: Vec<&str> = Vec::new();
    for found in mul_expression.captures_iter(&contents) {
        let found_match = found.get(0).unwrap().as_str();

        let digits: Vec<_> = digits_expression
            .captures_iter(found_match)
            .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        total_mul_result += digits[0] * digits[1];

        vector_match.push(found_match);
    }

    println!(
        "Total mul result: {} in {} ns",
        total_mul_result,
        start_time.elapsed().as_micros()
    );

    // -- Part 2 --

    // Restart the timer
    let start_time = Instant::now();

    // Also find the "do()" and "don't()" expressions
    let mul_expression = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut total_mul_result: i32 = 0;
    let mut vector_match: Vec<&str> = Vec::new();
    let mut include_mul = true;
    for found in mul_expression.captures_iter(&contents) {
        let found_match = found.get(0).unwrap().as_str();

        // Let the presence of "do()" and "don't()" determine whether to include the "mul()" expressions
        if found_match == "do()" {
            include_mul = true;
        } else if found_match == "don't()" {
            include_mul = false;
        } else if found_match.starts_with("mul(") && include_mul {
            let digits: Vec<_> = digits_expression
                .captures_iter(found_match)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
                .collect();

            if digits.len() == 2 {
                total_mul_result += digits[0] * digits[1];
            }

            vector_match.push(found_match);
        }
    }

    println!(
        "Total mul result: {} in {} ns",
        total_mul_result,
        start_time.elapsed().as_micros()
    );
}
