use crate::types::position::Position;
#[derive(Debug, Clone)]
// Representa un elemento vacio.
pub struct Empty {
    pub position: Position,
}

impl Empty {
    pub fn new(position: Position) -> Empty {
        Empty { position: position }
    }
    // Obtengo el codigo.
    pub fn code(&self) -> String {
        '_'.to_string()
    }
    // Obtengo el tipo.
    pub fn typef(&self) -> char {
        '_'
    }
    // Si es detonado entonces la rafaga sigue recorriendo.
    pub fn be_detonated(&mut self) -> bool {
        true
    }
}
