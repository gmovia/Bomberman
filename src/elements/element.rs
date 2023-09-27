use crate::elements::blast::Blast;
use crate::elements::bomb::Bomb;
use crate::elements::detour::Detour;
use crate::elements::empty::Empty;
use crate::elements::player::Player;
use crate::elements::rock::Rock;
use crate::elements::wall::Wall;
use crate::lab::Maze;

#[derive(Debug, Clone)]
/// Representa un elemento del mapa.
pub enum Element {
    Bomb(Bomb),
    Rock(Rock),
    Wall(Wall),
    Player(Player),
    Detour(Detour),
    Empty(Empty),
}

impl Element {
    /// Devuelve el tipo del elemento.
    /// Ejemplo. Si es una bomba devuelve 'B' y si es un jugador devuelve 'F'.
    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(bomb) => bomb.typef(),
            Element::Rock(rock) => rock.typef(),
            Element::Wall(wall) => wall.typef(),
            Element::Player(player) => player.typef(),
            Element::Detour(detour) => detour.typef(),
            Element::Empty(empty) => empty.typef(),
        }
    }
    /// Devuelve el codigo del elemento.
    /// Ejemplo. Si es una bomba con alcance 1 devuelve 'B1' y si es una pared devuelve 'W'.
    pub fn code(&self) -> String {
        match self {
            Element::Bomb(bomb) => bomb.code(),
            Element::Rock(rock) => rock.code(),
            Element::Wall(wall) => wall.code(),
            Element::Detour(detour) => detour.code(),
            Element::Player(player) => player.code(),
            Element::Empty(empty) => empty.code(),
        }
    }
    /// Recibe un mapa y una rafaga. Trata de detonar el elemento y devuelve un booleano si esto fue posible.
    /// Ejemplo. Si es una bomba devuelve true y si es una pared devuelve false.
    pub fn apply(&mut self, maze: &mut Maze, blast: &mut Blast) -> bool {
        match self {
            Element::Bomb(bomb) => bomb.be_detonated(maze),
            Element::Player(player) => player.be_detonated(maze, blast),
            Element::Detour(detour) => detour.be_detonated(maze, blast),
            Element::Rock(rock) => rock.be_detonated(blast),
            Element::Wall(wall) => wall.be_detonated(),
            Element::Empty(empty) => empty.be_detonated(),
        }
    }
}
