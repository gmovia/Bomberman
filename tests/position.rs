#[cfg(test)]

mod tests {

    use taller1_tp1_bomberman::types::position::Position;

    #[test]
    fn test_01_create_position() {
        let position = Position::new(2, 1);
        assert_eq!(2, position.x);
        assert_eq!(1, position.y);
    }

    #[test]
    fn test_02_position_is_equals() {
        let position = Position::new(2, 1);
        let other_position = Position::new(2, 1);
        assert_eq!(true, position.equals(other_position));
    }

    #[test]
    fn test_03_position_is_not_equals() {
        let position = Position::new(2, 1);
        let other_position = Position::new(2, 3);
        assert_eq!(false, position.equals(other_position));
    }

    #[test]
    fn test_04_position_right_is_ok() {
        let position = Position::new(2, 1);
        let position_right = Position::new(3, 1);
        assert_eq!(true, position.right().equals(position_right));
    }

    #[test]
    fn test_05_position_left_is_ok() {
        let position = Position::new(2, 1);
        let position_left = Position::new(1, 1);
        assert_eq!(true, position.left().equals(position_left));
    }

    #[test]
    fn test_06_position_up_is_ok() {
        let position = Position::new(2, 1);
        let position_up = Position::new(2, 0);
        assert_eq!(true, position.up().equals(position_up));
    }

    #[test]
    fn test_07_position_down_is_ok() {
        let position = Position::new(2, 1);
        let position_down = Position::new(2, 2);
        assert_eq!(true, position.down().equals(position_down));
    }
}
