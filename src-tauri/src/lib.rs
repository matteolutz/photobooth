use tauri::Manager;

use crate::path::init_dirs;

mod commands;
mod path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    println!(
        "Using MAIL_ADDRESS: {}",
        std::env::var("MAIL_ADDRESS").unwrap()
    );

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
