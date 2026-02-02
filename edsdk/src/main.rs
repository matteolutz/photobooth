use std::{ffi::CString, ptr::null_mut, thread, time::Duration};

use edsdk::{
    EdsBaseRef, EdsCameraListRef, EdsCapacity, EdsCloseSession, EdsCreateFileStream, EdsDeviceInfo,
    EdsDirectoryItemInfo, EdsDirectoryItemRef, EdsDownload, EdsDownloadComplete, EdsError,
    EdsGetCameraList, EdsGetChildAtIndex, EdsGetChildCount, EdsGetDeviceInfo,
    EdsGetDirectoryItemInfo, EdsGetEvent, EdsImageQuality, EdsInitializeSDK, EdsObjectEvent,
    EdsOpenSession, EdsRelease, EdsSaveTo, EdsSendCommand, EdsSetCapacity,
    EdsSetObjectEventHandler, EdsSetPropertyData, EdsStreamRef, EdsTerminateSDK, EdsVoid,
};

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
            println!("received DirItemCreated event");
            let directory_item = in_ref as EdsDirectoryItemRef;

            println!("getting directory info");
            let mut dir_item_info = EdsDirectoryItemInfo::default();
            let err = unsafe { EdsGetDirectoryItemInfo(directory_item, &mut dir_item_info) };
            assert!(err.is_ok());

            println!("got info: {:?}", dir_item_info);
            println!("file name: {}", dir_item_info.sz_file_name());

            println!("Creating stream");
            let mut stream = null_mut() as EdsStreamRef;
            let err = unsafe {
                EdsCreateFileStream(
                    CString::new("test.jpeg").unwrap().as_ptr(),
                    edsdk::EdsFileCreateDisposition::CreateAlways,
                    edsdk::EdsAccess::ReadWrite,
                    &mut stream,
                )
            };
            assert!(err.is_ok());

            let err = unsafe { EdsDownload(directory_item, dir_item_info.size, stream) };
            assert!(err.is_ok());

            let err = unsafe { EdsDownloadComplete(directory_item) };
            assert!(err.is_ok());

            let err = unsafe { EdsRelease(stream) };
            assert!(err.is_ok());
        }
        _ => {}
    }

    // Handle events here
    EdsError::Ok
}

pub fn main() {
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
        return;
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

    println!("pressing shutter");
    let err = unsafe { EdsSendCommand(camera, 4, 3) }; // 4 = shutter command, 3 = shutter completely
    unsafe { EdsSendCommand(camera, 4, 0) }; // 0 = shutter off
    assert!(err.is_ok());

    /*
    println!("taking picture");
    let err = unsafe { EdsSendCommand(camera, 0, 0) }; // 4 = shutter command, 3 = shutter completely
    assert!(err.is_ok());*/

    loop {
        let err = unsafe { EdsGetEvent() };
        assert!(err.is_ok());

        thread::sleep(Duration::from_millis(200));
    }

    let err = unsafe { EdsCloseSession(camera) };
    assert!(err.is_ok());

    let err = unsafe { EdsTerminateSDK() };
    assert!(err.is_ok());
}
