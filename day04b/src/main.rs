use std::collections::HashMap;

use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let mut games_multiplier: HashMap<usize, usize> = HashMap::new();
    println!("{}",
        include_str!("../input.txt")
        .lines()
        .fold(0, |acc, l| {
            let (game_title, game) = l.split_once(':').unwrap();
            let game_key = num_re.find(game_title).unwrap().as_str().parse::<usize>().unwrap();
            let (winning, bet) = game.split_once("|").unwrap();
            let bets: Vec<_> = num_re.find_iter(bet).map(|m| m.as_str()).collect();
            let score = num_re.find_iter(winning).fold(0, |acc, num| {
                if bets.contains(&num.as_str()) { acc + 1 } else { acc }
            });
            let current_multiplier: usize = *games_multiplier.get(&game_key).unwrap_or(&1);
            for i in (game_key + 1)..=(game_key + score) {
                games_multiplier.entry(i).and_modify(|e| { *e += current_multiplier}).or_insert(1 + current_multiplier);
            }
            acc + current_multiplier
        })
    )

}
