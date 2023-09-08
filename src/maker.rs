use crate::element::Element;
pub struct Maker;
impl Maker {
    pub fn make(code: &str) -> Element {
        let mut code_chars = code.chars();

        if let Some(first_char) = code_chars.next() {
            if let Some(second_char) = code_chars.next() {
                match (first_char, second_char) {
                    (_, _) => Element::Bomb(first_char, second_char as usize - 48),
                }
            } else {
                match first_char {
                    'R' => Element::Rock,
                    _ => Element::Empty,
                }
            }
        } else {
            Element::Empty
        }
    }
}
