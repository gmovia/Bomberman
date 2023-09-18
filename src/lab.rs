use crate::constants::constants::{ERR_NOT_BOMB, ERR_POSITION_NOT_INCLUDES};
use crate::element::element::Element;
use crate::utils::converter::Converter;
// Representa el mapa del juego.
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
    // Se fija si una posicion se encuentra dentro del mapa.
    pub fn is_in_maze(&self, x: usize, y: usize) -> bool {
        x < self.limit_x && y < self.limit_y
    }
    // Detona una direccion.
    pub fn detonate(&mut self, x: usize, y: usize) -> Result<String, String> {
        if !self.is_in_maze(x, y) {
            return Err(ERR_POSITION_NOT_INCLUDES.to_string());
        }

        if let Element::Bomb(bomb) = self.maze[y][x].clone() {
            bomb.clone().detonate(self);
        } else {
            return Err(ERR_NOT_BOMB.to_string());
        }

        Ok(Converter::matrix_object_to_string(&self.maze))
    }
}
