use std::iter::zip;

use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let file = include_str!("../input.txt");
    let mut padded_top: Vec<&str> = vec![""];
    padded_top.append(&mut file.lines().collect());
    let mut padded_bottom: Vec<&str> = file.lines().skip(1).collect();
    padded_bottom.push("");
    println!("{}",
        zip(zip(padded_top, file.lines()), padded_bottom).fold(0, |acc, ((top, middle), bottom)| {
            let mut symbols: Vec<usize> = Vec::<usize>::new();
            symbols.append(&mut top.match_indices(|c: char| c.is_ascii_punctuation() && !(c == '.')).map(|(i, _)| i).collect::<Vec<usize>>());
            symbols.append(&mut middle.match_indices(|c: char| c.is_ascii_punctuation() && !(c == '.')).map(|(i, _)| i).collect::<Vec<usize>>());
            symbols.append(&mut bottom.match_indices(|c: char| c.is_ascii_punctuation() && !(c == '.')).map(|(i, _)| i).collect::<Vec<usize>>());
            let numbers: Vec<_> = num_re.find_iter(middle).map(|m| {
                match m.start() {
                    0 => (0..=m.end(), m.as_str()),
                    _ => ((m.start()-1)..=m.end(), m.as_str()),
                }
            }).collect();
            let valid_numbers = numbers.iter().fold(vec![], |mut acc, (r, v)| {
                if symbols.iter().any(|i| r.contains(i)) {
                    acc.push(v.parse::<usize>().unwrap());
                }
                acc
            });
            acc + valid_numbers.iter().sum::<usize>()
        })
    );
}

