fn find_hashno(key: &str, n_zeroes: usize) -> Option<usize> {
    for i in 1.. {
        let d = format!("{:x}", md5::compute(format!("{}{}", key, i)));
        if d[..n_zeroes] == "0".repeat(n_zeroes) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let key = "iwrupvqb";

    let p1 = find_hashno(key, 5).expect("No solution found.");
    println!("Part 1: {}", p1);

    let p2 = find_hashno(key, 6).expect("No solution found.");
    println!("Part 2: {}", p2);
}
