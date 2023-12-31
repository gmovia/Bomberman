use std::env;
use taller1_tp1_bomberman::constants::consts::{ERR_ARGS, ERR_INVALID_X, ERR_INVALID_Y, ERR_READ};
use taller1_tp1_bomberman::files::reader::Reader;
use taller1_tp1_bomberman::files::writer::Writer;
use taller1_tp1_bomberman::lab::Maze;
use taller1_tp1_bomberman::utils::error::error;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content: String = String::new();
    let mut result: String = String::new();

    if args.len() != 5 {
        println!("{}", ERR_ARGS);
    }

    let input = Path::new(&args[1]);
    let path_input = &args[1];
    let path_output = &args[2];
    let mut file_name = "";

    if let Some(name) = input.file_name(){
        if let Some(name_str) = name.to_str(){
            file_name = name_str;
        }
    }
    
    let concatenated_path = format!("{}{}", path_output, file_name);
    
    let x: usize = match args[3].parse() {
        Ok(x) => x,
        Err(_) => {
            error(ERR_INVALID_X, &concatenated_path);
            return;
        }
    };
    
    let y: usize = match args[4].parse() {
        Ok(y) => y,
        Err(_) => {
            error(ERR_INVALID_Y, &concatenated_path);
            return;
        }
    };
    
    match Reader::read(path_input, &mut content) {
        Ok(()) => println!("Lectura exitosa"),
        Err(_) => {
            error(ERR_READ, &concatenated_path);
            return;
        }
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
