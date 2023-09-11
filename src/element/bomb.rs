use crate::types::position::Position;
#[derive(Debug, Clone)]
pub struct Bomb{
    pub code: char,
    pub scope: usize,
    pub position: Position
}

impl Bomb{
    pub fn new(scope: usize, position: Position) -> Bomb{
        Bomb{code: 'B', scope: scope, position: position}
    }
} 