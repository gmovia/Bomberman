use crate::element::blast::Blast;
use crate::types::position::Position;
#[derive(Debug, Clone)]

pub struct Rock {
    pub position: Position,
}

impl Rock {
    pub fn new(position: Position) -> Rock {
        Rock { position: position }
    }
    pub fn code(&self) -> String {
        'R'.to_string()
    }

    pub fn typef(&self) -> char {
        'R'
    }

    pub fn be_detonated(&mut self, blast: &mut Blast) -> bool {
        blast.code == 'S'
    }
}
