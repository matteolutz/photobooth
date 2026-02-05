use std::{
    ffi::CString,
    // os::unix::ffi::OsStrExt,
    ptr::null_mut,
    sync::{LazyLock, Mutex},
    thread,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Local};
use edsdk::{
    EdsBaseRef, EdsCameraListRef, EdsCameraRef, EdsCapacity, EdsCloseSession, EdsCreateEvfImageRef,
    EdsCreateFileStream, EdsDeviceInfo, EdsDirectoryItemInfo, EdsDirectoryItemRef, EdsDownload,
    EdsDownloadComplete, EdsDownloadEvfImage, EdsError, EdsEvfImageRef, EdsEvfOutputDevice,
    EdsGetCameraList, EdsGetChildAtIndex, EdsGetChildCount, EdsGetDeviceInfo,
    EdsGetDirectoryItemInfo, EdsGetEvent, EdsGetPropertyData, EdsImageQuality, EdsInitializeSDK,
    EdsObjectEvent, EdsOpenSession, EdsRelease, EdsSaveTo, EdsSendCommand, EdsSetCapacity,
    EdsSetObjectEventHandler, EdsSetPropertyData, EdsStreamRef, EdsTerminateSDK, EdsVoid,
};
use tauri::{async_runtime::Sender, AppHandle, Emitter};

use crate::{
    path::{CAMERA_PHOTO_DIR, EVF_IMAGE},
    CAMERA,
};

/// Global channel to receive the filename from the callback
static PHOTO_RESULT_SENDER: LazyLock<Mutex<Option<Sender<Result<String, String>>>>> =
    LazyLock::new(|| Mutex::new(None));

pub struct CameraRef {
    camera: EdsCameraRef,
}

impl From<EdsCameraRef> for CameraRef {
    fn from(camera: EdsCameraRef) -> Self {
        Self { camera }
    }
}

impl CameraRef {
    pub fn init(enable_live_view: bool) -> Result<Self, EdsError> {
        unsafe { Self::_init(enable_live_view) }
    }

    unsafe fn _init(enable_live_view: bool) -> Result<Self, EdsError> {
        unsafe { EdsInitializeSDK().res() }?;

        let mut camera_list = null_mut() as EdsCameraListRef;
        unsafe { EdsGetCameraList(&mut camera_list).res() }?;

        println!("camera list: {:?}, getting child count", camera_list);

        let mut num_cameras = 0;
        unsafe { EdsGetChildCount(camera_list, &mut num_cameras).res() }?;

        println!("Found {} cameras", num_cameras);

        if num_cameras == 0 {
            println!("No cameras found.");
            return Err(EdsError::NoCameraFound);
        }

        println!("Using first camera");

        let mut camera = null_mut() as EdsBaseRef;
        unsafe { EdsGetChildAtIndex(camera_list, 0, &mut camera).res() }?;

        let mut device_info = EdsDeviceInfo::default();
        unsafe { EdsGetDeviceInfo(camera, &mut device_info).res() }?;

        println!("Description: {}", device_info.sz_device_description());

        println!("opening session");
        unsafe { EdsOpenSession(camera).res() }?;

        unsafe { EdsSetObjectEventHandler(camera, 0x200, event_handler, 0 as *mut EdsVoid).res() }?;

        println!("save to size: {}", std::mem::size_of::<EdsSaveTo>());

        println!("setting save to");
        let save_to = EdsSaveTo::Host;
        unsafe {
            EdsSetPropertyData(
                camera,
                0xb,
                0,
                std::mem::size_of::<EdsSaveTo>() as u32,
                &save_to as *const EdsSaveTo as *const EdsVoid,
            )
            .res()
        }?;

        println!("setting capacity");
        let capacity = EdsCapacity {
            number_of_free_clusters: 0x7FFFFFFF,
            bytes_per_sector: 0x1000,
            reset: 1,
        };
        unsafe { EdsSetCapacity(camera, capacity).res() }?;

        println!("setting image quality");
        let quality = EdsImageQuality::JpegSmall;
        unsafe {
            EdsSetPropertyData(
                camera,
                0x100,
                0,
                std::mem::size_of::<EdsImageQuality>() as u32,
                &quality as *const EdsImageQuality as *const EdsVoid,
            )
            .res()
        }?;

        let camera: CameraRef = camera.into();

        if enable_live_view {
            camera.enable_evf_live_view()?;
        }

        Ok(camera)
    }

    pub fn take_picture(&self, respond_to: Sender<Result<String, String>>) {
        let camera = self.camera;

        *PHOTO_RESULT_SENDER.lock().unwrap() = Some(respond_to);

        let err = unsafe { EdsSendCommand(camera, 4, 3) }; // 4 = shutter command, 3 = shutter completely
        unsafe { EdsSendCommand(camera, 4, 0) }; // 0 = shutter off
        assert!(err.is_ok());
    }

    pub fn get_evf_image(&self) -> Result<(), EdsError> {
        let camera = self.camera;

        // open file stream
        let mut stream = null_mut() as EdsStreamRef;
        let file_name = CString::new(EVF_IMAGE.get().unwrap().as_os_str().as_encoded_bytes()).unwrap();
        unsafe {
            EdsCreateFileStream(
                file_name.as_ptr(),
                edsdk::EdsFileCreateDisposition::CreateAlways,
                edsdk::EdsAccess::ReadWrite,
                &mut stream,
            )
            .res()
        }?;
        /*
        unsafe {
            EdsCreateMemoryStreamFromPointer(
                self.evf_buffer.as_mut_ptr() as *mut EdsVoid,
                self.evf_buffer.len() as u64,
                &mut stream,
            )
            .res()
        }?;*/

        // get evf image ref
        let mut evf_image = null_mut() as EdsEvfImageRef;
        unsafe { EdsCreateEvfImageRef(stream, &mut evf_image).res() }?;

        // download evf image
        unsafe { EdsDownloadEvfImage(camera, evf_image).res() }?;

        // release file stream
        unsafe { EdsRelease(stream).res() }?;

        // relase evf image
        unsafe { EdsRelease(evf_image).res() }?;

        Ok(())
    }

    pub fn enable_evf_live_view(&self) -> Result<(), EdsError> {
        let mut current_live_view = Self::get_evf_output_device(self.camera)?;
        current_live_view.insert(EdsEvfOutputDevice::PC);

        Self::set_evf_output_device(self.camera, current_live_view)?;

        Ok(())
    }

    fn set_evf_output_device(
        camera: EdsCameraRef,
        evf_output: EdsEvfOutputDevice,
    ) -> Result<(), EdsError> {
        unsafe {
            EdsSetPropertyData(
                camera,
                0x500,
                0,
                std::mem::size_of::<EdsEvfOutputDevice>() as u32,
                &evf_output as *const EdsEvfOutputDevice as *const EdsVoid,
            )
            .res()
        }?;
        Ok(())
    }

    fn get_evf_output_device(camera: EdsCameraRef) -> Result<EdsEvfOutputDevice, EdsError> {
        let mut output_device = EdsEvfOutputDevice::default();
        unsafe {
            EdsGetPropertyData(
                camera,
                0x500,
                0,
                std::mem::size_of::<EdsEvfOutputDevice>() as u32,
                &mut output_device as *mut EdsEvfOutputDevice as *mut EdsVoid,
            )
            .res()
        }?;
        Ok(output_device)
    }
}

impl Drop for CameraRef {
    fn drop(&mut self) {
        // reset evf output
        if let Ok(mut current_live_view) = Self::get_evf_output_device(self.camera) {
            current_live_view.remove(EdsEvfOutputDevice::PC);
            let _ = Self::set_evf_output_device(self.camera, current_live_view);
        }

        unsafe { EdsCloseSession(self.camera) };
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

pub fn camera_event_thread(app: AppHandle, enable_live_view: bool) {
    loop {
        let err = unsafe { EdsGetEvent() };
        assert!(err.is_ok());

        // only take live image if we are not currently expecting a photo result
        if enable_live_view
            && PHOTO_RESULT_SENDER
                .try_lock()
                .is_ok_and(|sender| sender.is_none())
        {
            if let Some(cam) = CAMERA.blocking_lock().as_ref() {
                if cam.get_evf_image().is_ok() {
                    let _ = app.emit("evf-update", serde_json::Value::Null);
                }
            } else {
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
