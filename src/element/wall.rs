use crate::types::position::Position;
#[derive(Debug, Clone)]
pub struct Wall {
    pub position: Position,
}

impl Wall {
    pub fn new(position: Position) -> Wall {
        Wall { position: position }
    }

    pub fn code(&self) -> String {
        'W'.to_string()
    }

    pub fn typef(&self) -> char {
        'W'
    }

    pub fn be_detonated(&mut self) -> bool {
        false
    }
}
