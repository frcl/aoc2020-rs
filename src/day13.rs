#[aoc_generator(day13)]
fn parse_notes(input: &str) -> (u64, Vec<Option<u64>>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap().parse().unwrap();
    (a, lines.next().unwrap().split(',').map(|s| match s {
        "x" => None,
        o => Some(o.parse().unwrap()),
    }).collect())
}


#[aoc(day13, part1)]
pub fn solve_part1(input: &(u64, Vec<Option<u64>>)) -> u64 {
    let wait = input.0-1;
    input.1.iter().filter(|oid| oid.is_some())
                  .map(|oid| {
                      let id = oid.unwrap();
                      (id, (wait/id+1)*id-wait-1)
                  })
                  .min_by_key(|t| t.1).map(|t| t.0 * t.1).unwrap()
}


#[aoc(day13, part2)]
pub fn solve_part2(input: &(u64, Vec<Option<u64>>)) -> u64 {
    #[allow(unused_variables)]
    let ids = &input.1;
    // all ids are prime
    // use chinese remainder theorem to get result
    // implement euclidean algorithm
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "939\n7,13,x,x,59,x,31,19";

    #[test]
    fn test_parser() {
        let input = "939\n7,x";
        assert_eq!(parse_notes(input), (939, vec![Some(7), None]));
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_notes(TEST_INPUT)), 295);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&parse_notes(TEST_INPUT)), 1068781);
        assert_eq!(solve_part2(&parse_notes("0\n17,x,13,19")), 3417);
        assert_eq!(solve_part2(&parse_notes("0\n67,7,59,61")), 754018);
        assert_eq!(solve_part2(&parse_notes("0\n67,x,7,59,61")), 779210);
        assert_eq!(solve_part2(&parse_notes("0\n67,7,x,59,61")), 1261476);
        assert_eq!(solve_part2(&parse_notes("0\n1789,37,47,1889")), 1202161486);
    }
}
