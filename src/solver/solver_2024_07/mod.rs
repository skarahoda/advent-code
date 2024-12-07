mod input;

use regex::Regex;
use input::INPUT;

fn parse_inputs(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(|line| {
        let splits = line.split(": ").collect::<Vec<&str>>();
        (
            splits[0].parse().unwrap(),
            splits[1].split_whitespace().map(|s| s.parse().unwrap()).collect()
            )
    }).collect()
}

fn is_valid_equation(left: u64, right: &Vec<u64>, concat_operator: bool) -> bool {
    let mut right = right.clone();
    let last = right.pop().unwrap();
    if right.len() == 0 {
        return left == last;
    }
    let is_sum =  left > last && is_valid_equation(left - last, &right, concat_operator);
    if is_sum {
        return true;
    }
    let is_product = left % last == 0 && is_valid_equation(left / last, &right, concat_operator);
    if is_product {
        return true;
    }
    if !concat_operator {
        return false;
    }

    let re = Regex::new(format!(r"^(\d+){}$", last).as_str()).unwrap();
    let left = left.to_string();
    let captures = re.captures(&left);
    captures.is_some_and(|captures| {
        is_valid_equation(captures.get(1).unwrap().as_str().parse().unwrap(), &right, concat_operator)
    })
}


fn solve_first_part(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    equations.iter().filter(|(left, right)| is_valid_equation(*left, right, false)).fold(0, |acc, equation| {
        acc + equation.0
    })
}


fn solve_second_part(equations: &Vec<(u64, Vec<u64>)>) -> u64 {

    equations.iter().filter(|(left, right)| is_valid_equation(*left, right, true)).fold(0, |acc, equation| {
        acc + equation.0
    })
}

pub fn solve() -> (u64, u64) {
    let equations = parse_inputs(INPUT);
    (
        solve_first_part(&equations),
        solve_second_part(&equations),
    )
}

#[cfg(test)]
mod tests {
    static EXAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn is_valid_equation() {
        let equations = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::is_valid_equation(equations[0].0, &equations[0].1, false),
            true
        );
        assert_eq!(
            super::is_valid_equation(equations[1].0, &equations[1].1, false),
            true
        );
        assert_eq!(
            super::is_valid_equation(equations[2].0, &equations[2].1, false),
            false
        );
        assert_eq!(
            super::is_valid_equation(equations[3].0, &equations[3].1, false),
            false
        );
        assert_eq!(
            super::is_valid_equation(equations[4].0, &equations[4].1, false),
            false
        );
        assert_eq!(
            super::is_valid_equation(equations[5].0, &equations[5].1, false),
            false
        );
        assert_eq!(
            super::is_valid_equation(equations[6].0, &equations[6].1, false),
            false
        );
        assert_eq!(
            super::is_valid_equation(equations[7].0, &equations[7].1, false),
            false
        );
        assert_eq!(
            super::is_valid_equation(equations[8].0, &equations[8].1, false),
            true
        );
    }
    #[test]
    fn solve_first_part() {
        let equations = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::solve_first_part(&equations),
            3749
        );
    }

    #[test]
    fn solve_second_part() {
        let equations = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::solve_second_part(&equations),
            11387
        );
    }
}
