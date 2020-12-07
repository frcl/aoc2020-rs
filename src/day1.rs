use itertools::iproduct;

#[aoc_generator(day1)]
pub fn parse_entries(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    iproduct!(input, input)
        .filter(|(&i, &j)| i+j == 2020)
        .map(|(&i, &j)| i*j)
        .next().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    iproduct!(input, input, input)
        .filter(|(&i, &j, &k)| i+j+k == 2020)
        .map(|(&i, &j, &k)| i*j*k)
        .next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let test_input = "1721\n979\n366\n299\n675\n1456\n";
        assert_eq!(parse_entries(test_input), vec![1721, 979, 366, 299, 675, 1456]);
    }

    #[test]
    fn test_part1_solver() {
        let test_input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_part1(&test_input), 514579);
    }

    #[test]
    fn test_part2_solver() {
        let test_input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_part2(&test_input), 241861950);
    }
}
