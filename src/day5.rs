#[derive(Debug, PartialEq)]
pub struct Seat(u8, u8);

impl Seat {
    fn id(&self) -> u16 {
        ((self.0 as u16) << 3) + self.1 as u16
    }
}

#[aoc_generator(day5)]
fn parse_seats(input: &str) -> Vec<Seat> {
    input.lines().map(|l| Seat(
        u8::from_str_radix(&(l[0..7].chars().map(|c| match c {
            'F' => '0',
            'B' => '1',
            _ => panic!("Invalid input")
        }).collect::<String>()), 2).unwrap(),
        u8::from_str_radix(&(l[7..].chars().map(|c| match c {
            'L' => '0',
            'R' => '1',
            _ => panic!("Invalid input")
        }).collect::<String>()), 2).unwrap()
    )).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Seat]) -> u16 {
    input.iter().map(|s| s.id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Seat]) -> u16 {
    let mut ids = input.iter().map(|s| s.id()).collect::<Vec<u16>>();
    ids.sort();
    for (i, id) in ids[1..].iter().enumerate() {
        if ids[i] != id-1 {
            return id-1;
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
        assert_eq!(parse_seats(TEST_INPUT), vec![Seat(70, 7), Seat(14, 7), Seat(102, 4)]);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_seats(TEST_INPUT)), 820);
    }
}
