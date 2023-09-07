#[cfg(test)]
use taller1_tp1_bomberman::lab::Maze;

pub fn resolve(string: Result<String, String>) -> String{
    match string {
        Ok(exit) => exit,
        Err(err) => err
    }
}

mod tests {
    
    use super::*;

    #[test]
    fn test_01_result_lab() {
        let mut maze: Maze = Maze::new("B1");
        assert_eq!("_".to_string(), resolve(maze.detonate(0, 0)));
    }
    
    
    #[test]
    fn test_02_result_lab() {
        let mut maze: Maze = Maze::new("_");
        assert_eq!("ERR".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_03_result_lab() {
        let mut maze: Maze = Maze::new("B1 _");
        assert_eq!("_ _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_04_result_lab() {
        let mut maze: Maze = Maze::new("B1 _\n _ _");
        assert_eq!("_ _\n_ _".to_string(), resolve(maze.detonate(0, 0)));
    }


    #[test]
    fn test_05_result_lab() {
        let mut maze: Maze = Maze::new("B1 _\n _ B1");
        assert_eq!("_ _\n_ B1".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_06_result_lab() {
        let mut maze: Maze = Maze::new("B1 B1\n _ B1");
        assert_eq!("_ _\n_ _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_07_result_lab() {
        let mut maze: Maze = Maze::new("B2 B2\n _ B2");
        assert_eq!("_ _\n_ _".to_string(), resolve(maze.detonate(0, 0)));
    }
    
    #[test]
    fn test_08_result_lab() {
        let mut maze: Maze = Maze::new("B2 _ _\n_ _ _\nB2 _ _");
        assert_eq!("_ _ _\n_ _ _\n_ _ _".to_string(), resolve(maze.detonate(0, 0)));
    }
}   