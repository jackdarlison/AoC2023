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

    let graph_re = Regex::new(r"[0-9A-Z]+").unwrap();

    let graph: HashMap<&str, (&str, &str)> = HashMap::from_iter(file.map(|l| {
        let (from, left, right) = graph_re.find_iter(l).map(|m| m.as_str()).collect_tuple().unwrap();
        (from, (left, right))
    }));

    let start_nodes = graph.keys().filter(|k| k.chars().last().unwrap() == 'A').map(|s| *s).collect::<Vec<_>>();

    let steps: Vec<usize> = start_nodes.iter()
        .map(|s| {directions.iter().cycle().fold_while((*s, 0), |(current, steps), dir| {
            let next_node = get_node(dir, *graph.get(current).unwrap());
            if next_node.chars().last().unwrap() == 'Z' {
                FoldWhile::Done((next_node, steps + 1))
            } else {
                FoldWhile::Continue((next_node, steps + 1))
            }
        }).into_inner()}).map(|(_, steps)| steps).collect();

    
    let result = steps.iter().fold(1, |acc, v| lcm(acc, *v));

    println!("{:?}", result);
}

fn get_node<'a>(dir: &Directions, node: (&'a str, &'a str)) -> &'a str {
    match dir {
        Directions::Left => node.0,
        Directions::Right => node.1,
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

enum Directions {
    Left,
    Right,
}

