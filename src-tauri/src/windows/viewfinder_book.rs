use tauri::AppHandle;

#[tauri::command]
pub async fn viewfinder(app: AppHandle) -> Result<(), &'static str> {
    let window = tauri::WindowBuilder::new(
        &app, 
        "viewfinder", 
        tauri::WindowUrl::App("/viewfinderwindow".into())
    )
    .title("Viewfinder")
    .inner_size(1200.0, 800.0)
    .maximizable(true)
    .build();

    if let Err(_) = window {
        return Err("No fue posible abrir el visualizador");
    };

    Ok(())
}