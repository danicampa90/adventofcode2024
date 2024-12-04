use libutils::read_file_to_str;
use regex::{Match, Regex};

fn main() {
    let input = read_file_to_str("input.txt").unwrap();
    let mut input = parse_muls(&input);
    let result = add_up(&input);
    println!("Part 2: {:?}",result);

    // delete the do/don't to get back to part 1
    input.retain(|expr| *expr != Expr::Do && *expr!=Expr::Dont);
    let result = add_up(&input);
    println!("Part 1: {:?}",result)
}


#[derive(Debug, Eq, PartialEq)]
enum Expr {
    Mul(i32, i32),
    Do,
    Dont
}



fn parse_muls(input: &str) -> Vec<Expr> {
    let regex = Regex::new("(mul)\\(([0-9]+),([0-9]+)\\)|(do)\\(\\)|(don't)\\(\\)").unwrap();
    let mut result = Vec::new();
    for re_match in regex.captures_iter(input) {
        if Some("mul") == re_match.get(1).map(|m|m.as_str()) {
            let first_num : i32 = re_match.get(2).unwrap().as_str().parse().unwrap();
            let second_num : i32 = re_match.get(3).unwrap().as_str().parse().unwrap();
            result.push(Expr::Mul(first_num, second_num));
        }
        if Some("do")== re_match.get(4).map(|m|m.as_str()) {
            result.push(Expr::Do)
        }
        if Some("don't")== re_match.get(5).map(|m|m.as_str()) {
            result.push(Expr::Dont)
        }
    }
    result
}


fn add_up(input: &Vec<Expr>) -> i32 {
    let mut result = 0;
    let mut mul_enabled = true;
    for i in input{
        match i {
            Expr::Mul(a,b) if mul_enabled => result += a*b,
            Expr::Mul(_,_ ) => {}
            Expr::Do => mul_enabled = true,
            Expr::Dont => mul_enabled = false,
        }
    };
    result
}


#[cfg(test)]
mod tests{
    use crate::{add_up, parse_muls, Expr::{Do, Dont, Mul}};

    #[test]
    fn test_example() {
        let res = parse_muls("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(
            res,
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );
    }

    #[test]
    fn test_example2() {
        let res = parse_muls("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(
            res,
            vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)]
        );
        assert_eq!(add_up(&res), 48);
    }
    #[test]
    fn test_do_nothing_1() {
        let res = parse_muls("mul(4*, mul(6,9!, ?(12,34)");
        assert_eq!(
            res,
            vec![]
        );
    }

    #[test]
    fn test_do_nothing_2() {
        let res = parse_muls("mul ( 2 , 4 )");
        assert_eq!(
            res,
            vec![]
        );
    }
    
}