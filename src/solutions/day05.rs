use ahash::AHashSet;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (rules, updates) = parse!(input);

    let ruleset = RuleSet::new(&rules);

    let ans: u32 = updates
        .iter()
        .filter(|update| follows_rules(&ruleset, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(ans.to_string())
}

fn follows_rules(rules: &RuleSet, update: &[u32]) -> bool {
    update.is_sorted_by(|a, b| rules.partial_cmp(*a, *b).unwrap().is_lt())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (rules, updates) = parse!(input);

    let ruleset = RuleSet::new(&rules);

    fn reorder(rules: &RuleSet, update: &[u32]) -> Vec<u32> {
        let mut ret = update.to_vec();
        ret.sort_unstable_by(|a, b| rules.partial_cmp(*a, *b).unwrap());
        ret
    }

    let ans: u32 = updates
        .iter()
        .filter(|update| !follows_rules(&ruleset, update))
        .map(|update| reorder(&ruleset, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(ans.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    before: u32,
    after: u32,
}

struct RuleSet {
    rules: AHashSet<Rule>,
}

impl RuleSet {
    fn new(rule: &[Rule]) -> Self {
        Self {
            rules: AHashSet::from_iter(rule.iter().copied()),
        }
    }

    fn partial_cmp(&self, a: u32, b: u32) -> Option<std::cmp::Ordering> {
        if self.rules.contains(&Rule {
            before: a,
            after: b,
        }) {
            Some(std::cmp::Ordering::Less)
        } else if self.rules.contains(&Rule {
            before: b,
            after: a,
        }) {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Vec<u32>>)> {
        let rule = separated_pair(uint(), tag("|"), uint()).map(|(a, b)| Rule {
            before: a,
            after: b,
        });
        let rules = many1(ws_line(rule));

        let update = separated_list1((space0, tag(","), space0), uint());
        let updates = many1(ws_line(update));

        let parser = separated_pair(rules, multispace0, updates);

        ws_all_consuming(parser).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "143")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "123")
    }
}
