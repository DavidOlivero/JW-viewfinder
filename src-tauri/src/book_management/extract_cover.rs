use epub::doc::EpubDoc;
use std::fs::{self, File};
use std::io::Write;
use super::super::path_management::get_assets_path;

pub fn get_epub_cover<'a>(epub_folder: &str, epub_file: &str, folder_name: &str) -> Result<(), &'a str> {
    let image_path = match get_assets_path::assets_path(epub_folder, folder_name) {
        Err(err) => return Err(err),
        Ok(val) => val
    };
    
    if !image_path.exists() {
        if let Err(_) = fs::create_dir(&image_path) {
            return Err("No se puedo obtener el directorio necesario para almacenar la covertura del libro");
        };
    }
    
    let image_path = image_path.join("cover.png");
    
    // Load the epub book
    let mut book = match EpubDoc::new(epub_file) {
        Err(_) => return Err("No se pudo leer el contenido del libro para extraer la covertura, asegúrese que el libro no esté corrupto"),
        Ok(val) => val
    };

    // Get cover
    let cover = match book.get_cover() {
        Some(val) => val.0,
        _ => return Err("No se pudo extraer la convertura del libro")
    };

    // Make the file
    let mut file = File::create(&image_path).unwrap();

    // Overwrite the bytes to the image
    if let Err(_) = file.write_all(&cover) {
        return Err("No se pudo sobreescribir la información de la covertura en el fichero destino");
    };

    Ok(())
}