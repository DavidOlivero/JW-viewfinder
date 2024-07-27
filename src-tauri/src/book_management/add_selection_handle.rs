use super::super::path_management::get_assets_path;
use std::fs::{self, read_dir, File};
use std::io::Read;

pub async fn add_htmx_selection_handle<'a>(epub_folder: &str, folder_name: &str) -> Result<(), &'a str> {
    let path = match get_assets_path::assets_path(epub_folder, folder_name) {
        Err(err) => return Err(err),
        Ok(val) => val
    };

    let path = path.join("OEBPS\\");
    let files = match read_dir(&path) {
        Err(_) => return Err("No se pudo leer todos los ficheros del libro, asegúrese de haber cargado el libro correctamente"),
        Ok(val) => val
    };

    let files_list = files.filter_map(|val| {
        val.ok().and_then(|e| {
            let name = e.file_name().into_string().ok();
            if let Some(value) = name {
                if !value.contains("xhtml") {
                    return None;
                }

                return Some(value)
            }

            None
        })
    });

    for file in files_list {
        let mut file_for_overwrite = match File::open(&path.join(&file)) {
            Err(_) => return Err("No se puedo leer los ficheros necesarios del libro"),
            Ok(val) => val
        };
        
        let mut content: String = String::new();
        match file_for_overwrite.read_to_string(&mut content) {
            Err(_) => return Err("No se pudo leer el contenido de los ficheros del libro"),
            _ => ()
        }

        let mut new_content: String = String::new();
        for text in content.lines() {
            if text.eq_ignore_ascii_case("</body>") {
                new_content.push_str(
                    "</body>
                    <script>
                        document.addEventListener('mouseup', () => {
                            const selection = window.getSelection();
                            const selectedText = selection.toString();
                            
                            if (selectedText.length > 0) {
                                const infoText = { text: selectedText, show: false }
                                window.parent.postMessage({ type: 'text', text: selectedText }, '*')
                            }
                        });
                    </script>
                    </html>"
                );
                break;
            } else {
                new_content.push_str(text);
            }
        }

        if let Err(_) = fs::write(path.join(&file), &new_content) {
            return Err("No se pudo sobrescribir información necesaria en los ficheros del libro");
        };
    }

    return Ok(());
}