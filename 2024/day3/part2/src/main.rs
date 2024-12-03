use std::fs::read_to_string;
use regex::Regex;
fn sum_string(message: &str) -> i64 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let result: i64 = re.captures_iter(&message[..]).map(|caps| {
        let (_, [left, right]) = caps.extract();
        left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap()
    }).sum();

    return result;
}
fn main() {
    let message: String = read_to_string("src/input.txt").expect("Failed to read input");
    let result: i64 = message.split("do()").map(|do_line| {
        let left = match do_line.split_once("don't()") {
            None => { do_line }
            Some((left,_)) => {left}
        };
        return sum_string(left)
    }).sum();

    println!("{}", result);
}