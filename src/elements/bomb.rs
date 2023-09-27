use crate::elements::blast::Blast;
use crate::lab::Maze;
use crate::types::direction::Direction;
use crate::types::position::Position;
use crate::utils::maker::Maker;

#[derive(Debug, Clone)]

pub struct Bomb {
    pub code: char,
    pub scope: usize,
    pub position: Position,
    pub blasts: Vec<Blast>,
}

impl Bomb {
    pub fn new(code: char, scope: usize, position: Position) -> Bomb {
        let blast_right = Maker::make_blast(position.clone(), Direction::Right, scope, code);
        let blast_left = Maker::make_blast(position.clone(), Direction::Left, scope, code);
        let blast_up = Maker::make_blast(position.clone(), Direction::Up, scope, code);
        let blast_down = Maker::make_blast(position.clone(), Direction::Down, scope, code);

        let blasts = vec![blast_right, blast_left, blast_up, blast_down];

        Bomb {
            code,
            scope,
            position,
            blasts,
        }
    }

    pub fn detonate(&mut self, maze: &mut Maze) {
        let (x, y) = (self.position.x, self.position.y);
        maze.maze[y][x] = Maker::new_empty(Position::new(x, y));

        for blast in &mut self.blasts {
            blast.desplace(maze);
        }
    }

    pub fn code(&self) -> String {
        format!("{}{}", self.code, self.scope)
    }

    pub fn typef(&self) -> char {
        self.code
    }

    pub fn be_detonated(&mut self, maze: &mut Maze) -> bool {
        self.detonate(maze);
        true
    }
}
