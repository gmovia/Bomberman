use crate::types::position::Position;
#[derive(Debug, Clone)]
// Representa una pared.
pub struct Wall {
    pub position: Position,
}

impl Wall {
    pub fn new(position: Position) -> Wall {
        Wall { position: position }
    }
    // Obtengo el codigo.
    pub fn code(&self) -> String {
        'W'.to_string()
    }
    // Obtengo el tipo.
    pub fn typef(&self) -> char {
        'W'
    }
    // Si es detonada por una bomba entonces no es atravesada.
    pub fn be_detonated(&mut self) -> bool {
        false
    }
}
