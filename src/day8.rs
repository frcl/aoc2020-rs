#![allow(dead_code)]
//use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instr {
    op: Op,
    arg: i32,
}

impl Instr {
    fn new(op: Op, arg: i32) -> Instr {
        Instr { op: op, arg: arg }
    }

    fn acc(arg: i32) -> Instr {
        Instr { op: Op::Acc, arg: arg }
    }

    fn jmp(arg: i32) -> Instr {
        Instr { op: Op::Jmp, arg: arg }
    }

    fn nop(arg: i32) -> Instr {
        Instr { op: Op::Nop, arg: arg }
    }
}

struct Interp<'a> {
    pc: usize,
    acc: i32,
    code: &'a[Instr],
}

impl<'a> Interp<'a> {
    fn new(code: &[Instr]) -> Interp {
        Interp { pc: 0, acc: 0, code: code }
    }

    /// Run the instruction at the current value of pc.
    /// Returns a pair the values for pc and acc after the instruction.
    fn step(&mut self) -> (usize, i32) {
        let cur = self.code.get(self.pc).unwrap();
        match cur.op {
            Op::Acc => { self.acc += cur.arg; self.pc += 1; },
            Op::Jmp => { self.pc = (self.pc as i32 + cur.arg) as usize; },
            Op::Nop => { self.pc += 1; },
        }
        (self.pc, self.acc)
    }

    /// Run the code until it terminate or enters a loop.
    /// The first returned value is a bool indicating if the programm terminated regularly,
    /// the second is the accumulator before termination.
    fn run(&mut self) -> (bool, i32) {
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(0);
        loop {
            let (new_pc, new_acc) = self.step();
            if new_pc >= self.code.len() {
                return (true, new_acc);
            } else if visited.contains(&new_pc) {
                return (false, new_acc);
            } else {
                visited.insert(new_pc);
            }
        }
    }
}

#[aoc_generator(day8)]
fn parse_instr(input: &str) -> Vec<Instr> {
    input.lines().map(|l| {
        let mut parts = l.split_whitespace();
        Instr { op: match parts.next() {
            Some("acc") => Op::Acc,
            Some("jmp") => Op::Jmp,
            Some("nop") => Op::Nop,
            _ => panic!("unkown instruction"),
        }, arg: parts.next().unwrap().parse().unwrap() }
    }).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instr]) -> i32 {
    let mut interp = Interp::new(input);
    interp.run().1
}

struct Mutations<'a> {
    original: &'a[Instr],
    cursor: usize,
}

impl<'a> Mutations<'a> {
    fn new(code: &[Instr]) -> Mutations {
        Mutations{ original: code, cursor: 0 }
    }
}

impl<'a> Iterator for Mutations<'a> {
    //type Item = &'a[Instr];
    type Item = Vec<Instr>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(instr) = self.original.get(self.cursor) {
            match instr.op {
                Op::Acc => {
                    self.cursor += 1;
                    return self.next();
                },
                Op::Jmp => {
                    //let mut new = Vec::with_capacity(self.original.len());
                    let mut new = vec![Instr::nop(0); self.original.len()];
                    new.clone_from_slice(self.original);
                    new[self.cursor] = Instr::nop(instr.arg);
                    self.cursor += 1;
                    return Some(new);
                },
                Op::Nop => {
                    //let mut new = Vec::with_capacity(self.original.len());
                    let mut new = vec![Instr::nop(0); self.original.len()];
                    new.clone_from_slice(self.original);
                    new[self.cursor] = Instr::jmp(instr.arg);
                    self.cursor += 1;
                    return Some(new);
                },
            }
        }
        None
    }
}

trait Mutation {
    fn mutations(&self) -> Mutations;
}

impl Mutation for [Instr] {
    fn mutations(&self) -> Mutations {
        Mutations::new(self)
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instr]) -> i32 {
    for mutant in input.mutations() {
        let mut interp = Interp::new(&mutant);
        let (term, acc) = interp.run();
        if term {
            return acc;
        }
    }
    panic!("no terminating mutation found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

    #[test]
    fn test_parser() {
        let input = "nop +0\nacc +1";
        let expect = vec![Instr::new(Op::Nop, 0), Instr::new(Op::Acc, 1)];
        assert_eq!(parse_instr(&input), expect);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_instr(TEST_INPUT)), 5);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&parse_instr(TEST_INPUT)), 8);
    }
}
