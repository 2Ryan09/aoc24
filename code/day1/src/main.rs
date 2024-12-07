use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

static INPUT_FILE: &str = "src/input/input.txt";

fn main() {
    let input_path = Path::new(INPUT_FILE);
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    // Track speed of execution, ignoring file read time
    let start_time = Instant::now();

    // Keep the columns separate to allow for easier sorting
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    // Ingest the data
    for line in contents.lines() {
        // Better way to do this? Feels computationally expensive
        let mut parts = line.split("   ");
        let num1: i32 = parts.next().unwrap().parse().expect("Invalid number");
        let num2: i32 = parts.next().unwrap().parse().expect("Invalid number");
        first_column.push(num1);
        second_column.push(num2);
    }

    // Sort since we care about minimums
    first_column.sort();
    second_column.sort();

    // This leads to speed improvements.. why is constructing a 2D vector
    // then using pairs faster than using the individual vectors?
    let mut secret_locations: Vec<(i32, i32)> = Vec::new();
    for i in 0..first_column.len() {
        secret_locations.push((first_column[i], second_column[i]));
    }

    // Calculate the total distance
    let mut total_distance: i32 = 0;
    for pair in &secret_locations {
        total_distance += (pair.0 - pair.1).abs();
    }

    // First star answer
    println!(
        "Calculated total distance: {} in {} ns",
        total_distance,
        start_time.elapsed().as_nanos()
    );

    // --- Part Two ---

    // Restart the timer
    let start_time = Instant::now();

    // Initialize the HashMap with all the numbers in the first column
    let mut match_occurrences: HashMap<i32, usize> = HashMap::new();
    for &(num1, _) in &secret_locations {
        match_occurrences.insert(num1, 0);
    }

    // Get the number of occurrences of each number in the second column
    for &(_, num2) in &secret_locations {
        if let Some(count) = match_occurrences.get_mut(&num2) {
            *count += 1;
        }
    }

    // Calculate the similarity score
    let mut similarity_score: i32 = 0;
    for (num1, count) in &match_occurrences {
        similarity_score += (*count as i32) * num1;
    }

    // Second star answer
    println!(
        "Calculated similarity score: {} in {} ns",
        similarity_score,
        start_time.elapsed().as_nanos()
    );
}
