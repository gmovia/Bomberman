use crate::types::position::Position;
#[derive(Debug, Clone)]

pub struct Empty {
    pub position: Position,
}

impl Empty {
    pub fn new(position: Position) -> Empty {
        Empty { position: position }
    }
    pub fn code(&self) -> String {
        '_'.to_string()
    }

    pub fn typef(&self) -> char {
        '_'
    }

    pub fn be_detonated(&mut self) -> bool {
        true
    }
}
