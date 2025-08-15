use crate::solutions::prelude::*;

use ahash::AHashMap;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (mut list1, mut list2) = parse!(input);
    list1.sort();
    list2.sort();

    let pairs = list1.iter().copied().zip(list2.iter().copied());
    let ans: u32 = pairs.map(|(a, b)| a.abs_diff(b)).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (list1, list2) = parse!(input);
    let freq = freq_table(&list2);
    let ans: u32 = list1.iter().map(|x| x * freq.get(x).unwrap_or(&0)).sum();
    Ok(ans.to_string())
}

fn freq_table(list: &[u32]) -> AHashMap<u32, u32> {
    let mut ret = AHashMap::new();
    for &n in list {
        *ret.entry(n).or_insert(0) += 1;
    }
    ret
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
        let line = separated_pair(uint::<u32>(), space1, uint::<u32>());
        let lists = fold_many1(
            ws_line(line),
            || (Vec::new(), Vec::new()),
            |mut acc, (a, b)| {
                acc.0.push(a);
                acc.1.push(b);
                acc
            },
        );
        ws_all_consuming(lists).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "11")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "31")
    }
}
