use std::path::PathBuf;
use dirs;

pub fn assets_path(folder_name: &str) -> Result<PathBuf, &'static str> {
    // Get the user path
    let user_dir = match dirs::home_dir() {
        None => return Err("No se puedo obtener el directorio necesario para almacenar la covertura del libro"),
        Some(val) => val
    };
    
    Ok(user_dir.join(format!("JWviewfinder\\books\\files\\{folder_name}\\")))
}