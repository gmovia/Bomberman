use crate::{converter::Converter, element::Element};

pub struct Maze {
    maze: Vec<Vec<Element>>,
}

impl Maze {
    pub fn new(maze: &str) -> Maze {
        Maze {
            maze: Converter::string_to_matrix_object(maze),
        }
    }

    pub fn detonate(&mut self, x: usize, y: usize) -> Result<String, String> {
        if let Element::Bomb(_, scope) = &self.maze[x][y] {
            self.detonate_bomb(*scope, x, y)
        } else {
            return Err("ERR".to_string());
        }

        Ok(Converter::matrix_object_to_string(&self.maze))
    }

    pub fn detonate_bomb(&mut self, scope: usize, x: usize, y: usize) {
        self.maze[x][y] = Element::Empty;

        for row in x..x + scope + 1 {
            if row < self.maze.len() {
                if self.apply(row, y) == false{
                    break
                }
            }
        }
        let start = if scope > x { 0 } else { x - scope };
        for row in (start..x + 1).rev() {
            if row < self.maze.len() {
                if self.apply(row, y) == false{
                    break
                }
            }
        }

        for column in y..y + scope + 1 {
            if column < self.maze[0].len() {
                if self.apply(x, column) == false{
                    break
                }
            }
        }

        let start = if scope > y { 0 } else { y - scope };
        for column in (start..y + 1).rev() {
            if column < self.maze[0].len() {
                if self.apply(x, column) == false{
                    break
                }
            }
        }
    }

    pub fn apply(&mut self, x: usize, y: usize) -> bool{
        if let Element::Bomb(_, scope) = &self.maze[x][y] {
            self.detonate_bomb(*scope, x, y);
        }
        if let Element::Rock = &self.maze[x][y] {
            return false;
        }
        return true;        
    }
}
