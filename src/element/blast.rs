use crate::{types::direction::Direction, lab::Maze};

pub struct Blast;

impl Blast{
    pub fn expand(x: usize, y: usize, scope: usize, direction: Direction, maze: &mut Maze) {
        let (start, end, a, b, c, d, reverse) = match direction {
            Direction::Up => (if scope > y { 0 } else { y - scope }, y, 1, 0, 0, 1, true),
            Direction::Down => (y, y + scope, 1, 0, 0, 1, false),
            Direction::Left => (if scope > x { 0 } else { x - scope }, x, 0, 1, 1, 0, true),
            Direction::Right => (x, x + scope, 0, 1, 1, 0, false),
        };

        if reverse {
            for index in (start..=end).rev() {
                if maze.is_in_maze(x * a + index * c, y * b + index * d) {
                    if !maze.maze[y * b + index * d][x * a + index * c].clone().apply(
                        maze,
                        if end - index <= 0 { 0 } else { end - index - 1 },
                    ) {
                        break;
                    }
                }
            }
        } else {
            for index in start..=end {
                if maze.is_in_maze(x * a + index * c, y * b + index * d) {
                    if !maze.maze[y * b + index * d][x * a + index * c].clone().apply(
                        maze,
                        if end - index <= 0 { 0 } else { end - index - 1 },
                    ) {
                        break;
                    }
                }
            }
        }
    }
}