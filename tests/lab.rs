#[cfg(test)]
use taller1_tp1_bomberman::lab::Maze;

pub fn resolve(string: Result<String, String>) -> String {
    match string {
        Ok(exit) => exit,
        Err(err) => err,
    }
}

mod tests {

    use super::*;

    // BOMB AND NOTHING

    #[test]
    fn test_01_result_lab() {
        let mut maze: Maze = Maze::new("B1");
        assert_eq!("_".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_02_result_lab() {
        let mut maze: Maze = Maze::new("_");
        assert_eq!(
            "ERROR: The position does not includes a bomb".to_string(),
            resolve(maze.detonate(0, 0))
        );
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
        assert_eq!(
            "_ _ _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_09_result_lab() {
        let mut maze: Maze = Maze::new("B1 _ _\n_ _ _\nB2 _ _");
        assert_eq!(
            "_ _ _\n_ _ _\nB2 _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_10_result_lab() {
        let mut maze: Maze = Maze::new("B3 _ _\n_ _ _\nB1 _ _");
        assert_eq!(
            "_ _ _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_11_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _\nB2 _ _\n_ _ _");
        assert_eq!(
            "_ _ _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_12_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 _ _\n_ _ _");
        assert_eq!(
            "_ B1 _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_13_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\n_ B3 _\n_ _ _");
        assert_eq!(
            "_ _ _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(1, 1))
        );
    }

    #[test]
    fn test_14_result_lab() {
        let mut maze: Maze = Maze::new("B2 B1 _\n_ _ _\n_ _ _");
        assert_eq!(
            "_ _ _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(1, 0))
        );
    }

    #[test]
    fn test_15_result_lab() {
        let mut maze: Maze = Maze::new("B2 B1 _ _\n_ B2 _ _\nB1 _ B3 _\n_ _ _ B2");
        assert_eq!(
            "B2 B1 _ _\n_ B2 _ _\n_ _ _ _\n_ _ _ B2".to_string(),
            resolve(maze.detonate(2, 2))
        );
    }

    #[test]
    fn test_16_result_lab() {
        let mut maze: Maze = Maze::new("B2 B1 _ _\n_ B2 _ _\nB1 _ B3 _\n_ _ _ B2");
        assert_eq!(
            "_ _ _ _\n_ _ _ _\n_ _ B3 _\n_ _ _ B2".to_string(),
            resolve(maze.detonate(1, 1))
        );
    }

    // BOMB, ROCK AND NOTHING

    #[test]
    fn test_17_result_lab() {
        let mut maze: Maze = Maze::new("B4 _ R B1");
        assert_eq!("_ _ R B1".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_18_result_lab() {
        let mut maze: Maze = Maze::new("B4 _ R B1");
        assert_eq!("B4 _ R _".to_string(), resolve(maze.detonate(3, 0)));
    }

    #[test]
    fn test_19_result_lab() {
        let mut maze: Maze = Maze::new("B2 R R\nB3 _ _\nB2 R _");
        assert_eq!(
            "_ R R\n_ _ _\n_ R _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_20_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 R _\n_ B3 R");
        assert_eq!(
            "_ B1 _\nB2 R _\n_ _ R".to_string(),
            resolve(maze.detonate(1, 2))
        );
    }

    #[test]
    fn test_21_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 R _\nB4 B3 R");
        assert_eq!(
            "_ B1 _\n_ R _\n_ _ R".to_string(),
            resolve(maze.detonate(0, 2))
        );
    }

    // BOMB, WALL, ROCK AND NOTHING

    #[test]
    fn test_22_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 R _\n_ B3 W");
        assert_eq!(
            "_ B1 _\nB2 R _\n_ _ W".to_string(),
            resolve(maze.detonate(1, 2))
        );
    }

    // BOMB, WALL, ROCK, PLAYER AND NOTHING

    #[test]
    fn test_23_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _\nB2 F1 _\n_ _ _");
        assert_eq!(
            "_ _ _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_24_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _\nB2 F3 _\n_ _ _");
        assert_eq!(
            "_ _ _\n_ F2 _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_25_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 F3 B1\n_ _ _");
        assert_eq!(
            "_ B1 _\n_ F1 _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_26_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 R F1\n_ _ _");
        assert_eq!(
            "_ B1 _\n_ R F1\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_27_result_lab() {
        let mut maze: Maze = Maze::new("_ B1 _\nB2 W F1\n_ _ _");
        assert_eq!(
            "_ B1 _\n_ W F1\n_ _ _".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    // BOMB, WALL, ROCK, PLAYER, DETOUR AND NOTHING

    #[test]
    fn test_28_result_lab() {
        let mut maze: Maze = Maze::new("_ B2 _\n_ DR F1\n_ _ _");
        assert_eq!(
            "_ _ _\n_ DR _\n_ _ _".to_string(),
            resolve(maze.detonate(1, 0))
        );
    }

    #[test]
    fn test_29_result_lab() {
        let mut maze: Maze = Maze::new("_ B2 _\n_ DR F1\n_ B1 _");
        assert_eq!(
            "_ _ _\n_ DR _\n_ B1 _".to_string(),
            resolve(maze.detonate(1, 0))
        );
    }

    #[test]
    fn test_30_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _\n_ F1 _\nB2 DU _");
        assert_eq!(
            "_ _ _\n_ _ _\n_ DU _".to_string(),
            resolve(maze.detonate(0, 2))
        );
    }

    #[test]
    fn test_31_result_lab() {
        let mut maze: Maze = Maze::new("F1 DL _\n_ B2 _\n_ _ _");
        assert_eq!(
            "_ DL _\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(1, 1))
        );
    }

    #[test]
    fn test_32_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _\nDD B2 _\nF1 _ _");
        assert_eq!(
            "_ _ _\nDD _ _\n_ _ _".to_string(),
            resolve(maze.detonate(1, 1))
        );
    }

    #[test]
    fn test_33_result_lab() {
        let mut maze: Maze = Maze::new("B3 _ DD\n_ _ F1\n_ _ _");
        assert_eq!(
            "_ _ DD\n_ _ _\n_ _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_34_result_lab() {
        let mut maze: Maze = Maze::new("B3 _ DD\n_ _ _\n_ _ F1");
        assert_eq!(
            "_ _ DD\n_ _ _\n_ _ F1".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_35_result_lab() {
        let mut maze: Maze = Maze::new("B4 _ _ DD\n_ _ _ F1\n_ _ _ _\n_ _ _ _");
        assert_eq!(
            "_ _ _ DD\n_ _ _ _\n_ _ _ _\n_ _ _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_36_result_lab() {
        let mut maze: Maze = Maze::new("B4 _ _ DD\n_ _ _ _\n_ _ _ F1\n_ _ _ _");
        assert_eq!(
            "_ _ _ DD\n_ _ _ _\n_ _ _ F1\n_ _ _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_37_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _ _\nDD _ B4 _\nDR F2 _ _\nF2 _ _ _");
        assert_eq!(
            "_ _ _ _\nDD _ _ _\nDR F1 _ _\nF2 _ _ _".to_string(),
            resolve(maze.detonate(2, 1))
        );
    }

    // SUPER BOMB, BOMB, WALL, ROCK, PLAYER, DETOUR AND NOTHING

    #[test]
    fn test_38_result_lab() {
        let mut maze: Maze = Maze::new("S4 _ R F1");
        assert_eq!("_ _ R _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_39_result_lab() {
        let mut maze: Maze = Maze::new("S4 B1 R F2");
        assert_eq!("_ _ R F1".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_40_result_lab() {
        let mut maze: Maze = Maze::new("S4 _ W F1");
        assert_eq!("_ _ W F1".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_41_result_lab() {
        let mut maze: Maze = Maze::new("S4 B1 W F2");
        assert_eq!("_ _ W F2".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_42_result_lab() {
        let mut maze: Maze = Maze::new("B4 _ _ F2\nB4 _ _ DU");
        assert_eq!(
            "_ _ _ _\n_ _ _ DU".to_string(),
            resolve(maze.detonate(0, 1))
        );
    }

    #[test]
    fn test_43_result_lab() {
        let mut maze: Maze = Maze::new("B3 B2\n F4 B3");
        assert_eq!("_ _\nF2 _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_44_result_lab() {
        let mut maze: Maze = Maze::new("B3 _\n F3 _");
        assert_eq!("_ _\nF2 _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_45_result_lab() {
        let mut maze: Maze = Maze::new("B3 B2\n F3 _");
        assert_eq!("_ _\nF2 _".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_46_result_lab() {
        let mut maze: Maze = Maze::new("B3 _\n F3 B3");
        assert_eq!("_ _\nF2 B3".to_string(), resolve(maze.detonate(0, 0)));
    }

    #[test]
    fn test_47_result_lab() {
        let mut maze: Maze = Maze::new("S5 F2 _ DL");
        assert_eq!("_ F1 _ DL".to_string(), resolve(maze.detonate(0, 0)));
    }

    // INTEGRAL TEST

    #[test]
    fn test_01_integral_result_lab() {
        let mut maze: Maze = Maze::new("B2 R R _ F1 _ _\n_ W R W _ W _\nB5 _ _ _ B2 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(
            "_ R R _ _ _ _\n_ W R W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string(),
            resolve(maze.detonate(0, 0))
        );
    }

    #[test]
    fn test_02_integral_result_lab() {
        let mut maze: Maze = Maze::new("_ _ B2 _ B1 _ _\n_ W _ W _ W _\n_ _ B2 R F1 _ _\n_ W _ W R W _\n_ _ B4 _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1");
        assert_eq!(
            "_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ R F1 _ _\n_ W _ W R W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1".to_string(),
            resolve(maze.detonate(2, 4))
        );
    }

    #[test]
    fn test_03_integral_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _ _ _ _ _\n_ W _ W _ W _\nS4 R R R F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(
            "_ _ _ _ _ _ _\n_ W _ W _ W _\n_ R R R _ _ _\n_ W _ W _ W _\n_ _ _ _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string(),
            resolve(maze.detonate(0, 4))
        );
    }

    #[test]
    fn test_04_integral_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _ _ _ _ _\n_ W _ W _ W _\nB4 _ _ _ F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(
            "_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string(),
            resolve(maze.detonate(0, 4))
        );
    }

    #[test]
    fn test_05_integral_result_lab() {
        let mut maze: Maze = Maze::new("_ _ _ _ _ _ _\n_ W _ W _ W _\nB4 _ _ _ F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(
            "ERROR: The position is not includes in the maze".to_string(),
            resolve(maze.detonate(9, 9))
        );
    }
}
