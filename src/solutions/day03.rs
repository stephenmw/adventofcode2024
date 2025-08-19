use regex::Regex;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parser::parse(input)?;
    let ans: u64 = data.iter().filter_map(|x| x.value()).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parser::parse(input)?;

    let mut enable = true;
    let mut ans: u64 = 0;

    for token in data {
        match token {
            Token::Mul(a, b) => {
                if enable {
                    ans += a * b;
                }
            }
            Token::Do => {
                enable = true;
            }
            Token::Dont => {
                enable = false;
            }
        }
    }

    Ok(ans.to_string())
}

enum Token {
    Mul(u64, u64),
    Do,
    Dont,
}

impl Token {
    fn value(&self) -> Option<u64> {
        match self {
            Token::Mul(a, b) => Some(a * b),
            _ => None,
        }
    }
}

mod parser {
    use super::*;

    pub fn parse(input: &str) -> Result<Vec<Token>, anyhow::Error> {
        let re = Regex::new(r"don't|do|mul\(([0-9]+),([0-9]+)\)").unwrap();
        let tokens = re
            .captures_iter(input)
            .map(|cap| match cap.get(1).zip(cap.get(2)) {
                Some((a, b)) => {
                    let a1 = a.as_str().parse::<u64>()?;
                    let b1 = b.as_str().parse::<u64>()?;
                    Ok(Token::Mul(a1, b1))
                }
                None => match cap.get(0).unwrap().as_str() {
                    "do" => Ok(Token::Do),
                    "don't" => Ok(Token::Dont),
                    _ => unreachable!(),
                },
            })
            .collect::<Result<Vec<Token>, anyhow::Error>>()?;

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_test() {
        const EXAMPLE_INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "161")
    }

    #[test]
    fn problem2_test() {
        const EXAMPLE_INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "48")
    }
}
