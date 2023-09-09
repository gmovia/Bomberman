use crate::element::Element;
pub struct Maker;
impl Maker {
    pub fn make(code: &str) -> Element {
        let mut code_chars = code.chars();

        if let Some(first_char) = code_chars.next() {
            if let Some(second_char) = code_chars.next() {
                match (first_char, second_char) {
                    ('B', _) => Element::new_bomb(second_char as usize - 48),
                    ('F', _) => Element::new_player(second_char as usize - 48),
                    (_, _) => Element::new_empty(),
                }
            } else {
                match first_char {
                    'R' => Element::new_rock(),
                    'W' => Element::new_wall(),
                    _ => Element::new_empty(),
                }
            }
        } else {
            Element::new_empty()
        }
    }
}
