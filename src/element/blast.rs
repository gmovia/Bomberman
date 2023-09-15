use crate::types::position::Position;
use crate::{lab::Maze, types::direction::Direction};

use super::player::Player;
#[derive(Debug, Clone)]

pub struct Blast {
    pub position: Position,
    pub direction: Direction,
    pub scope: usize,
    pub code: char,
    pub players_attacked: Vec<Player>,
}

impl Blast {
    pub fn new(position: Position, direction: Direction, scope: usize, code: char) -> Blast {
        Blast {
            position: position,
            direction: direction,
            scope: scope,
            code: code,
            players_attacked: Vec::new(),
        }
    }

    pub fn attack_the_player(&mut self, player: &mut Player) -> bool {
        for player_attacked in &self.players_attacked {
            if player_attacked.position.equals(player.position.clone()) {
                return true;
            }
        }
        self.players_attacked.push(player.clone());
        return false;
    }

    fn move_to_position(&mut self, position: Position) -> bool {
        if !(self.position.equals(position.clone())) && self.scope > 0 {
            self.scope -= 1;
            self.position = position;
            return true;
        }
        return false;
    }

    pub fn deviate_to_right(&mut self, maze: &mut Maze) {
        self.direction = Direction::Right;
        if self.move_to_position(self.position.right()) {
            self.desplace(maze);
        }
    }

    pub fn deviate_to_left(&mut self, maze: &mut Maze) {
        self.direction = Direction::Left;
        if self.move_to_position(self.position.left()) {
            self.desplace(maze);
        }
    }

    pub fn deviate_to_up(&mut self, maze: &mut Maze) {
        self.direction = Direction::Up;
        if self.move_to_position(self.position.up()) {
            self.desplace(maze);
        }
    }

    pub fn deviate_to_down(&mut self, maze: &mut Maze) {
        self.direction = Direction::Down;
        if self.move_to_position(self.position.down()) {
            self.desplace(maze);
        }
    }

    pub fn desplace(&mut self, maze: &mut Maze) {
        let (start, end, a, b, c, d, reverse) = self.match_direction();

        if reverse {
            for index in (start..=end).rev() {
                if !self.match_element_and_apply(maze, a, b, c, d, index) {
                    break;
                }
            }
        } else {
            for index in start..=end {
                if !self.match_element_and_apply(maze, a, b, c, d, index) {
                    break;
                }
            }
        }
    }

    fn match_element_and_apply(
        &mut self,
        maze: &mut Maze,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        index: usize,
    ) -> bool {
        if maze.is_in_maze(
            self.position.x * a + index * c,
            self.position.y * b + index * d,
        ) {
            self.move_to_position(Position::new(
                self.position.x * a + index * c,
                self.position.y * b + index * d,
            ));
            return maze.maze[self.position.y][self.position.x]
                .clone()
                .apply(maze, self);
        }
        false
    }

    fn match_direction(&self) -> (usize, usize, usize, usize, usize, usize, bool) {
        match self.direction {
            Direction::Up => (
                if self.scope > self.position.y {
                    0
                } else {
                    self.position.y - self.scope
                },
                self.position.y,
                1,
                0,
                0,
                1,
                true,
            ),
            Direction::Down => (
                self.position.y,
                self.position.y + self.scope,
                1,
                0,
                0,
                1,
                false,
            ),
            Direction::Left => (
                if self.scope > self.position.x {
                    0
                } else {
                    self.position.x - self.scope
                },
                self.position.x,
                0,
                1,
                1,
                0,
                true,
            ),
            Direction::Right => (
                self.position.x,
                self.position.x + self.scope,
                0,
                1,
                1,
                0,
                false,
            ),
        }
    }
}
