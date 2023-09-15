use crate::element::blast::Blast;
use crate::element::bomb::Bomb;
use crate::element::detour::Detour;
use crate::element::empty::Empty;
use crate::element::player::Player;
use crate::element::rock::Rock;
use crate::element::wall::Wall;
use crate::lab::Maze;

#[derive(Debug, Clone)]

pub enum Element {
    Bomb(Bomb),
    Rock(Rock),
    Wall(Wall),
    Player(Player),
    Detour(Detour),
    Empty(Empty),
}

impl Element {
    pub fn typef(&self) -> char {
        match self {
            Element::Bomb(bomb) => bomb.typef(),
            Element::Rock(rock) => rock.typef(),
            Element::Wall(wall) => wall.typef(),
            Element::Player(player) => player.typef(),
            Element::Detour(detour) => detour.typef(),
            Element::Empty(empty) => empty.typef(),
        }
    }

    pub fn code(&self) -> String {
        match self {
            Element::Bomb(bomb) => bomb.code(),
            Element::Rock(rock) => rock.code(),
            Element::Wall(wall) => wall.code(),
            Element::Detour(detour) => detour.code(),
            Element::Player(player) => player.code(),
            Element::Empty(empty) => empty.code(),
        }
    }

    pub fn apply(&mut self, maze: &mut Maze, blast: &mut Blast) -> bool {
        match self {
            Element::Bomb(bomb) => bomb.be_detonated(maze),
            Element::Player(player) => player.be_detonated(maze),
            Element::Detour(detour) => detour.be_detonated(maze, blast),
            Element::Rock(rock) => rock.be_detonated(blast),
            Element::Wall(wall) => wall.be_detonated(),
            Element::Empty(empty) => empty.be_detonated(),
        }
    }
}
