// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use] extern crate rocket;

pub mod book_management {
    pub mod file_decompression;
    pub mod extract_cover;
    pub mod add_selection_handle;
    pub mod extract_nav_page;
}

pub mod utilities_commands {
    pub mod tauri_utilities;
}

pub mod path_management {
    pub mod get_assets_path;
}

pub mod windows {
    pub mod viewfinder_book;
}

pub mod server {
    pub mod files_server;
}

use tauri::WindowEvent;
use utilities_commands::tauri_utilities::{upload_book, get_title, get_nav_page, start_server};
use windows::viewfinder_book::viewfinder;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![upload_book, get_title, viewfinder, get_nav_page, start_server])
        .on_window_event(|event| {
            if let WindowEvent::Resized(_) = event.event() {
                let win = event.window();
                if win.is_maximized().unwrap() && win.label().contains("viewfinder") {
                    win.set_fullscreen(true).unwrap()
                } else {
                    win.set_fullscreen(false).unwrap()
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, _| {});
}
