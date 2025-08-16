use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let equations = parse!(input);

    let ans = equations
        .iter()
        .filter(|(target, terms)| is_valid(*target, terms, false))
        .map(|(target, _)| target)
        .sum::<u64>();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let equations = parse!(input);

    let ans = equations
        .iter()
        .filter(|(target, terms)| is_valid(*target, terms, true))
        .map(|(target, _)| target)
        .sum::<u64>();

    Ok(ans.to_string())
}

fn is_valid(target: u64, terms: &[u64], third_op: bool) -> bool {
    fn rec(current: u64, target: u64, terms: &[u64], third_op: bool) -> bool {
        if terms.is_empty() {
            return current == target;
        }

        if current > target && !terms.contains(&0) {
            return false;
        }

        if third_op {
            let new_current = concat_nums(current, terms[0]);
            if rec(new_current, target, &terms[1..], third_op) {
                return true;
            }
        }

        if rec(current * terms[0], target, &terms[1..], third_op) {
            return true;
        }

        rec(current + terms[0], target, &terms[1..], third_op)
    }

    rec(terms[0], target, &terms[1..], third_op)
}

fn concat_nums(a: u64, b: u64) -> u64 {
    let shift = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10u64.pow(shift) + b
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
        let terms = separated_list1(space1, uint());
        let line = separated_pair(uint(), (tag(":"), space1), terms);
        let lines = many1(ws_line(line));

        ws_all_consuming(lines).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "3749")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "11387")
    }
}
