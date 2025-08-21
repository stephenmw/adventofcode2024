use crate::grid::{Direction, Grid, Point};
use crate::solutions::prelude::*;

use ahash::AHashSet;
use anyhow::Ok;

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

    let ans = candidates
        .iter()
        .filter(|&&c| {
            let mut g = grid.clone();
            *(g.get_mut(c).unwrap()) = GridElem::Wall;
            let (_, is_loop) = get_guard_states(&g, start);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GridElem {
    Empty,
    Wall,
    GuardStart,
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
