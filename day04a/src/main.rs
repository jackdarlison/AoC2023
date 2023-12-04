use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    println!("{}",
        include_str!("../input.txt")
        .lines()
        .fold(0, |acc, l| {
            let game = l.split_once(':').unwrap().1;
            let (winning, bet) = game.split_once("|").unwrap();
            let bets: Vec<_> = num_re.find_iter(bet).map(|m| m.as_str()).collect();
            acc + num_re.find_iter(winning).fold(0, |acc, num| {
                if bets.contains(&num.as_str()) {
                    if acc == 0 { 1 } else { acc * 2 }
                } else {
                    acc
                }
            })
        })
    )

}
