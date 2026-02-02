use std::{
    ffi::CString,
    ptr::null_mut,
    sync::{mpsc, Mutex, OnceLock},
    thread,
    time::Duration,
};

use edsdk::{
    EdsBaseRef, EdsCameraListRef, EdsCameraRef, EdsCapacity, EdsCreateFileStream, EdsDeviceInfo,
    EdsDirectoryItemInfo, EdsDirectoryItemRef, EdsDownload, EdsDownloadComplete, EdsError,
    EdsGetCameraList, EdsGetChildAtIndex, EdsGetChildCount, EdsGetDeviceInfo,
    EdsGetDirectoryItemInfo, EdsGetEvent, EdsImageQuality, EdsInitializeSDK, EdsObjectEvent,
    EdsOpenSession, EdsRelease, EdsSaveTo, EdsSendCommand, EdsSetCapacity,
    EdsSetObjectEventHandler, EdsSetPropertyData, EdsStreamRef, EdsVoid,
};

use crate::comm::{
    CommRequest, CommRequestEnvelope, CommRequestHandler, CommRequestHandlerContext,
};
use crate::path::CAMERA_PHOTO_DIR;

pub struct TakePictureRequest;
impl CommRequest for TakePictureRequest {
    /// Ok -> name of file in the CAMERA dir
    /// Err -> error message
    type Response = Result<String, String>;
}

/// Global channel to receive the filename from the callback
static PHOTO_RESULT_SENDER: OnceLock<Mutex<Option<mpsc::Sender<Result<String, String>>>>> =
    OnceLock::new();

fn init_photo_result_channel() -> mpsc::Receiver<Result<String, String>> {
    let (tx, rx) = mpsc::channel();
    let _ = PHOTO_RESULT_SENDER.set(Mutex::new(Some(tx)));
    rx
}

fn reset_photo_result_sender(tx: mpsc::Sender<Result<String, String>>) {
    if let Some(lock) = PHOTO_RESULT_SENDER.get() {
        if let Ok(mut guard) = lock.lock() {
            *guard = Some(tx);
        }
    }
}

fn send_photo_result(result: Result<String, String>) {
    if let Some(lock) = PHOTO_RESULT_SENDER.get() {
        if let Ok(mut guard) = lock.lock() {
            if let Some(sender) = guard.take() {
                let _ = sender.send(result);
            }
        }
    }
}

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
            println!("received DirItemCreated/DirItemRequestTransfer event");
            let directory_item = in_ref as EdsDirectoryItemRef;

            println!("getting directory info");
            let mut dir_item_info = EdsDirectoryItemInfo::default();
            let err = unsafe { EdsGetDirectoryItemInfo(directory_item, &mut dir_item_info) };
            if !err.is_ok() {
                send_photo_result(Err(format!("Failed to get directory info: {:?}", err)));
                return err;
            }

            let file_name = dir_item_info.sz_file_name().to_string();
            println!("got info: {:?}", dir_item_info);
            println!("file name: {}", file_name);

            // Get the camera photo directory and create the full path
            let camera_dir = CAMERA_PHOTO_DIR
                .get()
                .expect("CAMERA_PHOTO_DIR not initialized");
            let full_path = camera_dir.join(&file_name);
            let full_path_str = full_path.to_string_lossy().to_string();

            println!("Creating stream to save file at: {}", full_path_str);
            let mut stream = null_mut() as EdsStreamRef;
            let c_path = match CString::new(full_path_str.clone()) {
                Ok(p) => p,
                Err(e) => {
                    send_photo_result(Err(format!("Invalid path string: {}", e)));
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
                send_photo_result(Err(format!("Failed to create file stream: {:?}", err)));
                return err;
            }

            let err = unsafe { EdsDownload(directory_item, dir_item_info.size, stream) };
            if !err.is_ok() {
                unsafe { EdsRelease(stream) };
                send_photo_result(Err(format!("Failed to download: {:?}", err)));
                return err;
            }

            let err = unsafe { EdsDownloadComplete(directory_item) };
            if !err.is_ok() {
                unsafe { EdsRelease(stream) };
                send_photo_result(Err(format!("Failed to complete download: {:?}", err)));
                return err;
            }

            let err = unsafe { EdsRelease(stream) };
            if !err.is_ok() {
                send_photo_result(Err(format!("Failed to release stream: {:?}", err)));
                return err;
            }

            println!("Photo saved successfully: {}", file_name);
            send_photo_result(Ok(file_name));
        }
        _ => {}
    }

    EdsError::Ok
}

unsafe fn initialize_edsdk() -> Option<EdsCameraRef> {
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

    Some(camera)
}

pub fn start_camera_thread(rx: mpsc::Receiver<CommRequestEnvelope>) {
    let mut handler = CommRequestHandler::new(rx);

    let camera = unsafe { initialize_edsdk().unwrap() };

    // Initialize the global photo result sender storage
    // Note: We don't keep the initial receiver since each photo request creates its own channel
    let _ = init_photo_result_channel();

    // Note: Photo requests are serialized by the CommRequestHandler (single-threaded),
    // so we don't need to worry about race conditions between concurrent requests.
    // Each request creates its own channel before sending the TakePicture command.
    handler.register(move |_: TakePictureRequest, _| {
        println!("Taking picture...");

        // Create a new channel for this specific photo request
        let (tx, rx) = mpsc::channel();
        reset_photo_result_sender(tx);

        // Send the take picture command to the camera
        // kEdsCameraCommand_TakePicture = 0x00000000
        let err = unsafe { EdsSendCommand(camera, 0x00000000, 0) };
        if !err.is_ok() {
            return Err(format!("Failed to send take picture command: {:?}", err));
        }

        println!("Take picture command sent, waiting for result...");

        // Wait for the callback to send us the result
        // The callback will be triggered by EdsGetEvent in the main loop
        // We need to poll EdsGetEvent while waiting since the EDSDK requires polling
        let timeout = Duration::from_secs(30);
        let start = std::time::Instant::now();

        loop {
            // Process pending events - EDSDK requires continuous polling
            let err = unsafe { EdsGetEvent() };
            if !err.is_ok() {
                return Err(format!("Failed to get event: {:?}", err));
            }

            // Check if we got a result from the callback
            match rx.try_recv() {
                Ok(result) => {
                    println!("Received photo result: {:?}", result);
                    return result;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // Continue waiting
                    if start.elapsed() > timeout {
                        return Err("Timeout waiting for photo".to_string());
                    }
                    // Poll at 100ms intervals - EDSDK requires periodic event polling
                    thread::sleep(Duration::from_millis(100));
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err("Photo result channel disconnected".to_string());
                }
            }
        }
    });

    loop {
        handler.handle_all(CommRequestHandlerContext {});

        let err = unsafe { EdsGetEvent() };
        assert!(err.is_ok());

        thread::sleep(Duration::from_millis(100));
    }
}
