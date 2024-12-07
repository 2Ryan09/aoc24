use std::fs;
use std::path::Path;
use std::time::Instant;

static INPUT_FILE: &str = "src/input/input.txt";

fn main() {
    let input_path = Path::new(INPUT_FILE);
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    // Track speed of execution, ignoring file read time
    let start_time = Instant::now();

    // Ingest the data
    let mut parsed_lines: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid number"))
            .collect();
        parsed_lines.push(numbers);
    }

    // Validate the data
    let mut num_safe: usize = 0;
    for numbers in parsed_lines.iter() {
        let diffs: Vec<i32> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();

        let monotonic =
            (diffs.iter().all(|&diff| diff > 0)) || (diffs.iter().all(|&diff| diff < 0));
        let has_zero = diffs.contains(&0);
        let large_diff = diffs.iter().any(|&diff| diff.abs() > 3);

        if has_zero || !monotonic || large_diff {
            continue;
        } else {
            num_safe += 1;
        }
    }

    // First star answer
    println!(
        "Number of safe routes: {} in {} ns",
        num_safe,
        start_time.elapsed().as_nanos()
    );

    // --- Part Two ---

    // Restart the timer
    let start_time = Instant::now();

    // Validate the data
    // grr this is (mostly) a copy-paste of the above code
    let mut num_safe: usize = 0;
    for numbers in parsed_lines.iter() {
        let diffs: Vec<i32> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();

        let mut non_monotonic_count = 0;
        for window in diffs.windows(2) {
            if (window[0] > 0 && window[1] < 0) || (window[0] < 0 && window[1] > 0) {
                non_monotonic_count += 1;
            }
        }

        let zero_count = diffs.iter().filter(|&&diff| diff == 0).count();
        let num_large_diff = diffs.iter().filter(|&diff| diff.abs() > 3).count();

        if (zero_count + non_monotonic_count + num_large_diff) > 1 {
            continue;
        } else {
            num_safe += 1;
        }
    }

    // Second star answer
    println!(
        "Number of safe routes: {} in {} ns",
        num_safe,
        start_time.elapsed().as_nanos()
    );
}
