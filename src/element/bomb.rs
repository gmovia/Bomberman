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
    pub fn new(scope: usize, position: Position) -> Bomb {
        Bomb {
            code: 'B',
            scope: scope,
            position: position,
        }
    }

    pub fn detonate(&self, maze: &mut Maze) {
        let (x, y, scope) = (self.position.x, self.position.y, self.scope);
        maze.maze[y][x] = Element::Empty(Position::new(self.position.x, self.position.y));
        Blast::expand(x, y, scope, Direction::Right, maze);
        Blast::expand(x, y, scope, Direction::Left, maze);
        Blast::expand(x, y, scope, Direction::Up, maze);
        Blast::expand(x, y, scope, Direction::Down, maze);
    }
}
