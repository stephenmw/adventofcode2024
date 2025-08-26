use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (towels, designs) = parse!(input);
    let ans = designs
        .iter()
        .filter(|d| num_designs(&towels, d) != 0)
        .count();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (towels, designs) = parse!(input);
    let ans: u64 = designs.iter().map(|d| num_designs(&towels, d)).sum();
    Ok(ans.to_string())
}

fn num_designs(dictionary: &[&str], word: &str) -> u64 {
    let mut paths = vec![0; word.len() + 1];
    paths[0] = 1;

    for i in 0..word.len() {
        let cur = paths[i];
        let candidates = dictionary.iter().filter(|&w| word[i..].starts_with(w));
        for c in candidates {
            paths[i + c.len()] += cur;
        }
    }

    *paths.last().unwrap()
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
        let word = || is_a("wubrg");
        let towels = separated_list1((space0, char(','), space0), word());
        let designs = many1(ws_line(word()));
        let parser = separated_pair(towels, multispace0, designs);
        ws_all_consuming(parser).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "6")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "16")
    }
}
