use itertools::Itertools;

fn do_replace(molecule: String, (from, to): (&str, &str)) -> Vec<String> {
    molecule
        .match_indices(from)
        .fold(Vec::new(), |mut acc, (idx, _)| {
            acc.push(format!(
                "{}{}{}",
                &molecule[0..idx],
                to,
                &molecule[idx + from.len()..]
            ));
            acc
        })
}

fn calibrate(molecule: String, rules: Vec<(&str, &str)>) -> usize {
    rules
        .into_iter()
        .flat_map(|rule| do_replace(molecule.clone(), rule))
        .unique()
        .count()
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .lines()
        .filter(|&s| !s.is_empty())
        .collect();

    let idx_end = input.len() - 1;
    let (rules, molecule) = (
        input[..idx_end]
            .iter()
            .map(|&rule| rule.split_once(" => ").unwrap())
            .collect::<Vec<_>>(),
        input[idx_end].to_owned(),
    );

    println!("Part 1: {}", calibrate(molecule, rules))
}
