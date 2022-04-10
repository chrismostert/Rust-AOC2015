use itertools::Itertools;

fn look_say(input: &str) -> String {
    input.chars().group_by(|c| *c).into_iter().fold(
        String::with_capacity(2 * input.len()),
        |mut acc, (key, group)| {
            acc.push(char::from_digit(group.count() as u32, 10).unwrap());
            acc.push(key);
            acc
        },
    )
}

fn main() {
    let mut look_say_str = String::from("1113122113");
    for _ in 0..40 {
        look_say_str = look_say(&look_say_str)
    }
    println!("Part 1: {}", look_say_str.len());

    for _ in 0..10 {
        look_say_str = look_say(&look_say_str)
    }
    println!("Part 2: {}", look_say_str.len());
}
