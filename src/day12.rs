#[derive(Debug, PartialEq, Clone)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

use Dir::*;

impl Dir {

    fn to_num(&self) -> i64 {
        match self { N => 0, E => 1, S => 2, W => 3 }
    }

    fn from_num(&self, i: i64) -> Dir {
        match i { 0 => N, 1 => E, 2 => S, 3 => W, _ => panic!("Invalid num for dir") }
    }

    fn rotated_by_num(&self, num: i64) -> Dir {
        self.from_num((self.to_num()+num)%4)
    }

    fn rotated(&self, angle: i64) -> Dir {
        match angle {
            0 => self.clone(),
            90 => self.rotated_by_num(1),
            180 => self.rotated_by_num(2),
            270 => self.rotated_by_num(3),
            360 => self.clone(),
            _ => panic!("Cannot rotate around odd angle")
        }
    }
}

// Ship with direction facing and coordinates (N, E) (negative means S/W)
pub struct Ship(Dir, i64, i64);

impl Ship {
    fn mov(&mut self, dir: Dir, len: i64) {
        match dir {
            N => { self.1 += len; },
            E => { self.2 += len; },
            S => { self.1 -= len; },
            W => { self.2 -= len; },
        }
    }

    fn turn(&mut self, angle: i64, right: bool) {
        if right {
            self.0 = self.0.rotated(angle);
        } else {
            self.0 = self.0.rotated(360-angle);
        }
    }

    fn rotate_origin(&mut self, angle: i64, right: bool) {
        match angle {
            0 => {},
            360 => {},
            90 => {
                if right {
                    let tmp = self.1;
                    self.1 = -self.2;
                    self.2 = tmp;
                } else {
                    let tmp = self.1;
                    self.1 = self.2;
                    self.2 = -tmp;
                }
            },
            180 => {
                self.1 = -self.1;
                self.2 = -self.2;
            },
            270 => {
                if right {
                    let tmp = self.1;
                    self.1 = self.2;
                    self.2 = -tmp;
                } else {
                    let tmp = self.1;
                    self.1 = -self.2;
                    self.2 = tmp;
                }
            },
            _ => panic!("Cannot rotate around odd angle")
        }
    }
}

#[aoc_generator(day12)]
fn parse_directions(input: &str) -> Vec<(char, i64)> {
    input.lines().map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap())).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[(char, i64)]) -> i64 {
    let mut s = Ship(E, 0, 0);
    for &(c, i) in input.into_iter() {
        match c {
            'N' => s.mov(N, i),
            'S' => s.mov(S, i),
            'E' => s.mov(E, i),
            'W' => s.mov(W, i),
            'F' => s.mov(s.0.clone(), i),
            'L' => s.turn(i, false),
            'R' => s.turn(i, true),
            _ => panic!("Invalid instruction"),
        }
    }
    s.1.abs() + s.2.abs()
}


#[aoc(day12, part2)]
pub fn solve_part2(input: &[(char, i64)]) -> i64 {
    let mut s = Ship(E, 0, 0);
    let mut wp = Ship(E, 1, 10);
    for &(c, i) in input.into_iter() {
        match c {
            'N' => wp.mov(N, i),
            'S' => wp.mov(S, i),
            'E' => wp.mov(E, i),
            'W' => wp.mov(W, i),
            'F' => { s.1 += i*wp.1; s.2 += i*wp.2; },
            'L' => wp.rotate_origin(i, false),
            'R' => wp.rotate_origin(i, true),
            _ => panic!("Invalid instruction"),
        }
    }
    s.1.abs() + s.2.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "F10\nN3\nF7\nR90\nF11";

    #[test]
    fn test_parser() {
        let input = "F10\nN3";
        assert_eq!(parse_directions(input), vec![('F', 10), ('N', 3)]);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_directions(TEST_INPUT)), 25);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&parse_directions(TEST_INPUT)), 286);
    }
}
