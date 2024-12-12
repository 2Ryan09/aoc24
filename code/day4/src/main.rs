use std::fs;
use std::path::Path;

static INPUT_FILE: &str = "src/input/input.txt";

fn main() {
    let input_path = Path::new(INPUT_FILE);
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let content_grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS";
    let count = find_word_in_grid(&content_grid, word, false);

    // First star solution
    println!("Word '{}' found {} times", word, count);

    let word = "MAS";
    let count = find_word_in_grid(&content_grid, word, true);

    // Second star solution
    println!("X-MAS found {} times", count);
}

fn find_word_in_grid(grid: &Vec<Vec<char>>, word: &str, x_only: bool) -> usize {
    let cardinal_directions = vec![
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0)  // up
    ];

    let diagonal_directions = vec![
        (1, 1),  // down-right
        (1, -1), // down-left
        (-1, 1), // up-right
        (-1, -1) // up-left
    ];

    let directions: Vec<(isize, isize)> = if x_only {
        diagonal_directions
    } else {
        let mut all_directions = cardinal_directions;
        all_directions.extend(diagonal_directions);
        all_directions
    };

    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let mut idx_count = 0;
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(grid, word, i, j, dx, dy, word_len) {
                    idx_count += 1;
                }
            }

            if x_only && (idx_count == 2) {
                count += 1;
                idx_count = 0;

                continue;
            } else if x_only {
                idx_count = 0;
                continue;
            }

            count += idx_count;
            idx_count = 0;
        }
    }

    count
}

fn check_word(grid: &Vec<Vec<char>>, word: &str, x: usize, y: usize, dx: isize, dy: isize, word_len: usize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for k in 0..word_len {
        let nx = x as isize + k as isize * dx;
        let ny = y as isize + k as isize * dy;

        if nx < 0 || ny < 0 || nx >= rows || ny >= cols {
            return false;
        }

        if grid[nx as usize][ny as usize] != word.chars().nth(k).unwrap() {
            return false;
        }
    }

    true
}