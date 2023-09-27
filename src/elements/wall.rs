use crate::types::position::Position;
#[derive(Debug, Clone)]
/// Representa una pared.
pub struct Wall {
    pub position: Position,
}

impl Wall {
    pub fn new(position: Position) -> Wall {
        Wall { position }
    }

    pub fn code(&self) -> String {
        'W'.to_string()
    }

    pub fn typef(&self) -> char {
        'W'
    }

    /// Como una pared no puede ser detonada, devuelve false.
    pub fn be_detonated(&mut self) -> bool {
        false
    }
}
