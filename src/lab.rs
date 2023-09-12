use crate::element::bomb::Bomb;
use crate::element::element::Element;
use crate::types::direction::Direction;
use crate::types::position::Position;
use crate::utils::converter::Converter;
pub struct Maze {
    pub maze: Vec<Vec<Element>>,
    limit_x: usize,
    limit_y: usize,
}

impl Maze {
    pub fn new(maze: &str) -> Maze {
        let matrix = Converter::string_to_matrix_object(maze);
        let limit_y: usize = matrix.len();
        let limit_x: usize = matrix[0].len();
        Maze {
            maze: matrix,
            limit_x: limit_x,
            limit_y: limit_y,
        }
    }

    pub fn is_in_maze(&self, x: usize, y: usize) -> bool {
        x < self.limit_x && y < self.limit_y
    }

    pub fn detonate(&mut self, x: usize, y: usize) -> Result<String, String> {
        if let Element::Bomb(bomb) = &self.maze[y][x] {
            self.detonate_bomb(bomb.clone())
        } else {
            return Err("ERR".to_string());
        }

        Ok(Converter::matrix_object_to_string(&self.maze))
    }

    pub fn detonate_bomb(&mut self, bomb: Bomb) {
        let (x, y, scope) = (bomb.position.x, bomb.position.y, bomb.scope);
        self.maze[y][x] = Element::Empty(Position::new(bomb.position.x, bomb.position.y));
        self.expand(x, y, scope, Direction::Right);
        self.expand(x, y, scope, Direction::Down);
        self.expand(x, y, scope, Direction::Up);
        self.expand(x, y, scope, Direction::Left);
    }

    pub fn expand(&mut self, x: usize, y: usize, scope: usize, direction: Direction) {
        let (start, end, a, b, c, d, reverse) = match direction {
            Direction::Up => (if scope > y { 0 } else { y - scope }, y, 1, 0, 0, 1, true),
            Direction::Down => (y, y + scope, 1, 0, 0, 1, false),
            Direction::Left => (if scope > x { 0 } else { x - scope }, x, 0, 1, 1, 0, true),
            Direction::Right => (x, x + scope, 0, 1, 1, 0, false),
        };

        if reverse {
            for index in (start..=end).rev() {
                if self.is_in_maze(x * a + index * c, y * b + index * d) {
                    if !self.applys(
                        x * a + index * c,
                        y * b + index * d,
                        if end - index <= 0 { 0 } else { end - index - 1 },
                    ) {
                        break;
                    }
                }
            }
        } else {
            for index in start..=end {
                if self.is_in_maze(x * a + index * c, y * b + index * d) {
                    if !self.applys(
                        x * a + index * c,
                        y * b + index * d,
                        if end - index <= 0 { 0 } else { end - index - 1 },
                    ) {
                        break;
                    }
                }
            }
        }
    }

    pub fn applys(&mut self, x: usize, y: usize, current_scope: usize) -> bool {
        let mut element = self.maze[y][x].clone();
        element.apply(self, current_scope)
    }
}
