use crate::elements::blast::Blast;
use crate::elements::bomb::Bomb;
use crate::elements::detour::Detour;
use crate::elements::element::Element;
use crate::elements::empty::Empty;
use crate::elements::player::Player;
use crate::elements::rock::Rock;
use crate::elements::wall::Wall;
use crate::types::direction::Direction;
use crate::types::position::Position;
/// Tiene como responsabilidad crear los elementos del laberinto.
pub struct Maker;
impl Maker {
    /// Recibe un alcance y una posicion.
    /// Devuelve una bomba.
    pub fn new_bomb(scope: usize, position: Position) -> Element {
        Element::Bomb(Bomb::new('B', scope, position))
    }

    /// Recibe un alcance y una posicion.
    /// Devuelve una super bomb.
    pub fn new_super_bomb(scope: usize, position: Position) -> Element {
        Element::Bomb(Bomb::new('S', scope, position))
    }

    /// Recibe un alcance, una posicion, un codigo y una direccion.
    /// Devuelve una rafaga.
    pub fn make_blast(position: Position, direction: Direction, scope: usize, code: char) -> Blast {
        Blast::new(position, direction, scope, code)
    }

    /// Recibe una posicion.
    /// Devuelve un elemento vacio.
    pub fn new_empty(position: Position) -> Element {
        Element::Empty(Empty::new(position))
    }

    /// Recibe una posicion.
    /// Devuelve una roca.
    pub fn new_rock(position: Position) -> Element {
        Element::Rock(Rock::new(position))
    }

    /// Recibe una posicion.
    /// Devuelve una pared.
    pub fn new_wall(position: Position) -> Element {
        Element::Wall(Wall::new(position))
    }

    /// Recibe una posicion y una cantidad de vidas.
    /// Devuelve un jugador.
    pub fn new_player(lifes: usize, position: Position) -> Element {
        Element::Player(Player::new(lifes, position))
    }

    /// Recibe un desvio y una posicion.
    /// Devuelve un desvio.
    pub fn new_detour(direction: char, position: Position) -> Element {
        Element::Detour(Detour::new(direction, position))
    }

    /// Recibe un codigo y una posicion.
    /// Devuelve un elemento.
    /// Ejemplo: Si el codigo es 'R' devuelve una Roca, si es 'W' una pared, si es 'F1' devuelve un jugador con una vida, etc.
    pub fn make(code: &str, position: Position) -> Element {
        let mut code_chars = code.chars();

        if let Some(first_char) = code_chars.next() {
            if let Some(second_char) = code_chars.next() {
                match (first_char, second_char) {
                    ('B', _) => Self::new_bomb(second_char as usize - 48, position),
                    ('S', _) => Self::new_super_bomb(second_char as usize - 48, position),
                    ('F', _) => Self::new_player(second_char as usize - 48, position),
                    ('D', _) => Self::new_detour(second_char, position),
                    (_, _) => Self::new_empty(position),
                }
            } else {
                match first_char {
                    'R' => Self::new_rock(position),
                    'W' => Self::new_wall(position),
                    _ => Self::new_empty(position),
                }
            }
        } else {
            Self::new_empty(position)
        }
    }
}
