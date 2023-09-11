use taller1_tp1_bomberman::element::element::Element;
#[cfg(test)]
use taller1_tp1_bomberman::utils::maker::Maker;

mod tests {

    use taller1_tp1_bomberman::types::position::Position;

    use super::*;

    #[test]
    fn test_01_create_empty() {
        let element: Element = Maker::make("_", Position::new(0, 0));
        assert_eq!('_', element.typef());
    }

    #[test]
    fn test_02_create_bomb() {
        let element: Element = Maker::make("B2", Position::new(0, 0));
        assert_eq!('B', element.typef());
    }

    #[test]
    fn test_03_create_rock() {
        let element: Element = Maker::make("R", Position::new(0, 0));
        assert_eq!('R', element.typef());
    }

    #[test]
    fn test_04_create_wall() {
        let element: Element = Maker::make("W", Position::new(0, 0));
        assert_eq!('W', element.typef());
    }

    #[test]
    fn test_05_create_player() {
        let element: Element = Maker::make("F2", Position::new(0, 0));
        assert_eq!('F', element.typef());
    }
}
