use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let regex_p1 = Regex::new(r#"(\\\\|\\"|\\x.{2})"#).unwrap();
    let regex_p2 = Regex::new(r#"(\\|")"#).unwrap();

    let mem_size =
        |inp| regex_p1.replace_all(inp, "").len() + regex_p1.captures_iter(inp).count() - 2;
    let encoded_size = |inp: &str| inp.len() + regex_p2.captures_iter(inp).count() + 2;

    let p1: usize = input.lines().map(|x| x.len() - mem_size(x)).sum();
    let p2: usize = input.lines().map(|x| encoded_size(x) - x.len()).sum();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
