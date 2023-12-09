use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"-?[0-9]+").unwrap();
    let lines = include_str!("../input.txt").lines()
        .map(|l| num_re.find_iter(l).map(|m| m.as_str().parse::<isize>().unwrap()).collect::<Vec<isize>>());

    println!("{:?}",
        lines.fold(0, |acc, v| acc + prev_value(v))
    );
}

fn prev_value(seq: Vec<isize>) -> isize {
    let mut firsts: Vec<isize> = vec![];
    let mut sub = seq.iter().zip(seq.iter().skip(1)).map(|(a, b)| b - a).collect::<Vec<isize>>();
    while sub.iter().any(|v| *v != 0) {
        firsts.push(*sub.first().unwrap());
        sub = sub.iter().zip(sub.iter().skip(1)).map(|(a, b)| b - a).collect::<Vec<isize>>();
    }

    seq.first().unwrap() - firsts.iter().rev().fold(0, |acc, v| v - acc)
}


