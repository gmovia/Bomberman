#[cfg(test)]

mod tests {

    use taller1_tp1_bomberman::types::position::Position;
    use taller1_tp1_bomberman::utils::converter::Converter;
    use taller1_tp1_bomberman::utils::maker::Maker;

    #[test]
    fn test_01_converter_string_to_matrix() {
        let string = "_ _\n_ R";
        let matrix = Converter::string_to_matrix(string);
        assert_eq!("_".to_string(), matrix[0][0]);
        assert_eq!("_".to_string(), matrix[0][1]);
        assert_eq!("_".to_string(), matrix[1][0]);
        assert_eq!("R".to_string(), matrix[1][1]);
    }

    #[test]
    fn test_02_converter_matrix_to_string() {
        let string = "_ _\n_ R";
        let matrix = Converter::string_to_matrix(string);       
        let new_string = Converter::matrix_to_string(&matrix);
        assert_eq!(string, new_string);
    }

    #[test]
    fn test_03_matrix_object_to_string() {
        let string = "_ _\n_ _";
        let matrix = vec![
            vec![Maker::new_empty(Position::new(0, 0)), Maker::new_empty(Position::new(1, 0))],
            vec![Maker::new_empty(Position::new(0, 1)), Maker::new_empty(Position::new(1, 1))],
        ];
        let result = Converter::matrix_object_to_string(&matrix);
        assert_eq!(string, result)
    }

    #[test]
    fn test_04_string_to_matrix_object() {
        let string = "_ _\n_ _";
        let matrix = vec![
            vec![Maker::new_empty(Position::new(0, 0)), Maker::new_empty(Position::new(1, 0))],
            vec![Maker::new_empty(Position::new(0, 1)), Maker::new_empty(Position::new(1, 1))],
        ];
        let result = Converter::string_to_matrix_object(string);
        assert_eq!(matrix[0][0].code(), result[0][0].code());
        assert_eq!(matrix[0][1].code(), result[0][1].code());
        assert_eq!(matrix[1][0].code(), result[1][0].code());
        assert_eq!(matrix[1][1].code(), result[1][1].code());
    }
}
