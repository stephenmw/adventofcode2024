use std::collections::BTreeSet;

use ahash::{AHashMap, AHashSet};

use crate::solutions::prelude::*;
use crate::utils::freq_table;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let nodes = {
        let mut nodes: AHashMap<NodeId, BTreeSet<NodeId>> = AHashMap::new();
        for (a, b) in data {
            nodes.entry(a).or_default().insert(b);
            nodes.entry(b).or_default().insert(a);
        }
        nodes
    };

    let mut groups = AHashSet::new();
    for (id1, conn1) in nodes.iter().filter(|(n, _)| n.0[0] == b't') {
        for id2 in conn1.iter() {
            let conn2 = nodes.get(id2).unwrap();
            for id3 in conn1.intersection(conn2) {
                let mut group = [id1, id2, id3];
                group.sort();
                groups.insert(group);
            }
        }
    }

    Ok(groups.len().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let nodes = {
        let mut nodes: AHashMap<NodeId, BTreeSet<NodeId>> = AHashMap::new();
        for (a, b) in data {
            nodes.entry(a).or_default().insert(b);
            nodes.entry(b).or_default().insert(a);
        }
        nodes
    };

    let ubs: Vec<_> = nodes.keys().map(|n| (n, upper_bound(n, &nodes))).collect();
    let max = ubs.iter().max_by_key(|(_, ub)| *ub).unwrap();
    let mut ret_nodes: Vec<_> = ubs
        .iter()
        .filter(|(_, ub)| *ub == max.1)
        .map(|(n, _)| n)
        .copied()
        .collect();
    ret_nodes.sort();

    let ans = ret_nodes
        .iter()
        .map(|n| format!("{:?}", n))
        .collect::<Vec<_>>()
        .join(",");

    Ok(ans)
}

fn upper_bound(node: &NodeId, nodes: &AHashMap<NodeId, BTreeSet<NodeId>>) -> u64 {
    let neighbors = nodes.get(node).unwrap();
    let neigh_in_common = neighbors
        .iter()
        .flat_map(|n| nodes.get(n).unwrap().intersection(neighbors));
    let freq = freq_table(neigh_in_common);

    let sorted_values = {
        let mut xs: Vec<_> = freq.values().cloned().collect();
        xs.sort_unstable();
        xs
    };

    n_gt_n(&sorted_values) + 2
}

// Takes a sorted list and finds the highest n such that there are n numbers
// greater than n.
fn n_gt_n(values: &[u64]) -> u64 {
    values
        .iter()
        .rev()
        .enumerate()
        .take_while(|&(i, &x)| (i + 1) as u64 <= x)
        .count() as u64
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NodeId([u8; 2]);

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<(NodeId, NodeId)>> {
        let nodeid = || {
            verify(take(2usize), |x: &str| {
                x.chars().all(|c| c.is_ascii_alphabetic())
            })
            .map(|x: &str| NodeId(x.as_bytes().try_into().unwrap()))
        };

        let edge = separated_pair(nodeid(), tag("-"), nodeid());
        let edges = many1(ws_line(edge));

        ws_all_consuming(edges).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "7")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "co,de,ka,ta")
    }
}
