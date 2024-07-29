use std::{fs::{self, File}, io::copy, path::Path};
use zip::ZipArchive;
use super::super::path_management::get_assets_path;

pub enum ProcessResult<'a> {
    Error(&'a str),
    Right
}

pub async fn unzip_epub_files(epub_file: &str, folder_name: &str) -> Result<bool, &'static str> {
    // Make the full path
    let path = match get_assets_path::assets_path(folder_name) {
        Err(err) => return Err(err),
        Ok(path) => path
    };
    
    // Open the epub file
    let file = match File::open(&epub_file) {
        Err(_) => return Err("No se pudo acceder a la ruta especificada. Verifique que el fichero tenga permisos de lectura"),
        Ok(f) => f
    };

    // Unzip epub file
    let mut pages = match ZipArchive::new(file) {
        Err(_) => return Err("No se puedo descomprimir el fichero, verifique que no este dañado o currupto"),
        Ok(p) => p
    };
    
    // Loop for read the pages for saved
    for i in 0..pages.len() {
        // Get unzip file
        let mut file = match pages.by_index(i) {
            Err(_) => return Err("No se pudo encontrar una de las páginas"),
            Ok(f) => f
        };

        // Get full file path
        let outpath = Path::new(&path).join(file.mangled_name());

        // Comprobate if it's a folder
        if file.is_dir() {
            if let Err(_) = fs::create_dir_all(&path) {
                return Err("No se pudo crear una de las carpetas del fichero epub");
            };
        } 
        else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    if let Err(_) = fs::create_dir_all(&p) {
                       return Err("No se pudo crear las carpetas necesarias para descomprimir el fichero epub");
                    }
                }
            }
        }

        // Make the file
        let mut outfile = match File::create(&outpath) {
            Err(_) => return Err("No se pudo crear uno de los ficheros necesarios del epub"),
            Ok(f) => f
        };

        // Copy the file content to the outfile
        if let Err(_) = copy(&mut file, &mut outfile) {
            return Err("No se pudo copiar el contenido del epub a la ruta destino");
        }
    }

    Ok(true)
}