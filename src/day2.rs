use regex::Regex;

struct PassEntry<'a> {
    low: u32,
    high: u32,
    letter: u8,
    password: &'a str
}

impl<'a> PassEntry<'a> {
    pub fn new(low: u32, high: u32, letter: u8, password: &'a str) -> PassEntry {
        PassEntry{ low: low, high: high, letter: letter, password: password }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}

#[aoc_generator(day2)]
fn parse_entries<'a>(input: &'a str) -> Vec<PassEntry<'a>> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)").unwrap();
    input.lines().map(|l| {
        let cap = re.captures(l).unwrap();
        PassEntry::new(cap.get(0).unwrap().as_str().parse().unwrap(),
                       cap.get(1).unwrap().as_str().parse().unwrap(),
                       cap.get(2).unwrap().as_str().parse().unwrap(),
                       cap.get(3).unwrap().as_str())
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1<'a>(input: &[PassEntry<'a>]) -> u32 {
    let mut count = 0;
    for pass in input.iter() {
        if pass.is_valid() {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let test_pass1 = PassEntry::new(1, 3, 'a', "abcde");
        assert!(test_pass1.is_valid());
        let test_pass2 = PassEntry::new(1, 3, 'b', "cdefg");
        assert!(!test_pass2.is_valid());
    }

    #[test]
    fn test_part1_solver() {
        let test_input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc\n";
        assert_eq!(solve_part1(&test_input), 2);
    }

    //#[test]
    //fn test_part2_solver() {
    //}
}
