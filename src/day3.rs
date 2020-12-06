#[aoc_generator(day3)]
fn parse_field(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect()
}

fn number_of_trees(field: &[Vec<bool>], slope: (usize, usize)) -> u64 {
    let w = field[0].len();
    let (vx, vy) = slope;
    (0..(field.len()/vy)).filter(|i| field[vy*i][(vx*i)%w]).count() as u64
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> u64 {
    number_of_trees(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> u64 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.into_iter().map(|s| number_of_trees(input, s)).product()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "..##.......\n#...#...#..\n.#....#..#.
..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#
.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn test_parser() {
        let input = ".#\n#.";
        assert_eq!(parse_field(input), vec![vec![false, true], vec![true, false]]);
    }

    #[test]
    fn test_part1_solver() {
        let input1 = vec![vec![false, true, false],
                         vec![true, true, false],
                         vec![false, true, true]];
        assert_eq!(solve_part1(&input1), 1);
        let input = vec![vec![false, true, false, false],
                         vec![true, true, false, true],
                         vec![true, true, true, false]];
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part1_solver_large() {
        let input = parse_field(TEST_INPUT);
        assert_eq!(solve_part1(&input), 7);
    }


    #[test]
    fn test_part2_solver_large() {
        let input = parse_field(TEST_INPUT);
        assert_eq!(solve_part2(&input), 336);
    }
}
