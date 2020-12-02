use regex::Regex;

pub struct PassEntry {
    low: usize,
    high: usize,
    letter: char,
    password: String
}

impl PassEntry {
    pub fn new(low: usize, high: usize, letter: char, password: String) -> PassEntry {
        PassEntry{ low: low, high: high, letter: letter, password: password }
    }

    pub fn is_valid(&self) -> bool {
        let num = self.password.chars().into_iter().filter(|c| *c == self.letter).count();
        self.low <= num && num <= self.high
    }

    pub fn is_valid_new(&self) -> bool {
        let letters: Vec<char> = self.password.chars().collect();
        (letters[self.low-1] == self.letter) ^ (letters[self.high-1] == self.letter)
    }
}

#[aoc_generator(day2)]
fn parse_entries<'a>(input: &str) -> Vec<PassEntry> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)").unwrap();
    input.lines().map(|l| {
        let cap = re.captures(l).unwrap();
        PassEntry::new(cap.get(1).unwrap().as_str().parse().unwrap(),
                       cap.get(2).unwrap().as_str().parse().unwrap(),
                       cap.get(3).unwrap().as_str().parse().unwrap(),
                       cap.get(4).unwrap().as_str().to_string())
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[PassEntry]) -> u32 {
    let mut count = 0;
    for pass in input.iter() {
        if pass.is_valid() {
            count += 1;
        }
    }
    count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[PassEntry]) -> u32 {
    let mut count = 0;
    for pass in input.iter() {
        if pass.is_valid_new() {
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
        let test_pass1 = PassEntry::new(1, 3, 'a', "abcde".to_string());
        assert!(test_pass1.is_valid());
        let test_pass2 = PassEntry::new(1, 3, 'b', "cdefg".to_string());
        assert!(!test_pass2.is_valid());
    }

    #[test]
    fn test_is_valid_new() {
        let test_pass1 = PassEntry::new(1, 3, 'a', "abcde".to_string());
        assert!(test_pass1.is_valid_new());
        let test_pass2 = PassEntry::new(1, 3, 'b', "cdefg".to_string());
        assert!(!test_pass2.is_valid_new());
        let test_pass3 = PassEntry::new(2, 9, 'c', "ccccccccc".to_string());
        assert!(!test_pass3.is_valid_new());
    }

    #[test]
    fn test_part1_solver() {
        let test_passes = vec![PassEntry::new(1, 3, 'a', "abcde".to_string()),
                               PassEntry::new(1, 3, 'b', "cdefg".to_string()),
                               PassEntry::new(2, 9, 'c', "ccccccccc".to_string())];
        assert_eq!(solve_part1(&test_passes), 2);
    }

    //#[test]
    //fn test_part2_solver() {
    //}
}
