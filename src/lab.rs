use crate::element::element::Element;
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
        if let Element::Bomb(bomb) = self.maze[y][x].clone() {
            bomb.detonate(self);
        } else {
            return Err("ERR".to_string());
        }

        Ok(Converter::matrix_object_to_string(&self.maze))
    }
}
