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

    #[test]
    fn test_09_result_lab() {
        let mut maze: Maze = Maze::new("B1 _ _\n_ _ _\nB2 _ _");
        assert_eq!("_ _ _\n_ _ _\nB2 _ _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_10_result_lab() {
        let mut maze: Maze = Maze::new("B3 _ _\n_ _ _\nB1 _ _");
        assert_eq!("_ _ _\n_ _ _\n_ _ _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_11_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _\nB2 _ _\n_ _ _");
        assert_eq!("_ _ _\n_ _ _\n_ _ _".to_string(), resolve(maze.detonate(1, 0)));
    }

    #[test]
    fn test_12_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 _ _\n_ _ _");
        assert_eq!("_ B1 _\n_ _ _\n_ _ _".to_string(), resolve(maze.detonate(1, 0)));
    }

    #[test]
    fn test_13_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\n_ B3 _\n_ _ _");
        assert_eq!("_ _ _\n_ _ _\n_ _ _".to_string(), resolve(maze.detonate(1, 1)));
    }
    
    #[test]
    fn test_14_result_lab() {
        let mut maze: Maze = Maze::new("B2 B1 _\n_ _ _\n_ _ _");
        assert_eq!("_ _ _\n_ _ _\n_ _ _".to_string(), resolve(maze.detonate(0, 1)));
    }

    #[test]
    fn test_15_result_lab() {
        let mut maze: Maze = Maze::new("B2 B1 _ _\n_ B2 _ _\nB1 _ B3 _\n_ _ _ B2");
        assert_eq!("B2 B1 _ _\n_ B2 _ _\n_ _ _ _\n_ _ _ B2".to_string(), resolve(maze.detonate(2, 2)));
    }

    #[test]
    fn test_16_result_lab() {
        let mut maze: Maze = Maze::new("B2 B1 _ _\n_ B2 _ _\nB1 _ B3 _\n_ _ _ B2");
        assert_eq!("_ _ _ _\n_ _ _ _\n_ _ B3 _\n_ _ _ B2".to_string(), resolve(maze.detonate(1, 1)));
    }
}   