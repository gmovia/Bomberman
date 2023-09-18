#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }

    pub fn equals(&self, position: Position) -> bool {
        self.x == position.x && self.y == position.y
    }

    pub fn right(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }

    pub fn left(&self) -> Position {
        Position::new(self.x - 1, self.y)
    }

    pub fn up(&self) -> Position {
        Position::new(self.x, self.y - 1)
    }

    pub fn down(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }
}
