use crate::element::blast::Blast;
use crate::element::bomb::Bomb;
use crate::element::detour::Detour;
use crate::element::element::Element;
use crate::element::empty::Empty;
use crate::element::player::Player;
use crate::element::rock::Rock;
use crate::element::wall::Wall;
use crate::types::direction::Direction;
use crate::types::position::Position;
// Es un creador de objetos.
pub struct Maker;
impl Maker {
    // Crea una bomba.
    pub fn new_bomb(scope: usize, position: Position) -> Element {
        Element::Bomb(Bomb::new('B', scope, position))
    }
    // Crea una super bomba.
    pub fn new_super_bomb(scope: usize, position: Position) -> Element {
        Element::Bomb(Bomb::new('S', scope, position))
    }
    // Crea una rafaga.
    pub fn make_blast(position: Position, direction: Direction, scope: usize, code: char) -> Blast {
        Blast::new(position, direction, scope, code)
    }
    // Crea un elemento vacio.
    pub fn new_empty(position: Position) -> Element {
        Element::Empty(Empty::new(position))
    }
    // Crea una roca.
    pub fn new_rock(position: Position) -> Element {
        Element::Rock(Rock::new(position))
    }
    // Crea una pared.
    pub fn new_wall(position: Position) -> Element {
        Element::Wall(Wall::new(position))
    }
    // Crea un jugador.
    pub fn new_player(lifes: usize, position: Position) -> Element {
        Element::Player(Player::new(lifes, position))
    }
    // Crea un desvio.
    pub fn new_detour(direction: char, position: Position) -> Element {
        Element::Detour(Detour::new(direction, position))
    }
    // Crea un elemento.
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
