use std::iter::zip;

use itertools::Itertools;
use regex::Regex;


pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let (times, dists): (Vec<usize>, Vec<usize>) = include_str!("../input.txt").lines()
        .map(|s| num_re.find_iter(s).map(|n| n.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>())
        .collect_tuple().unwrap();

    println!("{}",
        zip(times.into_iter(), dists.into_iter())
            .map(|(t, d)| num_possible(t, d))
            .product::<usize>()
    );
}

fn num_possible(time: usize, dist: usize) -> usize {
    (1..time)
        .filter(|h| (time - h) * h > dist)
        .collect::<Vec<_>>()
        .len()
}
