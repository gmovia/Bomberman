use crate::elements::blast::Blast;
use crate::types::position::Position;
#[derive(Debug, Clone)]
/// Representa una roca.
pub struct Rock {
    pub position: Position,
}

impl Rock {
    pub fn new(position: Position) -> Rock {
        Rock { position }
    }

    pub fn code(&self) -> String {
        'R'.to_string()
    }

    pub fn typef(&self) -> char {
        'R'
    }

    /// Recibe una rafaga. Se trata de detonar una piedra.
    /// Si pertenece a una bomba comun devuelve false y si pertenece a una super bomba devuelve true.
    pub fn be_detonated(&mut self, blast: &mut Blast) -> bool {
        blast.code == 'S'
    }
}
