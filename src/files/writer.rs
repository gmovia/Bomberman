use std::fs;
use std::io::{self, Write};
pub struct Writer;

impl Writer {
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
