use taller1_tp1_bomberman::elements::element::Element;
#[cfg(test)]
use taller1_tp1_bomberman::utils::maker::Maker;

mod tests {

    use taller1_tp1_bomberman::types::position::Position;

    use super::*;

    #[test]
    fn test_01_create_empty() {
        let element: Element = Maker::new_empty(Position::new(0, 0));
        assert_eq!('_', element.typef());
    }

    #[test]
    fn test_02_create_bomb() {
        let element: Element = Maker::new_bomb(2, Position::new(0, 0));
        assert_eq!('B', element.typef());
    }

    #[test]
    fn test_03_create_rock() {
        let element: Element = Maker::new_rock(Position::new(0, 0));
        assert_eq!('R', element.typef());
    }

    #[test]
    fn test_04_create_wall() {
        let element: Element = Maker::new_wall(Position::new(0, 0));
        assert_eq!('W', element.typef());
    }

    #[test]
    fn test_05_create_player() {
        let element: Element = Maker::new_player(3, Position::new(0, 0));
        assert_eq!('F', element.typef());
    }

    #[test]
    fn test_06_create_super_bomb() {
        let element: Element = Maker::new_super_bomb(3, Position::new(0, 0));
        assert_eq!('S', element.typef());
    }
}
