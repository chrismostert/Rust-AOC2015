use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse(line: &str) -> (&str, &str, usize) {
    let (from, rest) = line.split_once(" to ").unwrap();
    let (to, dist) = rest.split_once(" = ").unwrap();
    (from, to, dist.parse().unwrap())
}

fn main() {
    let (cities, distances): (HashSet<&str>, HashMap<(&str, &str), usize>) =
        include_str!("../input.txt").lines().fold(
            (HashSet::new(), HashMap::new()),
            |(mut cities, mut distances), line| {
                let (from, to, dist) = parse(line);
                cities.extend([from, to]);
                distances.extend([((from, to), dist), ((to, from), dist)]);
                (cities, distances)
            },
        );

    let route_lengths = cities
        .iter()
        .permutations(cities.len())
        .map(|route| {
            route
                .iter()
                .tuple_windows()
                .map(|(&&from, &&to)| distances[&(from, to)])
                .sum::<usize>()
        })
        .collect_vec();

    println!("Part 1: {}", route_lengths.iter().min().unwrap());
    println!("Part 2: {}", route_lengths.iter().max().unwrap());
}
