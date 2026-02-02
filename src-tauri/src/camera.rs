use std::{
    ffi::CString,
    ptr::null_mut,
    sync::{LazyLock, Mutex},
    thread,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};
use edsdk::{
    EdsBaseRef, EdsCameraListRef, EdsCameraRef, EdsCapacity, EdsCloseSession, EdsCreateFileStream,
    EdsDeviceInfo, EdsDirectoryItemInfo, EdsDirectoryItemRef, EdsDownload, EdsDownloadComplete,
    EdsError, EdsGetCameraList, EdsGetChildAtIndex, EdsGetChildCount, EdsGetDeviceInfo,
    EdsGetDirectoryItemInfo, EdsGetEvent, EdsImageQuality, EdsInitializeSDK, EdsObjectEvent,
    EdsOpenSession, EdsRelease, EdsSaveTo, EdsSendCommand, EdsSetCapacity,
    EdsSetObjectEventHandler, EdsSetPropertyData, EdsStreamRef, EdsTerminateSDK, EdsVoid,
};
use tauri::async_runtime::Sender;

use crate::path::CAMERA_PHOTO_DIR;

/// Global channel to receive the filename from the callback
static PHOTO_RESULT_SENDER: LazyLock<Mutex<Option<Sender<Result<String, String>>>>> =
    LazyLock::new(|| Mutex::new(None));

pub struct CameraRef(EdsCameraRef);

impl From<EdsCameraRef> for CameraRef {
    fn from(camera: EdsCameraRef) -> Self {
        Self(camera)
    }
}

impl CameraRef {
    pub fn init() -> Option<Self> {
        unsafe { initialize_edsdk() }
    }

    pub fn take_picture(&self, respond_to: Sender<Result<String, String>>) {
        let camera = self.0;

        *PHOTO_RESULT_SENDER.lock().unwrap() = Some(respond_to);

        let err = unsafe { EdsSendCommand(camera, 4, 3) }; // 4 = shutter command, 3 = shutter completely
        unsafe { EdsSendCommand(camera, 4, 0) }; // 0 = shutter off
        assert!(err.is_ok());
    }
}

impl Drop for CameraRef {
    fn drop(&mut self) {
        unsafe { EdsCloseSession(self.0) };
        unsafe { EdsTerminateSDK() };
    }
}

unsafe impl Send for CameraRef {}
unsafe impl Sync for CameraRef {}

#[no_mangle]
extern "C" fn event_handler(
    event: EdsObjectEvent,
    in_ref: EdsBaseRef,
    _context: *mut EdsVoid,
) -> EdsError {
    println!("received event: {:x}", event);
    match event {
        // DirItemCreated | DirItemRequestTransfer
        0x204 | 0x208 => {
            let Some(sender) = PHOTO_RESULT_SENDER.lock().unwrap().take() else {
                return EdsError::Ok;
            };

            println!("received DirItemCreated/DirItemRequestTransfer event");
            let directory_item = in_ref as EdsDirectoryItemRef;

            println!("getting directory info");
            let mut dir_item_info = EdsDirectoryItemInfo::default();
            let err = unsafe { EdsGetDirectoryItemInfo(directory_item, &mut dir_item_info) };
            if !err.is_ok() {
                sender
                    .blocking_send(Err(format!("Failed to get directory info: {:?}", err)))
                    .unwrap();
                return err;
            }

            // Get the camera photo directory and create the full path
            let camera_dir = CAMERA_PHOTO_DIR
                .get()
                .expect("CAMERA_PHOTO_DIR not initialized");

            let now: DateTime<Local> = SystemTime::now().into();
            let file_name = format!("{}.jpeg", now.format("%d-%m-%Y %H-%M-%S"));

            let full_path = camera_dir.join(&file_name);
            let full_path_str = full_path.to_string_lossy().to_string();

            println!("Creating stream to save file at: {}", full_path_str);
            let mut stream = null_mut() as EdsStreamRef;
            let c_path = match CString::new(full_path_str.clone()) {
                Ok(p) => p,
                Err(e) => {
                    sender
                        .blocking_send(Err(format!("Invalid path string: {}", e)))
                        .unwrap();
                    return EdsError::Ok;
                }
            };

            let err = unsafe {
                EdsCreateFileStream(
                    c_path.as_ptr(),
                    edsdk::EdsFileCreateDisposition::CreateAlways,
                    edsdk::EdsAccess::ReadWrite,
                    &mut stream,
                )
            };
            if !err.is_ok() {
                sender
                    .blocking_send(Err(format!("Failed to create file stream: {:?}", err)))
                    .unwrap();
                return err;
            }

            let err = unsafe { EdsDownload(directory_item, dir_item_info.size, stream) };
            if !err.is_ok() {
                unsafe { EdsRelease(stream) };
                sender
                    .blocking_send(Err(format!("Failed to download: {:?}", err)))
                    .unwrap();
                return err;
            }

            let err = unsafe { EdsDownloadComplete(directory_item) };
            if !err.is_ok() {
                unsafe { EdsRelease(stream) };
                sender
                    .blocking_send(Err(format!("Failed to complete download: {:?}", err)))
                    .unwrap();
                return err;
            }

            let err = unsafe { EdsRelease(stream) };
            if !err.is_ok() {
                sender
                    .blocking_send(Err(format!("Failed to release stream: {:?}", err)))
                    .unwrap();
                return err;
            }

            println!("Photo saved successfully: {}", file_name);
            sender.blocking_send(Ok(file_name)).unwrap();

            EdsError::Ok
        }
        _ => EdsError::Ok,
    }
}

unsafe fn initialize_edsdk() -> Option<CameraRef> {
    let err = unsafe { EdsInitializeSDK() };
    assert!(err.is_ok());

    let mut camera_list = null_mut() as EdsCameraListRef;
    let err = unsafe { EdsGetCameraList(&mut camera_list) };
    assert!(err.is_ok());

    println!("camera list: {:?}, getting child count", camera_list);

    let mut num_cameras = 0;
    let err = unsafe { EdsGetChildCount(camera_list, &mut num_cameras) };
    assert!(err.is_ok());

    println!("Found {} cameras", num_cameras);

    if num_cameras == 0 {
        println!("No cameras found.");
        return None;
    }

    println!("Using first camera");

    let mut camera = null_mut() as EdsBaseRef;
    let err = unsafe { EdsGetChildAtIndex(camera_list, 0, &mut camera) };
    assert!(err.is_ok());

    let mut device_info = EdsDeviceInfo::default();
    let err = unsafe { EdsGetDeviceInfo(camera, &mut device_info) };
    assert!(err.is_ok());

    println!("Description: {}", device_info.sz_device_description());

    println!("opening session");
    let err = unsafe { EdsOpenSession(camera) };
    assert!(err.is_ok());

    let err = unsafe { EdsSetObjectEventHandler(camera, 0x200, event_handler, 0 as *mut EdsVoid) };
    assert!(err.is_ok());

    println!("save to size: {}", std::mem::size_of::<EdsSaveTo>());

    println!("setting save to");
    let save_to = EdsSaveTo::Host;
    let err = unsafe {
        EdsSetPropertyData(
            camera,
            0xb,
            0,
            std::mem::size_of::<EdsSaveTo>() as u32,
            &save_to as *const EdsSaveTo as *const EdsVoid,
        )
    };
    assert!(err.is_ok());

    println!("setting capacity");
    let capacity = EdsCapacity {
        number_of_free_clusters: 0x7FFFFFFF,
        bytes_per_sector: 0x1000,
        reset: 1,
    };
    let err = unsafe { EdsSetCapacity(camera, capacity) };
    assert!(err.is_ok());

    println!("setting image quality");
    let quality = EdsImageQuality::JpegSmall;
    let err = unsafe {
        EdsSetPropertyData(
            camera,
            0x100,
            0,
            std::mem::size_of::<EdsImageQuality>() as u32,
            &quality as *const EdsImageQuality as *const EdsVoid,
        )
    };
    assert!(err.is_ok());

    Some(camera.into())
}

pub fn camera_event_thread() {
    loop {
        let err = unsafe { EdsGetEvent() };
        assert!(err.is_ok());

        thread::sleep(Duration::from_millis(100));
    }
}
