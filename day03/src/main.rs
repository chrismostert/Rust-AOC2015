use std::collections::HashSet;

fn get_n_houses(instructions: &str, alternating: bool) -> usize {
    let mut houses = HashSet::from([(0, 0)]);
    let mut robo = false;
    let (mut x0, mut x1, mut y0, mut y1) = (0, 0, 0, 0);

    for c in instructions.chars() {
        let (x, y) = if robo && alternating {
            (&mut x1, &mut y1)
        } else {
            (&mut x0, &mut y0)
        };

        match c {
            '>' => *x += 1,
            '<' => *x -= 1,
            '^' => *y += 1,
            'v' => *y -= 1,
            _ => unreachable!(),
        }

        houses.insert((*x, *y));
        robo = !robo;
    }

    houses.len()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", get_n_houses(input, false));
    println!("Part 2: {}", get_n_houses(input, true));
}
