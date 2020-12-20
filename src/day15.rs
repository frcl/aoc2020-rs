use std::collections::HashMap;


#[aoc_generator(day15)]
fn parse_nums(input: &str) -> Vec<u64> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}


fn play(input: &[u64], until: u64) -> u64 {
    let mut nums: HashMap<u64, u64> = HashMap::new();
    let mut turn = 1;
    for n in input.iter() {
        nums.insert(*n, turn);
        turn += 1;
    }

    let mut cur = 0;
    let mut next = 0;
    while turn < until {
        if let Some(n) = nums.get(&cur) {
            next = turn - n;
        } else {
            next = 0;
        }
        nums.insert(cur, turn);
        turn += 1;
        cur = next;
    }
    next
}


#[aoc(day15, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    play(input, 2020)
}


#[aoc(day15, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    play(input, 30000000)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_nums("1,2,3"), vec![1, 2, 3]);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&[0, 3, 6]), 436);
        assert_eq!(solve_part1(&[1, 3, 2]), 1);
        assert_eq!(solve_part1(&[2, 1, 3]), 10);
        assert_eq!(solve_part1(&[1, 2, 3]), 27);
        assert_eq!(solve_part1(&[2, 3, 1]), 78);
        assert_eq!(solve_part1(&[3, 2, 1]), 438);
        assert_eq!(solve_part1(&[3, 1, 2]), 1836);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&[0, 3, 6]), 175594);
        assert_eq!(solve_part2(&[1, 3, 2]), 2578);
        assert_eq!(solve_part2(&[2, 1, 3]), 3544142);
        assert_eq!(solve_part2(&[1, 2, 3]), 261214);
        assert_eq!(solve_part2(&[2, 3, 1]), 6895259);
        assert_eq!(solve_part2(&[3, 2, 1]), 18);
        assert_eq!(solve_part2(&[3, 1, 2]), 362);
    }
}
