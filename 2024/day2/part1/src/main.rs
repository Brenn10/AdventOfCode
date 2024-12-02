
fn is_safe(line: &Vec<i64>) -> bool {
    let mut last = line[0];
    let mut last_diff = 0i64;
    for i in 1..line.len() {
        let current = line[i];
        let diff = current-last;
        if diff.abs() > 3 || diff == 0 || last_diff * diff < 0i64 {
            return false
        }
        last = current;
        last_diff = diff;
    }

    return true;
}

fn main() {
    let str_lines: Vec<&str> = include_str!("./input.txt").lines().collect();

    let lines: Vec<Vec<i64>> = str_lines.iter().map(|line| line.split(' ').map(|entry| entry.parse::<i64>().unwrap()).collect()).collect();

    let sum: i64 = lines.iter().map(|line| {
        if is_safe(line) {1} else {0}
    }).sum();
    
    println!("{sum}");
}