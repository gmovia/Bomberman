use taller1_tp1_bomberman::element::Element;
#[cfg(test)]
use taller1_tp1_bomberman::maker::Maker;

mod tests {

    use super::*;

    #[test]
    fn test_01_create_empty() {
        let element: Element = Maker::make("_");
        assert_eq!('_', element.typef());
    }

    #[test]
    fn test_02_create_bomb() {
        let element: Element = Maker::make("B2");
        assert_eq!('B', element.typef());
    }

    #[test]
    fn test_03_create_rock() {
        let element: Element = Maker::make("R");
        assert_eq!('R', element.typef());
    }

    #[test]
    fn test_04_create_wall() {
        let element: Element = Maker::make("W");
        assert_eq!('W', element.typef());
    }
}
