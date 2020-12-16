#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Floor,
    Free,
    Occupied,
}

#[aoc_generator(day11)]
fn parse_map(input: &str) -> Vec<Vec<Cell>> {
    input.lines().map(|l| l.chars().map(|c| match c {
        '.' => Cell::Floor,
        'L' => Cell::Free,
        '#' => Cell::Occupied,
        _ => panic!("invalid input")
    }).collect()).collect()
}

const NEIGHBORS: [(usize, usize); 8] = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)];

fn checked_neighbor(map: &[Vec<Cell>], i: usize, j: usize, a: usize, b: usize) -> Option<&Cell> {
    map.get((i+a).checked_sub(1)?)?.get((j+b).checked_sub(1)?)
}

fn count_neighbors(map: &[Vec<Cell>], i: usize, j: usize) -> usize {
    NEIGHBORS.iter().filter(|&(a, b)| match checked_neighbor(map, i, j, *a, *b) {
        Some(c) => c == &Cell::Occupied,
        None => false,
    }).count()
}

fn step(old: &[Vec<Cell>], occ_thr: usize) -> Vec<Vec<Cell>> {
    old.iter().enumerate().map(|(i, v)| v.iter().enumerate().map(|(j, c)| match c {
        Cell::Floor => Cell::Floor,
        Cell::Free => if count_neighbors(old, i, j) == 0 { Cell::Occupied } else { Cell::Free },
        Cell::Occupied => if count_neighbors(old, i, j) >= occ_thr { Cell::Free } else { Cell::Occupied },
    }).collect()).collect()
}

fn count_occ(map: &[Vec<Cell>]) -> usize {
    map.iter().map(|v| v.iter().filter(|&c| *c == Cell::Occupied).count()).sum()
}


#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<Cell>]) -> usize {
    let mut old = input.to_vec();
    loop {
        let new = step(&old, 4);
        if new == old {
            return count_occ(&new);
        } else {
            old = new;
        }
    }
}

//#[aoc(day11, part2)]
//pub fn solve_part2(input: &[Vec<Cell>]) -> usize {
    //let mut old = input.to_vec();
    //loop {
        //let new = step(&old, 5);
        //if new == old {
            //return count_occ(&new);
        //} else {
            //old = new;
        //}
    //}
//}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_parser() {
        let input = "LL\n.#";
        assert_eq!(parse_map(input), vec![vec![Cell::Free, Cell::Free],
                                           vec![Cell::Floor, Cell::Occupied]]);
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_map(TEST_INPUT)), 37);
    }

    //#[test]
    //fn test_part2_solver() {
        //assert_eq!(solve_part2(&parse_map(TEST_INPUT)), 26);
    //}
}
