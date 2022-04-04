struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn new(line: &str) -> Self {
        let mut sizes = line.split('x').map(|c| c.parse().expect("Not a number!"));
        Present {
            l: sizes.next().unwrap(),
            w: sizes.next().unwrap(),
            h: sizes.next().unwrap(),
        }
    }
}

fn get_paper_amount(p: &Present) -> usize {
    let res = [p.l * p.w, p.w * p.h, p.h * p.l]
        .iter()
        .fold((0, usize::MAX), |acc, &x| (acc.0 + 2 * x, x.min(acc.1)));
    res.0 + res.1
}

fn get_ribbon_amount(p: &Present) -> usize {
    let mut s = vec![p.l, p.w, p.h];
    s.sort_unstable();
    2 * s[0] + 2 * s[1] + s[0] * s[1] * s[2]
}
fn main() {
    let input: Vec<Present> = include_str!("../input.txt")
        .lines()
        .map(Present::new)
        .collect();

    let p1: usize = input.iter().map(get_paper_amount).sum();
    let p2: usize = input.iter().map(get_ribbon_amount).sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
