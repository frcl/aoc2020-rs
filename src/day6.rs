use itertools::Itertools;


pub type Group = Vec<String>;

fn count_any(group: &Group) -> usize {
    group.concat().chars().unique().count()
}

fn count_all(group: &Group) -> usize {
    group.concat().chars().unique().filter(|&c|
        group.iter().all(|s| s.contains(c))
    ).count()
}

#[aoc_generator(day6)]
fn parse_groups(input: &str) -> Vec<Group> {
    input.split("\n\n").map(|g| g.lines().map(|s| s.to_string()).collect()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Group]) -> usize {
    input.iter().map(|g| count_any(g)).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Group]) -> usize {
    input.iter().map(|g| count_all(g)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn test_count_any() {
        let input = vec!["abc".to_string(), "ade".to_string()];
        assert_eq!(count_any(&input), 5);
    }

    #[test]
    fn test_count_all() {
        let input1 = vec!["abc".to_string(), "abe".to_string()];
        assert_eq!(count_all(&input1), 2);
        let input2 = vec!["bc".to_string(), "de".to_string()];
        assert_eq!(count_all(&input2), 0);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_groups(TEST_INPUT)), 11);
    }

    #[test]
    fn test_part2_solver() {
        assert_eq!(solve_part2(&parse_groups(TEST_INPUT)), 6);
    }
}
