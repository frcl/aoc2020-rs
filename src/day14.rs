use std::str::FromStr;
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct Mask {
    and: u64,
    or: u64,
}


impl Mask {
    fn apply_to(&self, num: u64) -> u64 {
        (num & self.and) | self.or
    }
}


impl FromStr for Mask {
    type Err = char;

    /// Parses bitmask from 'X01'-pattern, leading Xs can be skiped
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (and, or) = s.chars().rev().enumerate()
            .map(|(i, c)| match c {
                '0' => (1 << i, 0),
                '1' => (0, 1 << i),
                'X' => (0, 0),
                _ => { panic!("Invalid mask") }, //TODO: return Err(_) instead of panicing
            }).fold((0, 0), |(a, b), (c, d)| (a+c, b+d));
        Ok(Mask{ and: !and, or: or })
    }
}


#[derive(Debug, PartialEq)]
pub enum Instr {
    Mask(Mask),
    Assign((usize, u64)),
}


#[aoc_generator(day14)]
fn parse_prog(input: &str) -> Vec<Instr> {
    let mut instr = Vec::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            instr.push(Instr::Mask(line[7..].parse().unwrap()));
        } else if line.starts_with("mem") {
            let mut parts = line.split('=').map(|s| s.trim());
            instr.push(Instr::Assign(
                (parts.next().unwrap().split('[').last().unwrap().split(']').next().unwrap().parse().unwrap(),
                 parts.next().unwrap().parse().unwrap())
            ));
        } else {
            panic!("Invalid input");
        }
    }
    instr
}


#[aoc(day14, part1)]
pub fn solve_part1(input: &[Instr]) -> u64 {
    let mem_size = input.iter().filter_map(|i| match i {
        Instr::Mask(_) => None,
        Instr::Assign((s, _)) => Some(s),
    }).max().unwrap()+1;
    let mut mem = vec![0; mem_size];
    let mut mask: &Mask = &("".parse().unwrap());
    for instr in input.iter() {
        match instr {
            Instr::Mask(m) => { mask = m; },
            Instr::Assign((loc, val)) => { mem[*loc] = mask.apply_to(*val) },
        }
    }
    mem.iter().sum()
}


struct Floating {
    fixed: u64,
    floating: u64,
    count: u64,
}


impl Mask {
    /// Sets bits to one where mask is one,
    /// leaves bits where mask is zero untouched.
    /// Floating bit are set to zero.
    fn apply_to2(&self, num: u64) -> u64 {
        (num | ((0 & self.and) | self.or)) & (!self.and | self.or)
    }

    /// Returns iterator over all combinations alowed by floating bits.
    fn iter_floating(&self, addr: usize) -> Floating {
        // Only 36 bits
        let floating = (self.and & !self.or) & 0b111111111111111111111111111111111111;
        let fixed = self.apply_to2(addr as u64);
        Floating{ fixed: fixed, floating: floating, count: 0 }
    }
}


impl Floating {
    /// Returns number resulting from exchanging the floating bits in the mask
    /// with bits from num.
    fn expand(&self, num: u64) -> Option<u64> {
        let mut rfl = self.floating;
        let mut rfi = self.fixed;
        let mut rn = num;
        let mut res = 0;
        let mut i = 0;
        while rfl != 0 || rfi != 0 {
            if (rfl & 1) == 1 {
                res |= (rn & 1) << i;
                rn >>= 1;
            } else {
                res |= (rfi & 1) << i;
            }
            rfl >>= 1;
            rfi >>= 1;
            i += 1;
        }
        if rn != 0 {
            None
        } else {
            Some(res)
        }
    }
}


impl Iterator for Floating {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.expand(self.count);
        self.count += 1;
        tmp
    }
}


#[aoc(day14, part2)]
pub fn solve_part2(input: &[Instr]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask: &Mask = &("".parse().unwrap());
    for instr in input.iter() {
        match instr {
            Instr::Mask(m) => { mask = m; },
            Instr::Assign((loc, val)) => {
                for l in mask.iter_floating(*loc) {
                    mem.insert(l, val);
                }
            }
        }
    }
    mem.iter().fold(0, |i, (_, &j)| i+j)
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    const TEST_INPUT2: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_mask() {
        let mask: Mask = "1XXXX0X".parse().unwrap();
        assert_eq!(mask.apply_to(11), 73);
        assert_eq!(mask.apply_to(101), 101);
        assert_eq!(mask.apply_to(0), 64);
    }

    #[test]
    fn test_parser() {
        let mask: Mask = "1XXXX0X".parse().unwrap();
        let expect = vec![Instr::Mask(mask),
                          Instr::Assign((8, 11)),
                          Instr::Assign((7, 101)),
                          Instr::Assign((8, 0))];
        assert_eq!(parse_prog(TEST_INPUT), expect);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_prog(TEST_INPUT)), 165);
    }

    #[test]
    fn test_mask2() {
        let mask1: Mask = "000000000000000000000000000000X1001X".parse().unwrap();
        assert_eq!(mask1.apply_to2(0b101010), 0b011010);
        let mask2: Mask = "00000000000000000000000000000000X0XX".parse().unwrap();
        assert_eq!(mask2.apply_to2(0b11010), 0b10000);
    }

    #[test]
    fn test_to_floating() {
        let fl = "000000000000000000000000000000X1001X".parse::<Mask>().unwrap().iter_floating(0);
        assert_eq!(fl.floating, 0b100001);
        assert_eq!(fl.fixed, 0b010010);
        let fl = "00000000000000000000000000000000X0XX".parse::<Mask>().unwrap().iter_floating(0);
        assert_eq!(fl.floating, 0b1011);
        assert_eq!(fl.fixed, 0b0000);
    }

    #[test]
    fn test_expand() {
        let fl = "000000000000000000000000000000X1001X".parse::<Mask>().unwrap().iter_floating(0b000000);
        assert_eq!(fl.expand(0b00), Some(0b010010));
        assert_eq!(fl.expand(0b10), Some(0b110010));
    }

    #[test]
    fn test_floating_iteration() {
        let fl = "000000000000000000000000000000X1001X".parse::<Mask>().unwrap().iter_floating(0b000000);
        assert_eq!(fl.collect::<Vec<u64>>(), vec![
            0b010010,
            0b010011,
            0b110010,
            0b110011,
        ]);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&parse_prog(TEST_INPUT2)), 208);
    }
}
