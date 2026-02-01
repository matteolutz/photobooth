#[tauri::command]
pub async fn take_photo() -> Result<String, String> {
    let photo_id = rand::random_range(1..=4);
    let photo_file = format!("{}.jpg", photo_id);

    Ok(photo_file)
}
