use crate::constants::consts::{ERR_NOT_BOMB, ERR_POSITION_NOT_INCLUDES};
use crate::elements::element::Element;
use crate::utils::converter::Converter;

/// Representa el laberinto. Incluye al conjunto de elementos que lo conforman y sus limites.

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
            limit_x,
            limit_y,
        }
    }

    /// Recibe una posicion (x, y) y verifica si pertenece al laberinto.
    /// En caso afirmativo, devuelve true.
    /// En caso negativo, devuelve false.

    pub fn is_in_maze(&self, x: usize, y: usize) -> bool {
        x < self.limit_x && y < self.limit_y
    }

    /// Recibe una posicion (x, y) y en caso de que haya alojada una bomba, la detona.
    /// En caso de que la posicion no pertenezca al laberinto se produce un error (ERR_POSITION_NOT_INCLUDES).
    /// En caso de que no haya alojada una bomba se produce un error (ERR_NOT_BOMB).

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
