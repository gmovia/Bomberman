#[derive(Debug, Clone)]
// Representa una posicion del mapa.
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }
    // Me fijo si la posicion es igual a otra.
    pub fn equals(&self, position: Position) -> bool {
        self.x == position.x && self.y == position.y
    }
    // Obtengo la posicion derecha.
    pub fn right(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }
    // Obtengo la posicion izquierda.
    pub fn left(&self) -> Position {
        Position::new(self.x - 1, self.y)
    }
    // Obtengo la posicion superior.
    pub fn up(&self) -> Position {
        Position::new(self.x, self.y - 1)
    }
    // Obtengo la posicion inferior.
    pub fn down(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }
}
