use itertools::iproduct;


#[aoc_generator(day9)]
fn parse_nums(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn validate_xmas(stream: &[u64], pre: usize) -> Option<u64> {
    for n in pre..stream.len() {
        let it1 = stream[n-pre..n].iter();
        let it2 = it1.clone();
        if let None = iproduct!(it1, it2).filter(|(&i, &j)| i+j == stream[n]).next() {
            return Some(stream[n]);
        }
    }
    None
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    validate_xmas(input, 25).unwrap()
}

fn find_range(stream: &[u64], max_width: usize, sum: u64) -> Option<u64> {
    for w in 2..max_width {
        for n in w..stream.len() {
            let range = &stream[n-w..n];
            if range.iter().sum::<u64>() == sum {
                return Some(range.iter().min().unwrap()+range.iter().max().unwrap());
            }
        }
    }
    None
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let num = solve_part1(input);
    find_range(input, 25, num).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [u64; 20] = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

    #[test]
    fn test_parser() {
        let input = "0\n15";
        assert_eq!(parse_nums(input), vec![0, 15]);
    }

    #[test]
    fn test_validate_xmas() {
        assert_eq!(validate_xmas(&TEST_INPUT, 5), Some(127));
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(find_range(&TEST_INPUT, 5, 127), Some(62));
    }
}
