use crate::element::element::Element;
use crate::types::position::Position;
use crate::utils::maker::Maker;
// Representa un convertidor.
pub struct Converter;
impl Converter {
    // Convierte un string en una matriz de string.
    pub fn string_to_matrix(maze: &str) -> Vec<Vec<String>> {
        let mut matrix: Vec<Vec<String>> = Vec::new();
        let rows: Vec<String> = maze.split('\n').map(String::from).collect();
        for column in rows {
            matrix.push(column.split_whitespace().map(String::from).collect())
        }
        matrix
    }
    // Convierte un string en una matriz de objetos del mapa.
    pub fn string_to_matrix_object(maze: &str) -> Vec<Vec<Element>> {
        let mut new_matrix = Vec::new();
        let matrix = Self::string_to_matrix(maze);
        for (index_row, row) in matrix.iter().enumerate() {
            let mut new_row_matrix = Vec::new();
            for (index_column, element) in row.iter().enumerate() {
                new_row_matrix.push(Maker::make(
                    element.as_str(),
                    Position::new(index_column, index_row),
                ));
            }
            new_matrix.push(new_row_matrix);
        }
        new_matrix
    }
    // Obtengo una matriz de string en un string.
    pub fn matrix_to_string(maze: &Vec<Vec<String>>) -> String {
        let mut string_maze: String = String::new();
        for (index, row) in maze.iter().enumerate() {
            string_maze.push_str(&row.join(" "));
            if index != maze.len() - 1 {
                string_maze.push_str("\n");
            }
        }
        string_maze
    }
    // Obtengo una matriz de elementos en un string.
    pub fn matrix_object_to_string(maze: &Vec<Vec<Element>>) -> String {
        let mut string_maze = String::new();

        for (index, row) in maze.iter().enumerate() {
            for (other_index, element) in row.iter().enumerate() {
                string_maze.push_str(&element.code());
                if other_index != row.len() - 1 {
                    string_maze.push(' ');
                }
            }
            if index != maze.len() - 1 {
                string_maze.push('\n');
            }
        }
        string_maze
    }
}
