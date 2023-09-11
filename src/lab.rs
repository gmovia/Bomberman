use crate::element::element::Element;
use crate::element::bomb::Bomb;
use crate::types::position::Position;
use crate::utils::converter::Converter;
use crate::types::direction::Direction;
pub struct Maze {
    pub maze: Vec<Vec<Element>>,
    limit_x: usize,
    limit_y: usize
}

impl Maze {
    pub fn new(maze: &str) -> Maze {
        let matrix = Converter::string_to_matrix_object(maze);
        let limit_x: usize = matrix.len();
        let limit_y: usize = matrix[0].len();
        Maze {
            maze: matrix, 
            limit_x: limit_x, 
            limit_y: limit_y
        }
    }

    pub fn is_in_maze(&self, x: usize, y: usize) -> bool{
        x < self.limit_x && y < self.limit_y
    }

    pub fn detonate(&mut self, x: usize, y: usize) -> Result<String, String> {
        if let Element::Bomb(bomb) = &self.maze[x][y] {
            self.detonate_bomb(bomb.clone())
        } else {
            return Err("ERR".to_string());
        }

        Ok(Converter::matrix_object_to_string(&self.maze))
    }

    pub fn detonate_bomb(&mut self, bomb: Bomb) {
        let (x, y, scope) = (bomb.position.x, bomb.position.y, bomb.scope);
        self.maze[x][bomb.position.y] = Element::Empty(Position::new(bomb.position.x, bomb.position.y));
        self.expand(x, y, scope, Direction::Right, false);
        self.expand(x, y, scope, Direction::Down, false);
        self.expand(x, y, scope, Direction::Up, true);
        self.expand(x, y, scope, Direction::Left, true);
    }

    
    fn expand(&mut self, x: usize, y: usize, scope: usize, direction: Direction, reverse: bool){
        let (start, end, a, b, c, d) = match direction {
            Direction::Up => (if scope > x { 0 } else { x - scope }, x, 0, 1, 1, 0),
            Direction::Down => (x, x + scope, 0, 1, 1, 0),
            Direction::Left => (if scope > y { 0 } else { y - scope }, y, 1, 0, 0, 1),
            Direction::Right => (y, y + scope, 1, 0, 0, 1),
        };
        
        if reverse == true{
            for index in (start..=end).rev(){
                if self.is_in_maze(x*a + index*c, y*b + index*d){
                    if !self.apply(x*a + index*c, y*b + index*d){
                        break;
                    }
                }
            }
        }
        else{
            for index in start..=end{
                if self.is_in_maze(x*a + index*c, y*b + index*d){
                    if !self.apply(x*a + index*c, y*b + index*d){
                        break;
                    }
                }
            }
        }
    }
    
    pub fn apply(&mut self, x: usize, y: usize) -> bool{
        let mut element = self.maze[x][y].clone();
        element.apply(self, x, y)
    }    
}