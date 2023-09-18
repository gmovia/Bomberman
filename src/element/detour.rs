use crate::element::blast::Blast;
use crate::lab::Maze;
use crate::types::position::Position;
#[derive(Debug, Clone)]
// Representa un desvio.
pub struct Detour {
    direction: char,
    pub position: Position,
}

impl Detour {
    pub fn new(direction: char, position: Position) -> Detour {
        Detour {
            direction: direction,
            position: position,
        }
    }
    // Obtengo el codigo.
    pub fn code(&self) -> String {
        format!("{}{}", 'D', self.direction)
    }
    // Obtengo el tipo.
    pub fn typef(&self) -> char {
        'D'
    }
    // Si es detonada entonces cambia la direccion actual de la rafaga.
    pub fn be_detonated(&mut self, maze: &mut Maze, blast: &mut Blast) -> bool {
        match self.direction {
            'R' => blast.deviate_to_right(maze),
            'L' => blast.deviate_to_left(maze),
            'U' => blast.deviate_to_up(maze),
            'D' => blast.deviate_to_down(maze),
            _ => (),
        }
        false
    }
}
