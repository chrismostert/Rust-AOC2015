use fancy_regex::Regex;

fn n_nice_strings(input: &[&str]) -> (usize, usize) {
    // Part 1 rules
    let vowels = Regex::new("[aeiou]").unwrap();
    let doubles = Regex::new("([a-z])\\1").unwrap();
    let exclusions = Regex::new("ab|cd|pq|xy").unwrap();

    // Part 2 rules
    let twotwice = Regex::new("([a-z]{2}).*\\1").unwrap();
    let repeat_w_space = Regex::new("([a-z]).{1}\\1").unwrap();

    let p1 = input
        .iter()
        .filter(|s| is_nice(s, &vowels, &doubles, &exclusions))
        .count();

    let p2 = input
        .iter()
        .filter(|s| is_nicer(s, &twotwice, &repeat_w_space))
        .count();

    (p1, p2)
}

fn is_nice(tocheck: &str, v: &Regex, d: &Regex, e: &Regex) -> bool {
    v.captures_iter(tocheck).count() >= 3
        && d.captures_iter(tocheck).count() >= 1
        && e.captures_iter(tocheck).count() == 0
}

fn is_nicer(tocheck: &str, t: &Regex, r: &Regex) -> bool {
    t.captures_iter(tocheck).count() >= 1 && r.captures_iter(tocheck).count() >= 1
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let (p1, p2) = n_nice_strings(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
