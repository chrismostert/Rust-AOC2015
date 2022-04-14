use itertools::Itertools;
use serde_scan::scan;
use std::collections::{HashMap, HashSet};
type Parsedline = (&'static str, &'static str, isize, &'static str);

fn solve(people: &HashSet<&'static str>, costs: &HashMap<(&str, &str), isize>) -> isize {
    let n_people = people.len();
    let neighbours = |i| {
        (
            if i == 0 { n_people - 1 } else { i - 1 },
            if i == n_people - 1 { 0 } else { i + 1 },
        )
    };
    people
        .iter()
        .permutations(n_people)
        .map(|solution| {
            solution
                .iter()
                .enumerate()
                .map(|(idx, &&p_cur)| {
                    let (idx_l, idx_r) = neighbours(idx);
                    let (&p_l, &p_r) = (solution[idx_l], solution[idx_r]);
                    costs[&(p_cur, p_l)] + costs[&(p_cur, p_r)]
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    let (mut people, mut costs) = include_str!("../input.txt")
        .lines()
        .map(|line| -> Parsedline {
            scan!("{} would {} {} happiness units by sitting next to {}." <- line).unwrap()
        })
        .fold(
            (HashSet::new(), HashMap::new()),
            |(mut people, mut costs), (from, kind, val, to)| {
                let val_to_insert = if kind == "lose" { -val } else { val };
                people.insert(from);
                costs.insert((from, to), val_to_insert);
                (people, costs)
            },
        );

    println!("Part 1: {}", solve(&people, &costs));

    for person in &people {
        costs.insert(("Me", person), 0);
        costs.insert((person, "Me"), 0);
    }
    people.insert("Me");

    println!("Part 2: {}", solve(&people, &costs));
}
