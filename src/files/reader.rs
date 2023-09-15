use std::fs;
use std::io::{self, Read};

pub struct Reader;

impl Reader {
    pub fn read(path: &str, content: &mut String) -> Result<(), io::Error> {
        match fs::File::open(path) {
            Ok(mut file) => {
                if let Err(e) = file.read_to_string(content) {
                    return Err(e);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
