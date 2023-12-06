use itertools::Itertools;
use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let (time, dist): (usize, usize) = include_str!("../input.txt").lines()
        .map(|s| s.replace(" ", ""))
        .map(|s| num_re.find(&s).unwrap().as_str().parse::<usize>().unwrap())
        .collect_tuple().unwrap();

    println!("{}",
        num_possible(time, dist)
    );
}

fn num_possible(time: usize, dist: usize) -> usize {
    (1..time)
        .filter(|h| (time - h) * h > dist)
        .collect::<Vec<_>>()
        .len()
}
