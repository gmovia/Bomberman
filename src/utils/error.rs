use crate::files::writer::Writer;

pub fn error(message: &str, path: &String) {
    if let Err(e) = Writer::write(path, &message.to_string()) {
        eprintln!("Error al escribir en archivo: {}", e);
    }
}
