use std::str::Chars;
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum Rule {
    Char(char),
    Seq(Vec<Box<Rule>>),
    Any(Vec<Box<Rule>>),
}


use Rule::*;


impl Rule {
    fn _matches(&self, s: &mut Chars) -> bool {
        match self {
            Char(c) => s.next().map(|d| d == *c).unwrap_or(false),
            Seq(v) => v.iter().all(|r| r._matches(s)),
            Any(v) => v.iter().any(|r| {
                let mut new_s = s.clone();
                if r._matches(&mut new_s) {
                    r._matches(s)
                } else {
                    false
                }
            }),
        }
    }
    fn matches(&self, s: &str) -> bool {
        let mut c = s.chars();
        self._matches(&mut c) && c.next().is_none()
    }
}


fn parse_rule(rule: &str, map: &HashMap<u32, &str>) -> Rule {
    if rule.contains('|') {
        Any(rule.split('|').map(|r| {
            Box::new(parse_rule(r.trim(), map))
        }).collect())
    } else if rule.starts_with('"') && rule.ends_with('"') {
        Char(rule.trim_matches('"').chars().next().unwrap())
    } else {
        Seq(rule.split(' ').map(|id| {
            Box::new(parse_rule(map.get(&id.trim().parse().unwrap()).unwrap(), map))
        }).collect())
    }
}


#[aoc_generator(day19)]
fn parse_input(input: &str) -> (Rule, Vec<String>) {
    let mut parts = input.split("\n\n");
    let rule_map: HashMap<u32, &str> = parts
        .next().unwrap()
        .lines().map(|l| {
            let mut sp = l.split(':');
            (sp.next().unwrap().parse().unwrap(),
             sp.next().unwrap().trim())
        }).collect();
    (parse_rule(rule_map.get(&0).unwrap(), &rule_map),
     parts.next().unwrap().lines().map(|s| s.to_string()).collect())
}


#[aoc(day19, part1)]
pub fn solve_part1(input: &(Rule, Vec<String>)) -> usize {
    input.1.iter().filter(|s| input.0.matches(s)).count()
}


//#[aoc(day19, part2)]
//pub fn solve_part2(input: &Pocket) -> usize {
//}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn test_parser() {
        assert_eq!(parse_rule("\"a\"", &HashMap::new()), Char('a'));
    }

    #[test]
    fn test_chars_clone() {
        let mut c1 = "abc".chars();
        let mut c2 = c1.clone();
        assert_eq!(c1.next(), c2.next());
        assert_eq!(c1.next(), c2.next());
        assert_eq!(c1.next(), c2.next());
        assert_eq!(c1.next(), None);
    }

    #[test]
    fn test_matches() {
        let char_rule = parse_input("0: \"a\"\n\na").0;
        assert!(char_rule.matches("a"));
        assert!(!char_rule.matches("ab"));
        assert!(!char_rule.matches("b"));
        let seq_rule = parse_input("0: 1 2 1\n1: \"a\"\n2: \"b\"\n\na").0;
        assert!(seq_rule.matches("aba"));
        assert!(!seq_rule.matches("baa"));
        let any_rule = parse_input("0: 1 | 2\n1: \"a\"\n2: \"b\"\n\na").0;
        assert!(any_rule.matches("a"));
        assert!(any_rule.matches("b"));
        assert!(!any_rule.matches("aa"));
        let rule = parse_input(TEST_INPUT).0;
        assert!(rule.matches("ababbb"));
        assert!(!rule.matches("bababa"));
    }

    #[test]
    fn test_part1_solver() {
        assert_eq!(solve_part1(&parse_input(TEST_INPUT)), 2);
    }
}
