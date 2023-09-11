use crate::types::direction::Direction;
use crate::element::bomb::Bomb;
use crate::lab::Maze;
use crate::types::position::Position;
#[derive(Debug, Clone)]

pub enum Element {
    Bomb(Bomb),
    Rock(Position),
    Wall(Position),
    Player(char, usize, Position),
    Detour(char, char, Position),
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

    pub fn new_detour(direction: char, position: Position) -> Self{
        Element::Detour('D', direction, position)
    }

    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(bomb) => bomb.code,
            Element::Rock(_) => 'R',
            Element::Wall(_) => 'W',
            Element::Player(code, _, _) => *code,
            Element::Detour(code, _, _) => *code, 
            Element::Empty(_) => '_',
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(bomb) => format!("{}{}", bomb.code, bomb.scope),
            Element::Rock(_) => 'R'.to_string(),
            Element::Wall(_) => 'W'.to_string(),
            Element::Detour(code, direction, _) => format!("{}{}", *code, *direction),
            Element::Player(code, lifes, _) => if *lifes > 0 {format!("{}{}", *code, *lifes)} else {"_".to_string()},
            Element::Empty(_) => "_".to_string(),
        }
    }

    pub fn apply(&mut self, maze: &mut Maze, current_scope: usize) -> bool {
        match self {
            Element::Bomb(bomb) => {
                maze.detonate_bomb(bomb.clone());
                true
            },
            Element::Player(_, lifes, position) => {
                *lifes -= 1;
                maze.maze[position.x][position.y] = Element::new_player(*lifes, Position::new(position.x, position.y));                true
            },
            Element::Detour(_, direction, position) => {
                match *direction {
                    'R' => maze.expand(position.x, position.y + 1, current_scope, Direction::Right),
                    'L' => maze.expand(position.x, position.y - 1, current_scope, Direction::Left),
                    'U' => maze.expand(position.x - 1, position.y, current_scope, Direction::Up),
                    'D' => maze.expand(position.x + 1, position.y, current_scope, Direction::Down),
                    _ => ()
                }
                false
            }
            Element::Rock(_) | Element::Wall(_) => false,
            _ => true,
        }
    }
    
}