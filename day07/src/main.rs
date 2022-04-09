use itertools::Itertools;
use std::collections::HashMap;

pub enum Instruction {
    Set(&'static str),
    Not(&'static str),
    And(&'static str, &'static str),
    Or(&'static str, &'static str),
    Lshift(&'static str, &'static str),
    Rshift(&'static str, &'static str),
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
    let mut exec_cache = |query| {
        if let Some(&v) = cache.get(query) {
            return v;
        };
        let res = execute_instruction(wires, query, cache);
        cache.insert(query, res);
        res
    };

    if let Ok(v) = query.parse::<u16>() {
        return v;
    }
    match wires[query] {
        Instruction::Set(a) => {
            if let Ok(v) = a.parse() {
                return v;
            }
            exec_cache(a)
        }
        Instruction::Not(a) => !(exec_cache(a)),
        Instruction::And(a, b) => exec_cache(a) & exec_cache(b),
        Instruction::Or(a, b) => exec_cache(a) | exec_cache(b),
        Instruction::Lshift(a, n) => exec_cache(a) << n.parse::<usize>().unwrap(),
        Instruction::Rshift(a, n) => exec_cache(a) >> n.parse::<usize>().unwrap(),
    }
}

fn get_final_value(wires: &HashMap<&str, Instruction>, query: &str) -> u16 {
    let mut cache: HashMap<&str, u16> = HashMap::new();
    execute_instruction(wires, query, &mut cache)
}

fn main() {
    let input = include_str!("../input.txt").lines().collect_vec();

    let mut wires =
        input
            .into_iter()
            .map(parse_instruction)
            .fold(HashMap::new(), |mut acc, (inst, dest)| {
                acc.insert(dest, inst);
                acc
            });

    println!("Part 1: {}", get_final_value(&wires, "a"));
    wires.insert("b", Instruction::Set("956"));
    println!("Part 2: {}", get_final_value(&wires, "a"));
}
