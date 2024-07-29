use super::super::path_management::get_assets_path;
use std::fs::read_dir;

pub async fn nav_page(folder_name: &str) -> Result<String, &str> {
    let folder_path = match get_assets_path::assets_path(folder_name) {
        Err(err) => return Err(err),
        Ok(val) => val
    };

    let files = match read_dir(&folder_path.join("OEBPS")) {
        Err(_) => return Err("No fue posible leer los fichero necesarios del libro, asegúrese de haberlo cargado correctamente"),
        Ok(val) => val
    };

    let nav_page: Vec<String> = files.filter_map(|file| {
        let name = file.ok().and_then(|value| {
            value.file_name().into_string().ok()
        });
        
        if let Some(value) = name {
            if value.contains("nav") && !value.contains("ver") && !value.contains("chapter") {
                return Some(value)
            }

            return None
        };

        None
    }).collect();

    Ok(nav_page[0].to_owned())
}