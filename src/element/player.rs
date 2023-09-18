use crate::element::blast::Blast;
use crate::lab::Maze;
use crate::types::position::Position;
use crate::utils::maker::Maker;

#[derive(Debug, Clone)]
// Representa al enemigo.
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
    // Obtengo el codigo.
    pub fn code(&self) -> String {
        if self.lifes > 0 {
            format!("{}{}", 'F', self.lifes)
        } else {
            "_".to_string()
        }
    }
    // Obtengo el tipo
    pub fn typef(&self) -> char {
        'F'
    }
    // Si es detonado, tiene vidas y no fue atacado por la rafaga, entonces disminuyo sus vidas y actualizo el mapa.
    pub fn be_detonated(&mut self, maze: &mut Maze, blast: &mut Blast) -> bool {
        if self.lifes > 0 && !blast.attack_the_player(self) {
            self.lifes -= 1;
        }
        maze.maze[self.position.y][self.position.x] =
            Maker::new_player(self.lifes, Position::new(self.position.x, self.position.y));
        true
    }
}
