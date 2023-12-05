use itertools::Itertools;

use rangemap::RangeMap;
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
