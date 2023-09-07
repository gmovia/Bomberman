use std::io::Empty;

pub enum Element {
    Bomb(char, usize),
    Empty,
}

impl Element {
    pub fn new_bomb(code: char, scope: usize) -> Self {
        Element::Bomb('B', scope)
    }

    pub fn new_empty() -> Self {
        Element::Empty
    }

    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(code, _) => *code,
            Element::Empty => '_',
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(code, scope) => format!("{}{}", *code, *scope),
            Element::Empty => "_".to_string(),
        }
    }
}

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
                    _ => Element::Empty,
                }
            }
        } else {
            Element::Empty
        }
    }
}