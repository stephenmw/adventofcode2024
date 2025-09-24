use ahash::{AHashMap, AHashSet};
use arrayvec::ArrayVec;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let seeds = parse!(input);
    let ans: u64 = seeds.iter().map(|&seed| nth_secret(seed, 2000)).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let seeds = parse!(input);

    let mut pattern_map = AHashMap::new();

    for seed in seeds {
        let mut price_diff = iter_prices_diff(seed, 2000);

        let mut pattern = (&mut price_diff)
            .take(3)
            .fold(Pattern::default(), |acc, (d, _)| acc.push(d).unwrap());

        let mut seen = AHashSet::new();
        for (d, p) in price_diff {
            pattern = pattern.push(d).unwrap();
            if seen.insert(pattern) {
                *pattern_map.entry(pattern).or_insert(0) += p as u64;
            }
        }
    }

    let ans = pattern_map.values().max().unwrap();
    Ok(ans.to_string())
}

// Takes the first n prices and provides a Vec of n-1 (diff, price) tuples
fn iter_prices_diff(seed: u64, n: usize) -> impl Iterator<Item = (i8, u8)> {
    let secrets = std::iter::successors(Some(seed), |&x| Some(next_secret(x))).take(n);
    let mut prices = secrets.map(|x| (x % 10) as u8);

    let initial_value = match next_chunk(&mut prices) {
        Ok([a, b]) => Some((b as i8 - a as i8, b)),
        Err(_) => None,
    };

    std::iter::successors(initial_value, move |&(_, prev)| {
        let p = prices.next()?;
        Some((p as i8 - prev as i8, p))
    })
}

fn nth_secret(seed: u64, n: usize) -> u64 {
    let mut res = seed;
    for _ in 0..n {
        res = next_secret(res);
    }

    res
}

fn next_secret(prev: u64) -> u64 {
    fn mix_and_prune(a: u64, b: u64) -> u64 {
        (a ^ b) % 16777216
    }

    let mut s = prev;

    s = mix_and_prune(s, s * 64);
    s = mix_and_prune(s, s / 32);
    s = mix_and_prune(s, s * 2048);

    s
}

fn next_chunk<const N: usize, T>(iter: impl Iterator<Item = T>) -> Result<[T; N], ArrayVec<T, N>> {
    let mut ret = ArrayVec::new();
    ret.extend(iter.take(N));
    ret.into_inner()
}

// A pattern is logically an array of 4 ints in the range [-9, 9]. The int is
// shifted up by 9 and then bit packed where every 6 bits is a new number.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Pattern(u32);

impl Pattern {
    // Removes the first int and adds the new int on the end
    fn push(&self, n: i8) -> Option<Self> {
        if !(n >= -9 && n <= 9) {
            return None;
        }

        let shifted_n = (n + 9) as u32;
        let ret = ((self.0 << 6) + shifted_n) & 0xFFFFFF;
        Some(Self(ret))
    }
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<u64>> {
        let parser = many1(ws_line(uint()));
        ws_all_consuming(parser).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_secret_test() {
        assert_eq!(next_secret(123), 15887950);
    }

    #[test]
    fn nth_secret_test() {
        assert_eq!(nth_secret(1, 2000), 8685429);
    }

    #[test]
    fn problem1_test() {
        const EXAMPLE_INPUT: &str = "1
        10
        100
        2024";

        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "37327623")
    }

    #[test]
    fn problem2_test() {
        const EXAMPLE_INPUT: &str = "1
        2
        3
        2024";

        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "23")
    }
}
