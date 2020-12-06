use itertools::{sorted, zip};


#[aoc_generator(day5)]
fn parse_seats(input: &str) -> Vec<u16> {
    input.lines().map(|l| u16::from_str_radix(&(l.chars().map(|c| match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => panic!("Invalid input")
        }).collect::<String>()), 2).unwrap()
    ).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u16]) -> u16 {
    let ids: Vec<&u16> = sorted(input).collect();
    for (&a, &b) in zip(ids.iter(), ids[1..].iter()) {
        if *a != b - 1 {
            return b -1
        }
    }
    panic!("No free seat found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";

    #[test]
    fn test_parser() {
        assert_eq!(parse_seats(TEST_INPUT), vec![567, 119, 820]);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_seats(TEST_INPUT)), 820);
    }
}
