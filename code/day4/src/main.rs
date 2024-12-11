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
    let count = find_word_in_grid(&content_grid, word);

    // First star solution
    println!("Word '{}' found {} times", word, count);
}

fn find_word_in_grid(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let directions = vec![
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
        (1, 1),  // down-right
        (1, -1), // down-left
        (-1, 1), // up-right
        (-1, -1) // up-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(grid, word, i, j, dx, dy, word_len) {
                    count += 1;
                }
            }
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