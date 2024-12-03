use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let message: String = read_to_string("src/input.txt").expect("Failed to read input");
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let result: i64 = re.captures_iter(&message[..]).map(|caps| {
        let (_, [left, right]) = caps.extract();
        left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap()
    }).sum();

    println!("{}", result);
}