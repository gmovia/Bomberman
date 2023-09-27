use std::fs;
use std::io::{self, Read};
pub struct Reader;

impl Reader {
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
