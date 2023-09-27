use crate::types::position::Position;
#[derive(Debug, Clone)]
/// Representa un elemento vacio.
pub struct Empty {
    pub position: Position,
}

impl Empty {
    pub fn new(position: Position) -> Empty {
        Empty { position }
    }

    pub fn code(&self) -> String {
        '_'.to_string()
    }

    pub fn typef(&self) -> char {
        '_'
    }

    /// Como siempre puede ser detonado, devuelve true.
    pub fn be_detonated(&mut self) -> bool {
        true
    }
}
