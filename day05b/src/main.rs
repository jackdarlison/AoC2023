use itertools::Itertools;

use rangemap::RangeMap;
use regex::Regex;

pub fn main() {
    let num_re = Regex::new(r"[0-9]+").unwrap();
    let mut maps = include_str!("../input.txt").split("\n\n");
    let seed_values = num_re.find_iter(maps.next().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut seeds: Vec<usize> = vec![];
    // NOTE: this is obviously the naive solution, however, CPU needed exercise
    for (start, range) in seed_values.into_iter().tuples() {
        seeds.append(&mut (start..(start + range)).collect::<Vec<usize>>());
    }

    maps.for_each(|s| {
            let mut lines = s.split("\n");
            lines.next(); 
            let mut map = RangeMap::new();
            lines.for_each(|l| {
                let (dest, source, range) = num_re.find_iter(l)
                    .map(|m| m.as_str().parse::<usize>().unwrap())
                    .collect_tuple().unwrap();
                map.insert(source..(source + range), (dest, source));
            });
            seeds = seeds.iter().map(|s| {
                if let Some((dest, source)) = map.get(s) {
                    dest + (s - source) 
                } else {
                    *s
                }
            }).collect::<Vec<usize>>();
        });

    println!("{}", seeds.iter().min().unwrap());

}
