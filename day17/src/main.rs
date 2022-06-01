fn solution(containers: &[isize], liters: isize) -> (usize, usize) {
    let mut solution_sizes: Vec<usize> = Vec::new();
    let mut min_solution_size: usize = usize::MAX;

    let p1 = get_combinations(
        containers,
        liters,
        0,
        &mut solution_sizes,
        &mut min_solution_size,
    );

    let p2 = solution_sizes
        .into_iter()
        .filter(|&n| n == min_solution_size)
        .count();

    (p1, p2)
}

fn get_combinations(
    containers: &[isize],
    liters_left: isize,
    n_used: usize,
    solutions: &mut Vec<usize>,
    min_used: &mut usize,
) -> usize {
    if liters_left == 0 {
        solutions.push(n_used);
        if n_used < *min_used {
            *min_used = n_used;
        }
        return 1;
    }
    if liters_left < 0 {
        return 0;
    }

    if !containers.is_empty() {
        return get_combinations(
            &containers[1..],
            liters_left - containers[0],
            n_used + 1,
            solutions,
            min_used,
        ) + get_combinations(&containers[1..], liters_left, n_used, solutions, min_used);
    }

    0
}

fn main() {
    let containers: Vec<isize> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let (p1, p2) = solution(&containers, 150);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
