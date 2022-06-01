#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

type Sue = HashMap<&'static str, usize>;
lazy_static! {
    static ref SUE_REGEX: Regex = Regex::new(r"Sue (?P<id>\d+): (?P<item_1>\w+): (?P<amount_1>\d+), (?P<item_2>\w+): (?P<amount_2>\d+), (?P<item_3>\w+): (?P<amount_3>\w+)").unwrap();
}

fn is_match(tocheck: &Sue, reference: &Sue) -> bool {
    tocheck.iter().all(|(&k, &v)| v == reference[k])
}

fn is_match_2(tocheck: &Sue, reference: &Sue) -> bool {
    tocheck.iter().all(|(&k, &v)| match k {
        "cats" | "trees" => v > reference[k],
        "pomeranians" | "goldfish" => v < reference[k],
        _ => v == reference[k],
    })
}

fn main() {
    let reference: Sue = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();

    let input: Vec<Sue> = include_str!("../input.txt")
        .lines()
        .map(|line| SUE_REGEX.captures(line).unwrap())
        .map(|captures| {
            let mut hashmap = HashMap::new();
            for i in 1..(captures.len() - 4) {
                hashmap.insert(
                    captures.name(&format!("item_{}", i)).unwrap().as_str(),
                    captures
                        .name(&format!("amount_{}", i))
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                );
            }
            hashmap
        })
        .collect();

    let p1 = input
        .iter()
        .position(|candidate| is_match(candidate, &reference))
        .unwrap()
        + 1;
    println!("Part 1: {}", p1);

    let p2 = input
        .iter()
        .position(|candidate| is_match_2(candidate, &reference))
        .unwrap()
        + 1;
    println!("Part 2: {}", p2);
}
