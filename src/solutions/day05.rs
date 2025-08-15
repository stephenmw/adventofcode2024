use ahash::{AHashMap, AHashSet};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (rules, updates) = parse!(input);
    let after_rules = {
        let mut m: AHashMap<u32, AHashSet<u32>> = AHashMap::new();
        for rule in rules {
            m.entry(rule.before).or_default().insert(rule.after);
        }
        m
    };

    let ans: u32 = updates
        .iter()
        .filter(|update| follows_rules(&after_rules, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(ans.to_string())
}

fn follows_rules(rules: &AHashMap<u32, AHashSet<u32>>, update: &[u32]) -> bool {
    let mut seen = AHashSet::new();
    for &page in update {
        if rules
            .get(&page)
            .map(|after| !seen.is_disjoint(after))
            .unwrap_or(false)
        {
            return false;
        }

        seen.insert(page);
    }

    true
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (rules, updates) = parse!(input);
    let after_rules = {
        let mut m: AHashMap<u32, AHashSet<u32>> = AHashMap::new();
        for rule in &rules {
            m.entry(rule.before).or_default().insert(rule.after);
        }
        m
    };
    let before_rules = {
        let mut m: AHashMap<u32, AHashSet<u32>> = AHashMap::new();
        for rule in &rules {
            m.entry(rule.after).or_default().insert(rule.before);
        }
        m
    };

    fn reorder(rules: &AHashMap<u32, AHashSet<u32>>, update: &[u32]) -> Vec<u32> {
        let pages = AHashSet::from_iter(update.iter().copied());
        let mut relevant_rules: AHashMap<u32, AHashSet<u32>> =
            AHashMap::from_iter(update.iter().copied().map(|p| {
                (
                    p,
                    rules
                        .get(&p)
                        .map(|x| x.intersection(&pages).copied().collect())
                        .unwrap_or_default(),
                )
            }));

        // Remove leaves from relevant rules and add to froniter
        let mut frontier: Vec<u32> = relevant_rules
            .extract_if(|_, v| v.is_empty())
            .map(|(k, _)| k)
            .collect();

        let mut ret = Vec::new();

        while let Some(x) = frontier.pop() {
            ret.push(x);

            // remove current page and if the new page is ready add to frontier
            let new_leaves = relevant_rules
                .extract_if(|_, v| {
                    v.remove(&x);
                    v.is_empty()
                })
                .map(|(k, _)| k);
            frontier.extend(new_leaves);
        }

        ret
    }

    let ans: u32 = updates
        .iter()
        .filter(|update| !follows_rules(&after_rules, update))
        .map(|update| reorder(&before_rules, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(ans.to_string())
}

struct Rule {
    before: u32,
    after: u32,
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
