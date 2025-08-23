use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

use ahash::AHashSet;

use std::collections::BTreeSet;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let Some((start, _)) = grid.iter_items().find(|(_, e)| **e == GridElem::GuardStart) else {
        bail!("no guard start found")
    };

    let (states, is_loop) = get_guard_states(&grid, start);
    if is_loop {
        bail!("loop detected");
    }

    let guard_locations = AHashSet::from_iter(states.into_iter().map(|(p, _)| p));
    Ok(guard_locations.len().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);

    let Some((start, _)) = grid.iter_items().find(|(_, e)| **e == GridElem::GuardStart) else {
        bail!("no guard start found")
    };

    let (states, is_loop) = get_guard_states(&grid, start);
    if is_loop {
        bail!("unexpected loop detected");
    }

    let candidates = AHashSet::from_iter(
        states
            .into_iter()
            .filter(|(p, _)| *p != start)
            .map(|(p, _)| p),
    );

    let mut g_idx = GridIndex::new(&grid);

    let ans = candidates
        .iter()
        .filter(|&&c| {
            g_idx.insert(c);
            let is_loop = detect_loop(&g_idx, start, Direction::Down);
            g_idx.remove(c);
            is_loop
        })
        .count();

    Ok(ans.to_string())
}

// Returns every guard state and if a loop was detected.
fn get_guard_states(grid: &Grid<GridElem>, start: Point) -> (AHashSet<(Point, Direction)>, bool) {
    let mut seen = AHashSet::new();

    let mut cur_pos = start;
    let mut cur_dir = Direction::Down; // Up in the problem is towards lower y values.
    'loop1: loop {
        for (pos, &value) in grid.iter_line(cur_pos, cur_dir) {
            if value == GridElem::Wall {
                cur_dir = cur_dir.rotate_left(); // rotates right
                continue 'loop1;
            }

            if !seen.insert((pos, cur_dir)) {
                return (seen, true);
            }
            cur_pos = pos;
        }

        // Hit the end of the grid without hitting a wall.
        return (seen, false);
    }
}

fn detect_loop(g: &GridIndex, start: Point, dir: Direction) -> bool {
    let mut cur_pos = start;
    let mut cur_dir = dir;

    let mut seen = AHashSet::new();
    loop {
        let Some(next_pos) = g.next(cur_pos, cur_dir) else {
            return false;
        };

        let next_dir = cur_dir.rotate_left(); // rotates right

        if !seen.insert((next_pos, next_dir)) {
            return true;
        }

        cur_pos = next_pos;
        cur_dir = next_dir; // rotates right
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GridElem {
    Empty,
    Wall,
    GuardStart,
}

#[derive(Clone, Debug)]
struct GridIndex {
    // x -> y values that contain walls.
    x_index: Vec<BTreeSet<usize>>,
    // y -> x values that contain walls.
    y_index: Vec<BTreeSet<usize>>,
}

impl GridIndex {
    fn new(grid: &Grid<GridElem>) -> Self {
        let (x_len, y_len) = grid.size();
        let mut ret = Self {
            x_index: vec![BTreeSet::new(); x_len],
            y_index: vec![BTreeSet::new(); y_len],
        };

        for (p, v) in grid.iter_items() {
            if v == &GridElem::Wall {
                ret.insert(p)
            }
        }

        ret
    }

    fn insert(&mut self, p: Point) {
        self.x_index[p.x].insert(p.y);
        self.y_index[p.y].insert(p.x);
    }

    fn remove(&mut self, p: Point) {
        self.x_index[p.x].remove(&p.y);
        self.y_index[p.y].remove(&p.x);
    }

    // returns location just before next wall.
    fn next(&self, pos: Point, dir: Direction) -> Option<Point> {
        let wall_pos = match dir {
            Direction::Up => self.x_index[pos.x]
                .range(pos.y + 1..)
                .next()
                .map(|&y| Point::new(pos.x, y)),
            Direction::Down => self.x_index[pos.x]
                .range(0..pos.y)
                .rev()
                .next()
                .map(|&y| Point::new(pos.x, y)),
            Direction::Right => self.y_index[pos.y]
                .range(pos.x + 1..)
                .next()
                .map(|&x| Point::new(x, pos.y)),
            Direction::Left => self.y_index[pos.y]
                .range(0..pos.x)
                .rev()
                .next()
                .map(|&x| Point::new(x, pos.y)),
        }?;

        wall_pos.next(dir.opposite())
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<GridElem>> {
        let grid_elem = alt((
            value(GridElem::Empty, char('.')),
            value(GridElem::Wall, char('#')),
            value(GridElem::GuardStart, char('^')),
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

    const EXAMPLE_INPUT: &str = "....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "41")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "6")
    }
}
