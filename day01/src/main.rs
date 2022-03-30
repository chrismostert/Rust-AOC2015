fn main() {
    let input = include_str!("../input.txt");
    let (final_floor, basement_step) =
        input
            .chars()
            .enumerate()
            .fold((0, 0), |(floor_res, step_res), (i, c)| {
                let next = match c {
                    '(' => floor_res + 1,
                    ')' => floor_res - 1,
                    _ => unreachable!(),
                };

                (
                    next,
                    if next < 0 && step_res == 0 {
                        i + 1
                    } else {
                        step_res
                    },
                )
            });

    println!("Part 1: {}", final_floor);
    println!("Part 2: {}", basement_step);
}
