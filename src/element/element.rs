use crate::element::bomb::Bomb;
use crate::lab::Maze;
use crate::types::direction::Direction;
use crate::types::position::Position;
use crate::utils::maker::Maker;
use crate::element::blast::Blast;
#[derive(Debug, Clone)]

pub enum Element {
    Bomb(Bomb),
    Rock(Position),
    Wall(Position),
    Player(usize, Position),
    Detour(char, Position),
    Empty(Position),
}

impl Element {
    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(bomb) => bomb.code,
            Element::Rock(_) => 'R',
            Element::Wall(_) => 'W',
            Element::Player(_, _) => 'F',
            Element::Detour(_, _) => 'D',
            Element::Empty(_) => '_',
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(bomb) => format!("{}{}", bomb.code, bomb.scope),
            Element::Rock(_) => 'R'.to_string(),
            Element::Wall(_) => 'W'.to_string(),
            Element::Detour(direction, _) => format!("{}{}", 'D', *direction),
            Element::Player(lifes, _) => {
                if *lifes > 0 {
                    format!("{}{}", 'F', *lifes)
                } else {
                    "_".to_string()
                }
            }
            Element::Empty(_) => "_".to_string(),
        }
    }

    pub fn apply(&mut self, maze: &mut Maze, current_scope: usize) -> bool {
        match self {
            Element::Bomb(bomb) => {
                bomb.detonate(maze);
                true
            }
            Element::Player(lifes, position) => {
                *lifes -= 1;
                maze.maze[position.y][position.x] =
                    Maker::new_player(*lifes, Position::new(position.x, position.y));
                true
            }
            Element::Detour(direction, position) => {
                match *direction {
                    'R' => Blast::expand(position.x + 1, position.y, current_scope, Direction::Right, maze),
                    'L' => Blast::expand(position.x - 1, position.y, current_scope, Direction::Left, maze),
                    'U' => Blast::expand(position.x, position.y - 1, current_scope, Direction::Up, maze),
                    'D' => Blast::expand(position.x, position.y + 1, current_scope, Direction::Down, maze),
                    _ => (),
                }
                false
            }
            Element::Rock(_) | Element::Wall(_) => false,
            _ => true,
        }
    }
}
