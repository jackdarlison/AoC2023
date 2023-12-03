
use std::{iter::zip, ops::RangeInclusive};

use regex::{Regex, Match};

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let file = include_str!("../input.txt");
    let mut padded_top: Vec<&str> = vec![""];
    padded_top.append(&mut file.lines().collect());
    let mut padded_bottom: Vec<&str> = file.lines().skip(1).collect();
    padded_bottom.push("");
    println!("{}",
        zip(zip(padded_top, file.lines()), padded_bottom).fold(0, |acc, ((top, middle), bottom)| {
            let gears: Vec<usize> = middle.match_indices(|c: char| c=='*').map(|(i, _)| i).collect();
            let mut numbers: Vec<(RangeInclusive<usize>, &str)> = vec![];
            numbers.append(&mut num_re.find_iter(top).map(|m| range_from_match(m)).collect::<Vec<(RangeInclusive<usize>, &str)>>());
            numbers.append(&mut num_re.find_iter(middle).map(|m| range_from_match(m)).collect::<Vec<(RangeInclusive<usize>, &str)>>());
            numbers.append(&mut num_re.find_iter(bottom).map(|m| range_from_match(m)).collect::<Vec<(RangeInclusive<usize>, &str)>>());
            let gear_numbers: Vec<_> = gears.iter().map(|i| {
                numbers.iter().filter(|(r, _)| r.contains(i)).map(|(_, v)| v.parse::<usize>().unwrap()).collect::<Vec<usize>>()
            }).collect();
            acc + gear_numbers.iter().map(|v| {
                if v.len() == 2 {
                    v.iter().product()
                } else {
                    0
                }
            }).sum::<usize>()
        })
    );
}

fn range_from_match(m: Match) -> (RangeInclusive<usize>, &str) {
    match m.start() {
        0 => (0..=m.end(), m.as_str()),
        _ => ((m.start()-1)..=m.end(), m.as_str()),
    }
}