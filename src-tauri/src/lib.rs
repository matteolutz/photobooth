use std::{cell::RefCell, path::PathBuf, time::SystemTime};

use chrono::{DateTime, Local};
use tauri::Manager;

thread_local! {
    static APP_DATA_DIR: RefCell<Option<PathBuf>> = RefCell::new(None);
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn send_mail(_mail_address: String, image: Vec<u8>) {
    let app_data_dir = APP_DATA_DIR.with_borrow(|path| path.clone());

    let Some(app_data_dir) = app_data_dir else {
        println!("App data directory not initialized");
        return;
    };

    let now: DateTime<Local> = SystemTime::now().into();
    let img_name = format!("{}.png", now.format("%d-%m-%Y %H-%M-%S"));
    let img_path = app_data_dir.join(img_name);

    println!("Writing to {}", img_path.display());

    std::fs::write(&img_path, image).unwrap();

    println!("Wrote to {}", img_path.display());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path().app_data_dir()?;
            std::fs::create_dir_all(&path)?;

            APP_DATA_DIR.replace(Some(path));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_mail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
