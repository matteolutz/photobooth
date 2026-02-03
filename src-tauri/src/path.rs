use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub static CAMERA_PHOTO_DIR: OnceLock<PathBuf> = OnceLock::new();
pub static PHOTO_STRIP_DIR: OnceLock<PathBuf> = OnceLock::new();
pub static EVF_IMAGE: OnceLock<PathBuf> = OnceLock::new();

pub fn init_dirs(app_data_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    CAMERA_PHOTO_DIR
        .set(app_data_dir.join("camera"))
        .map_err(|_| "Failed to set camera photo directory".to_string())?;

    PHOTO_STRIP_DIR
        .set(app_data_dir.join("strip"))
        .map_err(|_| "Failed to set photo strip directory".to_string())?;

    EVF_IMAGE
        .set(app_data_dir.join("evf.jpeg"))
        .map_err(|_| "Failedto set evf image file path".to_string())?;

    std::fs::create_dir_all(CAMERA_PHOTO_DIR.get().unwrap())?;
    std::fs::create_dir_all(PHOTO_STRIP_DIR.get().unwrap())?;

    Ok(())
}
