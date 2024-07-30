use std::path::PathBuf;

use rocket::{fairing::AdHoc, fs::FileServer, http::Header, Config};
use dirs;

#[launch]
pub fn rocket() -> _ {
    let resources_path = get_resources_path();
    let figment = Config::figment()
        .merge(("secret_key", vec![0u8; 64]))
        .select("debug");

    rocket::custom(figment)
        .mount("/resources", FileServer::from(resources_path))
        .attach(AdHoc::on_response("Add Headers", |_, response| Box::pin(async move {
            response.set_header(Header::new("X-Frame-Options", "ALLOWALL"));
            response.set_header(Header::new("Content-Security-Policy", "frame-ancestors *;"));
        })))
}

fn get_resources_path() -> PathBuf {
    if let Some(dir) = dirs::home_dir() {
        let user_dir = dir.join("JWviewfinder\\books\\files");
        
        return user_dir;
    }

    PathBuf::new()
}