use ahash::AHashMap;

use crate::solutions::prelude::*;
use crate::utils;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    solve(input, 25)
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    solve(input, 75)
}

fn solve(input: &str, blinks: u64) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let mut a = utils::freq_table(data);
    let mut b = AHashMap::new();

    for _ in 0..blinks {
        b.clear();
        count_step(&a, &mut b);
        std::mem::swap(&mut a, &mut b);
    }

    let ans: u64 = a.values().sum();

    Ok(ans.to_string())
}

fn count_step(i: &AHashMap<u64, u64>, o: &mut AHashMap<u64, u64>) {
    for (&k, &v) in i {
        if k == 0 {
            *o.entry(1).or_default() += v;
        } else {
            match split_num_if_even(k) {
                Some((a, b)) => {
                    *o.entry(a).or_default() += v;
                    *o.entry(b).or_default() += v;
                }
                None => *o.entry(k * 2024).or_default() += v,
            }
        }
    }
}

fn split_num_if_even(n: u64) -> Option<(u64, u64)> {
    let num_digits = n.checked_ilog10().unwrap_or(0) + 1;
    if num_digits % 2 == 0 {
        let factor = 10u64.pow(num_digits / 2);
        Some((n / factor, n % factor))
    } else {
        None
    }
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<u64>> {
        let nums = separated_list1(space1, uint());
        ws_all_consuming(nums).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "55312")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "65601038650482")
    }
}
