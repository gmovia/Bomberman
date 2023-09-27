use std::fs;
use std::io::{self, Read};
pub struct Reader;
/// Tiene como responsabilidad aquellas funciones encargadas de leer.
impl Reader {
    /// Recibe la ruta de un archivo y una variable, y almacena el contenido del archivo en esa variable.
    pub fn read(path: &str, content: &mut String) -> Result<(), io::Error> {
        match fs::File::open(path) {
            Ok(mut file) => {
                file.read_to_string(content)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
