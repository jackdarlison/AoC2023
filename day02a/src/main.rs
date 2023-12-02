use regex::Regex;


fn main() {
    let game_re = Regex::new(r"Game (?<game>[0-9]+): ").unwrap();
    let colour_re = Regex::new(r"(?<num>[0-9]+) (?<col>(?:green|red|blue))").unwrap();
    println!("{}",
        include_str!("../input.txt")
            .split("\n")
            .fold(0, |acc: usize, s: &str| {
                let game_caps = game_re.captures(s).unwrap();
                let impossible = colour_re.captures_iter(s).any(|c| {
                    let num: usize = c["num"].parse::<usize>().unwrap();
                    match &c["col"] {
                        "red" => num > 12,
                        "green" => num > 13,
                        "blue" => num > 14,
                        _ => panic!("Not a colour"),
                    }
                });
                if !impossible {
                    acc + game_caps["game"].parse::<usize>().unwrap()
                } else {
                    acc
                }
            })
    )
}
