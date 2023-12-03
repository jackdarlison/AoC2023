use std::collections::HashMap;

use regex::Regex;

pub fn main() {
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    println!("{}",
    include_str!("../input.txt")
        .split("\n")
        .fold(0, |acc: i32, s: &str| {
            let mut matches: Vec<&str> = vec![];
            for i in 0..s.len() {
                if let Some(m) = re.find_at(s, i).map(|m| m.as_str()) {
                    matches.push(m)
                }
            }
            let left = *map.get(matches.first().unwrap()).unwrap_or(matches.first().unwrap());
            let right = *map.get(matches.last().unwrap()).unwrap_or(matches.last().unwrap());
            acc + String::from_iter([left, right]).parse::<i32>().unwrap()
        })
    )
}
