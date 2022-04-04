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

    fn paper_amount(&self) -> usize {
        let (area_sum, min_size) = [self.l * self.w, self.w * self.h, self.h * self.l]
            .iter()
            .fold((0, usize::MAX), |acc, &x| (acc.0 + 2 * x, x.min(acc.1)));
        area_sum + min_size
    }

    fn ribbon_amount(&self) -> usize {
        let mut s = [self.l, self.w, self.h];
        s.sort_unstable();
        2 * s[0] + 2 * s[1] + (s[0] * s[1] * s[2])
    }
}

fn main() {
    let input: Vec<Present> = include_str!("../input.txt")
        .lines()
        .map(Present::new)
        .collect();

    let p1: usize = input.iter().map(|x| x.paper_amount()).sum();
    let p2: usize = input.iter().map(|x| x.ribbon_amount()).sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
