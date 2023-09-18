use crate::element::blast::Blast;
use crate::types::position::Position;
#[derive(Debug, Clone)]
// Representa una roca.
pub struct Rock {
    pub position: Position,
}

impl Rock {
    pub fn new(position: Position) -> Rock {
        Rock { position: position }
    }
    // Obtengo el codigo.
    pub fn code(&self) -> String {
        'R'.to_string()
    }
    // Obtengo el tipo.
    pub fn typef(&self) -> char {
        'R'
    }
    // Si es atacada por una superbomba entonces es atravesada.
    pub fn be_detonated(&mut self, blast: &mut Blast) -> bool {
        blast.code == 'S'
    }
}
