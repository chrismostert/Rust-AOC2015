use itertools::Itertools;
use std::collections::HashMap;

enum Instruction {
    Set(&'static str),
    Not(&'static str),
    And(&'static str, &'static str),
    Or(&'static str, &'static str),
    Lshift(&'static str, &'static str),
    Rshift(&'static str, &'static str),
    Setnum(u16),
}

fn parse_instruction(inst: &'static str) -> (Instruction, &str) {
    let (lhs, rhs) = inst.split_once(" -> ").unwrap();
    if let Some((a, op, b)) = lhs.split(' ').collect_tuple() {
        return match op {
            "AND" => (Instruction::And(a, b), rhs),
            "OR" => (Instruction::Or(a, b), rhs),
            "LSHIFT" => (Instruction::Lshift(a, b), rhs),
            "RSHIFT" => (Instruction::Rshift(a, b), rhs),
            _ => unreachable!(),
        };
    }
    if let Some((_, a)) = lhs.split(' ').collect_tuple() {
        return (Instruction::Not(a), rhs);
    }
    (Instruction::Set(lhs), rhs)
}

fn execute_instruction(
    wires: &HashMap<&str, Instruction>,
    query: &str,
    cache: &mut HashMap<&str, u16>,
) -> u16 {
    if let Ok(v) = query.parse::<u16>() {
        return v;
    }
    match wires[query] {
        Instruction::Set(a) => exec_cache(wires, a, cache),
        Instruction::Setnum(a) => a,
        Instruction::Not(a) => !(exec_cache(wires, a, cache)),
        Instruction::And(a, b) => exec_cache(wires, a, cache) & exec_cache(wires, b, cache),
        Instruction::Or(a, b) => exec_cache(wires, a, cache) | exec_cache(wires, b, cache),
        Instruction::Lshift(a, n) => exec_cache(wires, a, cache) << n.parse::<usize>().unwrap(),
        Instruction::Rshift(a, n) => exec_cache(wires, a, cache) >> n.parse::<usize>().unwrap(),
    }
}

fn exec_cache(
    wires: &HashMap<&str, Instruction>,
    query: &'static str,
    cache: &mut HashMap<&str, u16>,
) -> u16 {
    if let Some(&v) = cache.get(query) {
        return v;
    };
    let res = execute_instruction(wires, query, cache);
    cache.insert(query, res);
    res
}

fn find_value(wires: &HashMap<&str, Instruction>, query: &str) -> u16 {
    let mut cache: HashMap<&str, u16> = HashMap::new();
    execute_instruction(wires, query, &mut cache)
}

fn main() {
    let mut wires = include_str!("../input.txt")
        .lines()
        .map(parse_instruction)
        .fold(HashMap::new(), |mut acc, (inst, dest)| {
            acc.insert(dest, inst);
            acc
        });

    let p1 = find_value(&wires, "a");
    wires.insert("b", Instruction::Setnum(p1));
    let p2 = find_value(&wires, "a");

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
