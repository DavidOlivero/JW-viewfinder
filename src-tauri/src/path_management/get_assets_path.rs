use std::{env, path::PathBuf};

pub fn assets_path<'a>(epub_folder: &str, folder_name: &str) -> Result<PathBuf, &'a str> {
    let directory_error = "No se puedo obtener el directorio necesario para almacenar la covertura del libro";
    
    // Get the current path
    let current_dir = match env::current_dir() {
        Err(_) => return Err(&directory_error),
        Ok(val) => val
    };

    // Get assets path
    let root_dir = match current_dir.parent() {
        Some(val) => val,
        _ => return Err(&directory_error)
    };
    
    Ok(root_dir.join(format!("{epub_folder}\\books\\files\\{folder_name}\\")))
}