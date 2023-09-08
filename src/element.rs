pub enum Element {
    Bomb(char, usize),
    Rock,
    Wall,
    Empty,
}

impl Element {
    pub fn new_bomb(scope: usize) -> Self {
        Element::Bomb('B', scope)
    }

    pub fn new_empty() -> Self {
        Element::Empty
    }

    pub fn new_rock() -> Self{
        Element::Rock
    }

    pub fn new_wall() -> Self{
        Element::Wall
    }

    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(code, _) => *code,
            Element::Rock => 'R',
            Element::Wall => 'W',
            Element::Empty => '_',
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(code, scope) => format!("{}{}", *code, *scope),
            Element::Rock => 'R'.to_string(),
            Element::Wall => 'W'.to_string(),
            Element::Empty => "_".to_string(),
        }
    }
}

