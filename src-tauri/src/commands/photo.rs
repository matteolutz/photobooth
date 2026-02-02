use crate::{camera::TakePictureRequest, COMM};

#[tauri::command]
pub async fn take_photo() -> Result<String, String> {
    let result = COMM.get().unwrap().send(TakePictureRequest).await?;

    println!("got result: {:?}", result);

    result
}
