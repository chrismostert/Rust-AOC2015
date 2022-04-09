use cached::{cached_key, UnboundCache};
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

cached_key! {
    EXECUTE_INSTRUCTION: UnboundCache<String, u16> = UnboundCache::new();
    Key = { query.to_owned() };
    fn execute_instruction(wires: &HashMap<&str, Instruction>, query: &str) -> u16 = {
        if let Ok(v) = query.parse::<u16>() {
            return v;
        }
        match wires[query] {
            Instruction::Set(a) => {
                if let Ok(v) = a.parse() {
                    return v;
                }
                execute_instruction(wires, a)
            }
            Instruction::Not(a) => !(execute_instruction(wires, a)),
            Instruction::And(a, b) => execute_instruction(wires, a) & execute_instruction(wires, b),
            Instruction::Or(a, b) => execute_instruction(wires, a) | execute_instruction(wires, b),
            Instruction::Lshift(a, n) => execute_instruction(wires, a) << n.parse::<usize>().unwrap(),
            Instruction::Rshift(a, n) => execute_instruction(wires, a) >> n.parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt").lines().collect_vec();

    let wires =
        input
            .into_iter()
            .map(parse_instruction)
            .fold(HashMap::new(), |mut acc, (inst, dest)| {
                acc.insert(dest, inst);
                acc
            });

    println!("Part 1: {}", execute_instruction(&wires, "a"));
}
