fn main() {
    let input = include_str!("../input.txt");
    let mut final_floor = 0;
    let mut basement_step = 0;

    for (i, c) in input.chars().enumerate() {
        final_floor += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };
        if final_floor < 0 && basement_step == 0 {
            basement_step = i + 1;
        };
    }

    println!("Part 1: {}", final_floor);
    println!("Part 2: {}", basement_step);
}
