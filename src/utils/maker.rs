use crate::element::element::Element;
use crate::types::position::Position;
pub struct Maker;
impl Maker {
    pub fn make(code: &str, position: Position) -> Element {
        let mut code_chars = code.chars();

        if let Some(first_char) = code_chars.next() {
            if let Some(second_char) = code_chars.next() {
                match (first_char, second_char) {
                    ('B', _) => Element::new_bomb(second_char as usize - 48, position),
                    ('F', _) => Element::new_player(second_char as usize - 48, position),
                    (_, _) => Element::new_empty(position),
                }
            } else {
                match first_char {
                    'R' => Element::new_rock(position),
                    'W' => Element::new_wall(position),
                    _ => Element::new_empty(position),
                }
            }
        } else {
            Element::new_empty(position)
        }
    }
}
