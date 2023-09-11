use crate::element::bomb::Bomb;
use crate::lab::Maze;
use crate::types::position::Position;
#[derive(Debug, Clone)]

pub enum Element {
    Bomb(Bomb),
    Rock(Position),
    Wall(Position),
    Player(char, usize, Position),
    Empty(Position),
}

impl Element {
    pub fn new_bomb(scope: usize, position: Position) -> Self {
        Element::Bomb(Bomb::new(scope, position))
    }

    pub fn new_empty(position: Position) -> Self {
        Element::Empty(position)
    }

    pub fn new_rock(position: Position) -> Self{
        Element::Rock(position)
    }

    pub fn new_wall(position: Position) -> Self{
        Element::Wall(position)
    }

    pub fn new_player(lifes: usize, position: Position) -> Self{
        Element::Player('F', lifes, position)
    }

    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(bomb) => bomb.code,
            Element::Rock(_) => 'R',
            Element::Wall(_) => 'W',
            Element::Player(code, _, _) => *code,
            Element::Empty(_) => '_',
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(bomb) => format!("{}{}", bomb.code, bomb.scope),
            Element::Rock(_) => 'R'.to_string(),
            Element::Wall(_) => 'W'.to_string(),
            Element::Player(code, lifes, _) => if *lifes > 0 {format!("{}{}", *code, *lifes)} else {"_".to_string()},
            Element::Empty(_) => "_".to_string(),
        }
    }

    pub fn apply(&mut self, maze: &mut Maze, x: usize, y: usize) -> bool {
        match self{
            Element::Bomb(bomb) => {
                maze.detonate_bomb(bomb.clone());
                true
            },
            Element::Player(_, ref mut lifes, position) => {
                *lifes -= 1;
                maze.maze[x][y] = Element::new_player(*lifes, Position::new(position.x, position.y));
                true
            }
            Element::Rock(_) | Element::Wall(_) => false,
            _ => true,
        }
    }
}