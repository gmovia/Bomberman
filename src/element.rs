pub enum Element {
    Bomb(char, usize),
    Rock,
    Wall,
    Player(char, usize),
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

    pub fn new_player(lifes: usize) -> Self{
        Element::Player('F', lifes)
    }

    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(code, _) => *code,
            Element::Rock => 'R',
            Element::Wall => 'W',
            Element::Player(code, _) => *code,
            Element::Empty => '_',
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(code, scope) => format!("{}{}", *code, *scope),
            Element::Rock => 'R'.to_string(),
            Element::Wall => 'W'.to_string(),
            Element::Player(code, lifes) => if(*lifes > 0){format!("{}{}", *code, *lifes)} else {"_".to_string()},
            Element::Empty => "_".to_string(),
        }
    }

    
}