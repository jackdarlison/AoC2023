use std::collections::HashMap;


pub fn main() {
    let mut games: Vec<(&str, usize)> = include_str!("../input.txt").lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(h, v)| (h, v.parse::<usize>().unwrap()))
        .collect();

    games.sort_by(|(a, _), (b, _)| {
        let ordering = hand_type(&a).cmp(&hand_type(&b));
        match ordering {
            std::cmp::Ordering::Equal => {
                if beats_other(&a, &b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            },
            _ => ordering,
        }   
    });

    println!("{:?}",
        games.iter().enumerate().fold(0, |acc, (i, (_, v))| {
            acc + v * (i + 1)
        })
    );
}

fn hand_type(s: &str) -> usize {
    let bags = s.chars().fold(HashMap::new(), |mut acc: HashMap<char, usize>, c| {
        acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
        acc 
    });
    let num_bags = bags.len();
    match num_bags {
        1 => 7, // 5 of a kind
        2 => {
            if bags.values().any(|&v| v == 4) {
                6 // 4 of a kind
            } else {
                5 // full house
            }
        },
        3 => {
            if bags.values().any(|&v| v == 3) {
                4 // 3 of a kind
            } else {
                3 // 2 pair
            }
        },
        4 => 2, // 1 pair
        5 => 1, // high card
        _ => panic!("Invalid number of bags"),
    }

}

fn beats_other(s: &str, other: &str) -> bool {
    let ranks: HashMap<char, usize> = HashMap::from_iter("23456789TJQKA".chars().zip(1..)); 
    for (c1, c2) in s.chars().zip(other.chars()) {
        if ranks.get(&c1) > ranks.get(&c2) {
            return true;
        }
        if ranks.get(&c1) < ranks.get(&c2) {
            return false;
        }
    }
    false
}
