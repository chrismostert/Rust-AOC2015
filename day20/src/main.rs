fn find_house(
    min_presents: usize,
    max_deliveries: Option<usize>,
    elf_factor: usize,
) -> Option<usize> {
    // Preallocate (bounded) space on the heap for calculation
    const BOUND: usize = 1000000;
    let mut houses = vec![0; BOUND];

    let deliveries = match max_deliveries {
        Some(val) => val,
        None => BOUND,
    };

    for elf in 1..BOUND {
        for house in (elf..BOUND).step_by(elf).take(deliveries) {
            houses[house] += elf * elf_factor;
        }
    }

    houses.into_iter().position(|house| house > min_presents)
}

fn main() {
    println!("Part 1: {:?}", find_house(34000000, None, 10).unwrap());
    println!("Part 2: {:?}", find_house(34000000, Some(50), 11).unwrap());
}
