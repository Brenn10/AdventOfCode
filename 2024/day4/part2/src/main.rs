use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file_to_char_array(path: &str) -> Vec::<Vec::<char>> {

    BufReader::new(File::open(path).expect("Failed to open file"))
        .lines()
        .map(|line| line.expect("Unable to read line").chars().collect())
        .collect()
}

fn test(grid: &Vec::<Vec::<char>>, i: usize, j: usize) -> bool {
    if grid[i][j] != 'A' { return false }

    let top = match usize::try_from(i64::try_from(i).unwrap() - 1) {
        Ok(number) => number,
        Err(_e) => {return false}
    };
    let left = match usize::try_from(i64::try_from(j).unwrap() - 1) {
        Ok(number) => number,
        Err(_e) => {return false}
    };
    let bot = i + 1;
    let right = j + 1;

    if bot >= grid.len() || right >= grid[0].len() { return false }
    
    let tl = grid[top][left];
    let tr = grid[top][right];
    let bl = grid[bot][left];
    let br = grid[bot][right];

    return ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
        && ((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M'));
}

fn main() {
    let grid: Vec::<Vec::<char>> = read_file_to_char_array("src/input.txt");

    let mut sum = 0i64;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            sum += if test(&grid, i, j) {1} else {0};
        }
    }
    println!("Sum: {sum}");
}