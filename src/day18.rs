use std::str::FromStr;
use std::slice::Iter;


#[derive(Debug, PartialEq)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Val(u64),
}


use Expr::*;


impl Expr {
    fn eval(&self) -> u64 {
        match self {
            Add(a, b) => a.eval() + b.eval(),
            Mul(a, b) => a.eval() * b.eval(),
            Val(a) => *a,
        }
    }
}


#[derive(Debug)]
enum Token {
    Plus,
    Times,
    POpen,
    PClose,
    Digit(u64),
}


use Token::*;


fn tokenize(s: &str) -> Vec<Token> {
    s.chars().filter_map(|c| match c {
        '0'..='9' => Some(Digit(c.to_string().parse().unwrap())),
        '(' => Some(POpen),
        ')' => Some(PClose),
        '+' => Some(Plus),
        '*' => Some(Times),
        _ => None,
    }).collect()
}


fn expected(e: &str, u: Option<&Token>) -> Result<Expr, String> {
    match u {
        Some(f) => Err(format!("Expected {}, found {:?}", e, f)),
        None => Err(format!("Expected {}, found end of string", e)),
    }
}


fn consume_op(tokens: &mut Iter<Token>) -> Result<Expr, String> {
    match tokens.next() {
        Some(&Digit(d)) => Ok(Val(d)),
        Some(POpen) => {
            parse_multi_expr(tokens)
            //match tokens.next() {
                //Some(PClose) => Ok(expr),
                //u => expected("closed paren", u),
            //}
        },
        u => expected("operand", u),
    }
}


fn parse_expr(tokens: &mut Iter<Token>) -> Result<Expr, String> {
    let left = consume_op(tokens)?;
    match tokens.next() {
        Some(Plus) => Ok(Add(Box::new(left), Box::new(consume_op(tokens)?))),
        Some(Times) => Ok(Mul(Box::new(left), Box::new(consume_op(tokens)?))),
        Some(u) => expected("operator", Some(u)),
        None => Ok(left),
    }
}


fn parse_multi_expr(tokens: &mut Iter<Token>) -> Result<Expr, String> {
    let mut e = parse_expr(tokens)?;
    while let Some(t) = tokens.next() {
        match t {
            Plus => { e = Add(Box::new(e), Box::new(consume_op(tokens)?)); },
            Times => { e = Mul(Box::new(e), Box::new(consume_op(tokens)?)); },
            PClose => { return Ok(e); },
            u => { expected("operator", Some(u))?; },
        }
    }
    Ok(e)
}


impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(s);
        parse_multi_expr(&mut tokens.iter())
    }
}


#[aoc_generator(day18)]
fn parse_exprs(input: &str) -> Vec<Expr> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}


#[aoc(day18, part1)]
pub fn solve_part1(input: &[Expr]) -> u64 {
    input.iter().map(|e| e.eval()).sum()
}


//#[aoc(day18, part2)]
//pub fn solve_part2(input: &Pocket) -> usize {
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let expect1 = Add(
            Box::new(Mul(
                Box::new(Val(2)),
                Box::new(Val(3))
            )),
            Box::new(Mul(
                Box::new(Val(4)),
                Box::new(Val(5))
            ))
        );
        assert_eq!("2 * 3 + (4 * 5)".parse::<Expr>().unwrap(), expect1);
        let expect2 = Mul(
            Box::new(Add(
                Box::new(Val(9)),
                Box::new(Val(3)),
            )),
            Box::new(Val(4))
        );
        assert_eq!("9 + 3 * 4)".parse::<Expr>().unwrap(), expect2);
        assert_eq!(parse_exprs("1\n2"), vec![Val(1), Val(2)]);
    }

    #[test]
    fn test_eval() {
        assert_eq!("2 * 3 + (4 * 5)".parse::<Expr>().unwrap().eval(), 26);
        assert_eq!("5 + (8 * 3 + 9 + 3 * 4 * 3)".parse::<Expr>().unwrap().eval(), 437);
    }

    //#[test]
    //fn test_part1_solver() {
        //assert_eq!(solve_part1(&parse_init(TEST_INPUT)), 848);
    //}
}
