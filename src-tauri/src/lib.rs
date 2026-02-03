use std::sync::LazyLock;

use tauri::{async_runtime::Mutex, Manager};

use crate::{
    camera::{camera_event_thread, CameraRef},
    path::init_dirs,
};

mod camera;
mod commands;
mod path;

const ENABLE_LIVE_VIEW: bool = false;

pub static CAMERA: LazyLock<Mutex<Option<CameraRef>>> = LazyLock::new(|| Mutex::new(None));

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();

    println!(
        "Using MAIL_ADDRESS: {}",
        std::env::var("PHOTOBOOTH_MAIL_ADDRESS").unwrap()
    );

    let cam = CameraRef::init(ENABLE_LIVE_VIEW).expect("Failed to initialize camera");
    {
        let mut global_cam = CAMERA.try_lock().expect("Failed to lock camera");
        *global_cam = Some(cam);
    }

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

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn_blocking(move || {
                camera_event_thread(app_handle, ENABLE_LIVE_VIEW)
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::send_mail,
            commands::take_photo
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, event| match event {
            tauri::RunEvent::Exit => {
                println!("Exiting, dropping camera");
                let _ = CAMERA.blocking_lock().take();
            }
            _ => {}
        });
}
