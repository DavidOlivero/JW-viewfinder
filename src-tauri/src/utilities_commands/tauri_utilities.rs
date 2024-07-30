use super::super::book_management::{file_decompression, extract_cover, add_selection_handle, extract_nav_page};
use super::super::server::files_server;
use epub::doc::EpubDoc;

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
pub async fn upload_book(epub_file: &str, folder_name: &str) -> Result<bool, String> {
    // Unzip the epub
    if let Err(err) = file_decompression::unzip_epub_files(epub_file, folder_name).await {
        return Err(err.to_string());
    };

    // Get cover image
    if let Err(err) = extract_cover::get_epub_cover(epub_file, folder_name) {
        return Err(err.to_string())
    };

    // Edit all files for overwrite
    if let Err(err) = add_selection_handle::add_htmx_selection_handle(folder_name).await {
        return Err(err.to_string())
    }

    Ok(true)
}

#[tauri::command]
pub async fn get_nav_page(folder_name: &str) -> Result<String, String> {
    let nav_page = extract_nav_page::nav_page(folder_name).await;

    match nav_page {
        Err(err) => return Err(err.to_string()),
        Ok(val) => return Ok(val)
    }
}

#[tauri::command]
pub async fn start_server() -> Result<(), &'static str> {
    match files_server::rocket().launch().await {
        Err(_) => return Err("No fue posible iniciar el servidor"),
        _ => Ok(())
    }
}