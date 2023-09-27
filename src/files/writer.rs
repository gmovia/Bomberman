use std::fs;
use std::io::{self, Write};
pub struct Writer;
/// Tiene como responsabilidad aquellas funciones encargadas de escribir.
impl Writer {
    /// Recibe la ruta de un archivo y una variable, y escribe el contenido de la variable en el archivo.
    pub fn write(path: &str, content: &String) -> Result<(), io::Error> {
        match fs::File::create(path) {
            Ok(mut file) => {
                file.write_all(content.as_bytes())?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
