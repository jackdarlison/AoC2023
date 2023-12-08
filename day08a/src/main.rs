use std::collections::HashMap;

use itertools::{Itertools, FoldWhile};
use regex::Regex;

pub fn main() {
    let mut file = include_str!("../input.txt").lines();

    let directions = file.next().unwrap().chars().map(|c| match c {
        'L' => Directions::Left,
        'R' => Directions::Right,
        _ => panic!("Invalid direction"),
    }).collect::<Vec<Directions>>();

    //skip empty line
    file.next();

    let graph_re = Regex::new(r"[A-Z]+").unwrap();

    let graph: HashMap<&str, (&str, &str)> = HashMap::from_iter(file.map(|l| {
        let (from, left, right) = graph_re.find_iter(l).map(|m| m.as_str()).collect_tuple().unwrap();
        (from, (left, right))
    }));

    let (_, steps) = directions.iter().cycle().fold_while(("AAA", 0), |(current, steps), dir| {
        let (left, right) = graph.get(current).unwrap();
        match dir {
            Directions::Left => {
                if left == &"ZZZ" {
                    return FoldWhile::Done((left, steps + 1));
                }
                FoldWhile::Continue((left, steps + 1))
            }
            Directions::Right => {
                if right == &"ZZZ" {
                    return FoldWhile::Done((right, steps + 1));
                }
                FoldWhile::Continue((right, steps + 1))
            }
        }
    }).into_inner();

    println!("{:?}", steps);
}

enum Directions {
    Left,
    Right,
}