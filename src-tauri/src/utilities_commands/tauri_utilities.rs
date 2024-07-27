use super::super::book_management::{file_decompression, extract_cover, add_selection_handle, extract_nav_page};
use epub::doc::EpubDoc;
use std::process::{Command, Stdio};

#[tauri::command]
pub fn get_title(epub_file: &str) -> Result<String, &str> {
    let book = match EpubDoc::new(epub_file) {
        Err(_) => return Err("No se pudo abrir el libro, por favor verifique la integridad de este"),
        Ok(val) => val
    };

    match book.mdata("title") {
        Some(val) => return Ok(val),
        None => return Ok("Sin título".to_string())
    }
}

#[tauri::command]
pub async fn upload_book(epub_folder: &str, epub_file: &str, folder_name: &str) -> Result<bool, String> {
    // Unzip the epub
    if let Err(err) = file_decompression::unzip_epub_files(epub_folder, epub_file, folder_name).await {
        return Err(err.to_string());
    };

    // Get cover image
    if let Err(err) = extract_cover::get_epub_cover(epub_folder, epub_file, folder_name) {
        return Err(err.to_string())
    };

    // Edit all files for overwrite
    if let Err(err) = add_selection_handle::add_htmx_selection_handle(epub_folder, folder_name).await {
        return Err(err.to_string())
    }

    Ok(true)
}

#[tauri::command]
pub async fn get_nav_page(epub_folder: &str, folder_name: &str) -> Result<String, String> {
    let nav_page = extract_nav_page::nav_page(epub_folder, folder_name).await;

    match nav_page {
        Err(err) => return Err(err.to_string()),
        Ok(val) => return Ok(val)
    }
}

#[tauri::command]
pub fn start_server(mut local_data_dir: String) {
    println!("Iniciando servidor");
    
    local_data_dir.push_str("books\\main.js");
    
    Command::new("node")
        .arg(&local_data_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
}