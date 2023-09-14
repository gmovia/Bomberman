use crate::element::blast::Blast;
use crate::element::bomb::Bomb;
use crate::lab::Maze;
use crate::types::position::Position;
use crate::utils::maker::Maker;
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

    pub fn apply(&mut self, maze: &mut Maze, blast: &mut Blast) -> bool {
        match self {
            Element::Bomb(bomb) => {
                bomb.detonate(maze);
                true
            }
            Element::Player(lifes, position) => {
                if *lifes > 0{
                    *lifes -= 1;
                }
                maze.maze[position.y][position.x] =
                    Maker::new_player(*lifes, Position::new(position.x, position.y));
                true
            }
            Element::Detour(direction, _) => {
                match *direction {
                    'R' => blast.deviate_to_right(maze),
                    'L' => blast.deviate_to_left(maze),
                    'U' => blast.deviate_to_up(maze),
                    'D' => blast.deviate_to_down(maze),
                    _ => (),
                }
                false
            }
            Element::Rock(_) => blast.code == 'S',
            Element::Wall(_) => false,
            _ => true,
        }
    }
}
