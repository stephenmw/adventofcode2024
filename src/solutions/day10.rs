use std::collections::BTreeSet;

use crate::grid::{Grid, Point};
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let ans: usize = grid
        .iter_items()
        .filter(|(_, v)| **v == 0)
        .map(|(p, _)| num_nines(&grid, p))
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let ans: usize = grid
        .iter_items()
        .filter(|(_, v)| **v == 0)
        .map(|(p, _)| num_paths(&grid, p))
        .sum();
    Ok(ans.to_string())
}

fn num_nines(grid: &Grid<u8>, start: Point) -> usize {
    fn rec(grid: &Grid<u8>, point: Point, ends: &mut BTreeSet<Point>, expected: u8) {
        let Some(&v) = grid.get(point) else {
            return;
        };

        if v != expected {
            return;
        }

        if v == 9 {
            ends.insert(point);
            return;
        }

        for p in point.iter_adjacent() {
            rec(grid, p, ends, expected + 1);
        }
    }

    let mut ends = BTreeSet::new();
    rec(grid, start, &mut ends, 0);
    ends.len()
}

fn num_paths(grid: &Grid<u8>, start: Point) -> usize {
    fn rec(grid: &Grid<u8>, point: Point, expected: u8) -> usize {
        let Some(&v) = grid.get(point) else {
            return 0;
        };

        if v != expected {
            return 0;
        }

        if v == 9 {
            return 1;
        }

        point
            .iter_adjacent()
            .map(|p| rec(grid, p, expected + 1))
            .sum()
    }

    rec(grid, start, 0)
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<u8>> {
        let elem = one_of("0123456789").map(|c| c.to_digit(10).unwrap() as u8);
        let row = ws_line(many1(elem));
        let rows = many1(row);
        let grid = rows.map(Grid::from);
        ws_all_consuming(grid).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "36")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
