use std::collections::BinaryHeap;

fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    lines.iter().for_each(|line| {
        if let Some((left, right)) = line.split_once("   ") {
            left_heap.push(left.parse::<i32>().unwrap());
            right_heap.push(right.parse::<i32>().unwrap());
        } else {
            panic!("{line} could not be decomposed");
        }
    });
    
    let mut sum: i64 = 0;
    while !left_heap.is_empty() {
        let Some(left) = left_heap.pop() else {panic!("Cannot parse left_heap")};
        let Some(right) = right_heap.pop() else {panic!("Cannot parse right_heap")};
        sum += left.abs_diff(right) as i64;
    }
    println!("{sum}");
}