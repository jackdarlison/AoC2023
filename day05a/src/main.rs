use itertools::Itertools;

use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let mut maps = include_str!("../input.txt").split("\n\n");
    let mut seeds = num_re.find_iter(maps.next().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    maps.for_each(|s| {
            let mut lines = s.split("\n");
            lines.next(); 
            let mut new_seeds: Vec<usize> = vec![];
            let mut mapped_seeds: Vec<usize> = vec![];
            lines.for_each(|l| {
                let (dest, source, range) = num_re.find_iter(l)
                    .map(|m| m.as_str().parse::<usize>().unwrap())
                    .collect_tuple().unwrap();
                seeds.iter()
                    .filter(|v| v >= &&source && v < &&(source + range))
                    .for_each(|v| {
                        new_seeds.push(dest + (v - source));
                        mapped_seeds.push(*v);
                    });
            });
            new_seeds.append(&mut seeds.iter().filter(|v| !mapped_seeds.contains(v)).map(|v| *v).collect::<Vec<usize>>());
            seeds = new_seeds;
        });

    println!("{}", seeds.iter().min().unwrap());

}
