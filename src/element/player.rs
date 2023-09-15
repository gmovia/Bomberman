use crate::lab::Maze;
use crate::types::position::Position;
use crate::utils::maker::Maker;

#[derive(Debug, Clone)]

pub struct Player {
    pub position: Position,
    lifes: usize,
}

impl Player {
    pub fn new(lifes: usize, position: Position) -> Player {
        Player {
            lifes: lifes,
            position: position,
        }
    }

    pub fn code(&self) -> String {
        if self.lifes > 0 {
            format!("{}{}", 'F', self.lifes)
        } else {
            "_".to_string()
        }
    }

    pub fn typef(&self) -> char {
        'F'
    }

    pub fn be_detonated(&mut self, maze: &mut Maze) -> bool {
        if self.lifes > 0 {
            self.lifes -= 1;
        }
        maze.maze[self.position.y][self.position.x] =
            Maker::new_player(self.lifes, Position::new(self.position.x, self.position.y));
        true
    }
}