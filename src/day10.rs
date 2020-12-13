use itertools::izip;
use cached::proc_macro::cached;


#[aoc_generator(day10)]
fn parse_nums(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn get_diff(input: &[u64]) -> Vec<u64> {
    let mut s: Vec<u64> = input.iter().map(|i| *i).collect();
    s.sort();
    s.insert(0, 0);
    izip!(s.iter(), s[1..].iter()).map(|(&i, &j)| j-i).collect()
}

fn count_n(sequence: &[u64], n: u64) -> u64 {
    sequence.iter().fold(0, |acc, &i| if i == n { acc + 1 } else { acc })
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    let diff = get_diff(input);
    let ones = count_n(&diff, 1);
    let threes = count_n(&diff, 3) + 1;
    ones*threes
}

#[cached]
fn trib(n: usize) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => trib(n-1) + trib(n-2) + trib(n-3),
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let diff = get_diff(input);

    #[allow(unused_assignments)]
    let mut i = 0;
    let mut j = 0;
    let mut acc = 1;

    while j < diff.len() {
        // find next 1 or 2
        i = j;
        while diff.get(i) == Some(&3) { i+=1; }
        // find end of [1,2] cluster
        j = i;
        while diff.get(j) != Some(&3) && j < diff.len() { j+=1; }

        if !diff[i..j].contains(&2) {
            acc *= trib(j-i);
        } //else {
            //TODO: handle case of 2
            //not needed, there is no 2
        //}
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: [u64; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const TEST_INPUT2: [u64; 31] = [28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24,
                                    23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
                                    8, 17, 7, 9, 4, 2, 34, 10, 3];

    #[test]
    fn test_parser() {
        let input = "0\n15";
        assert_eq!(parse_nums(input), vec![0, 15]);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&TEST_INPUT1), 7*5);
        assert_eq!(solve_part1(&TEST_INPUT2), 22*10);
    }

    #[test]
    fn test_trib() {
        assert_eq!(trib(4), 7);
        assert_eq!(trib(11), 504);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&TEST_INPUT1), 8);
        assert_eq!(solve_part2(&TEST_INPUT2), 19208);
    }
}
