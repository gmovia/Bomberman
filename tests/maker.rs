#[cfg(test)]
use taller1_tp1_bomberman::maker::Maker;
use taller1_tp1_bomberman::maker::Element;

mod tests {
    

    use super::*;

    #[test]
    fn test_01_create_rock() {
        let element: Element = Maker::make("_");
        assert_eq!('_', element.typef());
    }

    #[test]
    fn test_02_create_bomb() {
        let element: Element = Maker::make("B2");
        assert_eq!('B', element.typef());
    }
}   