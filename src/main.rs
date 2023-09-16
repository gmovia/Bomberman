use std::env;
use taller1_tp1_bomberman::files::reader::Reader;
use taller1_tp1_bomberman::files::writer::Writer;
use taller1_tp1_bomberman::lab::Maze;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content: String = String::new();
    let mut result: String = String::new();

    if args.len() != 5 {
        println!("ERR: No se enviaron los parametros correspondientes")
    }

    let path_input = &args[1];
    let path_output = &args[2];
    let concatenated_path = format!("{}{}", path_output, path_input);
    let x: usize = args[3].parse().expect("ERR: Position X invalid");
    let y: usize = args[4].parse().expect("ERR: Position Y invalid");

    match Reader::read(&path_input, &mut content) {
        Ok(()) => println!("Lectura exitosa"),
        Err(e) => {
            match Writer::write(&concatenated_path, &format!("Error al leer archivo: {}", e)) {
                Ok(()) => println!("Error escrito en archivo"),
                Err(e) => eprintln!("Error al escribir en archivo: {}", e),
            }
            return;
        },
    }

    let new_content = content.trim_end_matches('\n');
    let mut maze: Maze = Maze::new(new_content);

    match maze.detonate(x, y) {
        Ok(r) => result.push_str(r.as_str()),
        Err(e) => result.push_str(e.as_str()),
    }

    match Writer::write(&concatenated_path, &result) {
        Ok(()) => println!("Escritura exitosa"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
