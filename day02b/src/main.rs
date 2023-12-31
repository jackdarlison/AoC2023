use regex::Regex;

pub fn main() {
    let colour_re = Regex::new(r"(?<num>[0-9]+) (?<col>(?:green|red|blue))").unwrap();
    println!("{}",
        include_str!("../input.txt")
            .split("\n")
            .fold(0, |acc: usize, s: &str| {
                let (r, g, b) = colour_re.captures_iter(s).fold((0, 0, 0), |(r, g, b), c| {
                    let num: usize = c["num"].parse::<usize>().unwrap();
                    match &c["col"] {
                        "red" if num > r => (num, g, b),
                        "green" if num > g => (r, num, b),
                        "blue" if num > b => (r, g, num),
                        _ => (r, g, b),
                    }
                });
                acc + (r * g * b)
            })
    )
}
