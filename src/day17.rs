use std::collections::HashSet;


#[derive(Debug, PartialEq, Hash, Clone)]
#[derive(Eq)]
pub struct Point(i32, i32, i32);


type Pocket = HashSet<Point>;


impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        let mut v = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    if i != 0 || j != 0 || k != 0 {
                        v.push(Point(self.0+i, self.1+j, self.2+k))
                    }
                }
            }
        }
        v
    }

    pub fn activate_neighbours(&self, state: &Pocket) -> usize {
        self.neighbours().iter().filter(|p| state.contains(p)).count()
    }
}


#[aoc_generator(day17)]
fn parse_init(input: &str) -> Pocket {
    let mut hs = HashSet::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => { hs.insert(Point(j as i32, i as i32, 0)); },
                '.' => {},
                _ => { panic!("Invalid input"); },
            }
        }
    }
    hs
}


fn step(state: &Pocket) -> Pocket {
    let mut activation_candidates: Pocket = HashSet::new();
    for active in state {
        for p in active.neighbours() {
            if !state.contains(&p) {
                activation_candidates.insert(p);
            }
        }
    }
    let mut new_state = HashSet::new();
    for candidate in activation_candidates {
        if candidate.activate_neighbours(state) == 3 {
            new_state.insert(candidate);
        }
    }
    for active in state {
        let num = active.activate_neighbours(state);
        if num == 2 || num == 3 {
            new_state.insert(active.clone());
        }
    }
    new_state
}


#[aoc(day17, part1)]
pub fn solve_part1(input: &Pocket) -> usize {
    let mut poc = input.clone();
    for _ in 0..6 {
        poc = step(&poc);
    }
    poc.len()
}


#[derive(Debug, PartialEq, Hash, Clone)]
#[derive(Eq)]
pub struct Point4(i32, i32, i32, i32);


type Pocket4 = HashSet<Point4>;


impl Point4 {
    pub fn neighbours(&self) -> Vec<Point4> {
        let mut v = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        if i != 0 || j != 0 || k != 0 || l != 0 {
                            v.push(Point4(self.0+i, self.1+j, self.2+k, self.3+l))
                        }
                    }
                }
            }
        }
        v
    }

    pub fn activate_neighbours(&self, state: &Pocket4) -> usize {
        self.neighbours().iter().filter(|p| state.contains(p)).count()
    }
}


fn step4(state: &Pocket4) -> Pocket4 {
    let mut activation_candidates: Pocket4 = HashSet::new();
    for active in state {
        for p in active.neighbours() {
            if !state.contains(&p) {
                activation_candidates.insert(p);
            }
        }
    }
    let mut new_state = HashSet::new();
    for candidate in activation_candidates {
        if candidate.activate_neighbours(state) == 3 {
            new_state.insert(candidate);
        }
    }
    for active in state {
        let num = active.activate_neighbours(state);
        if num == 2 || num == 3 {
            new_state.insert(active.clone());
        }
    }
    new_state
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Pocket) -> usize {
    let mut poc4 = input.iter().map(|p| Point4(p.0, p.1, p.2, 0)).collect();
    for _ in 0..6 {
        poc4 = step4(&poc4);
    }
    poc4.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = ".#.\n..#\n###";

    #[test]
    fn test_parser() {
        let expect: Pocket = [Point(1, 0, 0), Point(2, 1, 0), Point(0, 2, 0), Point(1, 2, 0), Point(2, 2, 0)].iter().cloned().collect();
        assert_eq!(parse_init(TEST_INPUT), expect);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_init(TEST_INPUT)), 112);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&parse_init(TEST_INPUT)), 848);
    }
}
