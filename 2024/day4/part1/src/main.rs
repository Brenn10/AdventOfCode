use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRECTIONS: [(i64, i64); 8] = [
    (-1i64, -1i64),
    (-1i64, 0i64),
    (-1i64, 1i64),
    (0i64, -1i64),
    (0i64, 1i64),
    (1i64, -1i64),
    (1i64, 0i64),
    (1i64, 1i64)
];

fn read_file_to_char_array(path: &str) -> Vec::<Vec::<char>> {

    BufReader::new(File::open(path).expect("Failed to open file"))
        .lines()
        .map(|line| line.expect("Unable to read line").chars().collect())
        .collect()
}

fn get_next(curr: char) -> char {
    match curr {
        'X' => {'M'}
        'M' => {'A'}
        'A' => {'S'}
        _ => panic!("Invalid character")
    }
}

fn test(grid: &Vec::<Vec::<char>>, i: i64, j: i64, direction: (i64,i64)) -> i64 {
    let curr_char = grid[usize::try_from(i).unwrap()][usize::try_from(j).unwrap()];
    if curr_char == 'S' { return 1 }

    let new_i = match usize::try_from(i + direction.0) {
        Ok(number) => number,
        Err(e) => {return 0}
    };
    let new_j = match usize::try_from(j + direction.1) {
        Ok(number) => number,
        Err(e) => {return 0}
    };

    if new_i >= grid.len() || new_j >= grid[0].len() { return 0 }
    if grid[new_i][new_j] != get_next(curr_char) { return 0 }

    return test(grid, new_i as i64, new_j as i64, direction);
}

fn main() {
    let grid: Vec::<Vec::<char>> = read_file_to_char_array("src/input.txt");

    let mut sum = 0i64;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                sum += DIRECTIONS.into_iter().map(|direction| 
                    test(&grid, i.try_into().unwrap(), j.try_into().unwrap(), direction)
                ).sum::<i64>();
            }
        }
    }
    println!("Sum: {sum}");
}