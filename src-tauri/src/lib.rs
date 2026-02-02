use std::sync::{mpsc, OnceLock};

use tauri::Manager;

use crate::{camera::start_camera_thread, comm::CommRequestDispatcher, path::init_dirs};

mod camera;
mod comm;
mod commands;
mod path;

pub static COMM: OnceLock<CommRequestDispatcher> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    println!(
        "Using MAIL_ADDRESS: {}",
        std::env::var("MAIL_ADDRESS").unwrap()
    );

    let (tx, rx) = mpsc::channel();

    let dispatcher = CommRequestDispatcher::new(tx);
    let _ = COMM.set(dispatcher);

    tauri::async_runtime::spawn_blocking(|| start_camera_thread(rx));

    tauri::Builder::default()
        .on_window_event(|_, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;

            init_dirs(&app_data_dir)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::send_mail,
            commands::take_photo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
