use std::collections::BinaryHeap;

use ahash::AHashSet;

use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;
use crate::utils::RevHeapElem;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let maze = Maze::try_from(grid)?;

    let ans = distance(&maze.grid, maze.start, maze.end)?;

    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    bail!("not yet implemented")
}

fn distance(grid: &Grid<GridElem>, start: Point, end: Point) -> Result<u64, anyhow::Error> {
    let mut frontier = BinaryHeap::new();
    frontier.push(RevHeapElem {
        key: 0,
        value: (start, Direction::Right),
    });
    let mut visited = AHashSet::new();

    while let Some(elem) = frontier.pop() {
        let cost = elem.key;
        let (cur_pos, cur_dir) = elem.value;

        if cur_pos == end {
            return Ok(cost);
        };

        if !visited.insert(elem.value) {
            continue;
        };

        let candidate_dir = [cur_dir, cur_dir.rotate_left(), cur_dir.rotate_right()];

        for dir in candidate_dir {
            let Some(next_pos) = cur_pos.next(dir) else {
                continue;
            };

            match grid.get(next_pos) {
                None | Some(&GridElem::Wall) => {
                    continue;
                }
                _ => (),
            };

            if visited.contains(&(next_pos, dir)) {
                continue;
            };

            let new_cost = cost + if cur_dir == dir { 1 } else { 1001 };

            frontier.push(RevHeapElem {
                key: new_cost,
                value: (next_pos, dir),
            });
        }
    }

    bail!("end unreachable");
}

struct Maze {
    grid: Grid<GridElem>,
    start: Point,
    end: Point,
}

impl TryFrom<Grid<GridElem>> for Maze {
    type Error = anyhow::Error;

    fn try_from(grid: Grid<GridElem>) -> Result<Self, Self::Error> {
        let Some((start, _)) = grid.iter_items().find(|(_, e)| **e == GridElem::Start) else {
            bail!("no start found")
        };

        let Some((end, _)) = grid.iter_items().find(|(_, e)| **e == GridElem::End) else {
            bail!("no end found")
        };

        Ok(Self { grid, start, end })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GridElem {
    Empty,
    Wall,
    Start,
    End,
}

mod parser {
    use super::*;
    use crate::{grid::Grid, parser::prelude::*};

    pub fn parse(input: &str) -> IResult<&str, Grid<GridElem>> {
        let grid_elem = alt((
            value(GridElem::Empty, char('.')),
            value(GridElem::Wall, char('#')),
            value(GridElem::Start, char('S')),
            value(GridElem::End, char('E')),
        ));

        let row = ws_line(many1(grid_elem));
        let rows = many1(row);
        let grid = rows.map(Grid::new);

        ws_all_consuming(grid).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "7036")
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
