use crate::types::position::Position;
use crate::element::blast::Blast;
use crate::types::direction::Direction;
use crate::element::element::Element;
use crate::lab::Maze;

#[derive(Debug, Clone)]
pub struct Bomb {
    pub code: char,
    pub scope: usize,
    pub position: Position,
}

impl Bomb {
    pub fn new(code: char, scope: usize, position: Position) -> Bomb {
        Bomb {
            code: code,
            scope: scope,
            position: position,
        }
    }

    pub fn detonate(&self, maze: &mut Maze) {
        let (x, y, scope) = (self.position.x, self.position.y, self.scope);
        maze.maze[y][x] = Element::Empty(Position::new(x, y));
        Blast::expand(x, y, scope, Direction::Right, maze, self.code);
        Blast::expand(x, y, scope, Direction::Left, maze, self.code);
        Blast::expand(x, y, scope, Direction::Up, maze, self.code);
        Blast::expand(x, y, scope, Direction::Down, maze, self.code);
    }
}
