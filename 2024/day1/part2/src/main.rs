use std::collections::HashMap;

fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut left_numbers = Vec::new();
    let mut right_count = HashMap::new();

    lines.iter().for_each(|line| {
        if let Some((left, right)) = line.split_once("   ") {
            left_numbers.push(left.parse::<i64>().unwrap());
            let right_val = right.parse::<i64>().unwrap();
            right_count.entry(right_val).and_modify(|counter| *counter += 1).or_insert(1);
        } else {
            panic!("{line} could not be decomposed");
        }
    });
    
    let sum: i64 = left_numbers.iter().map(|left| {
        left * *right_count.get(&left).get_or_insert(&0i64)
    }).sum();
    
    println!("{sum}");
}