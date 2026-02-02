use tauri::async_runtime::channel;

use crate::CAMERA;

#[tauri::command]
pub async fn take_photo() -> Result<String, String> {
    let (tx, mut rx) = channel(1);

    let _ = CAMERA.lock().await.as_ref().unwrap().take_picture(tx);

    let result = rx.recv().await.expect("Channel has hung up");

    result
}
