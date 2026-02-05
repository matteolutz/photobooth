/// Rust bindings for Canon's EDSDK (v. 13.20.10)
use std::ffi::CStr;

use bitflags::bitflags;

#[repr(u32)]
#[derive(PartialEq, Eq, Debug)]
pub enum EdsError {
    Ok = 0,

    /// This error doesn't exist on the EDSDK but is used to indicate that no camera was found.
    NoCameraFound = u32::MAX,
}

impl EdsError {
    pub fn is_ok(&self) -> bool {
        *self == EdsError::Ok
    }

    pub fn res(self) -> Result<(), EdsError> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl From<EdsError> for Result<(), EdsError> {
    fn from(value: EdsError) -> Self {
        if value.is_ok() {
            Ok(())
        } else {
            Err(value)
        }
    }
}

#[repr(C)]
pub enum EdsFileCreateDisposition {
    CreateNew = 0,
    CreateAlways = 1,
    OpenExisting = 2,
    OpenAlways = 3,
    TruncateExisting = 4,
}

#[repr(C)]
pub enum EdsAccess {
    Read = 0,
    Write = 1,
    ReadWrite = 2,
    Error = 0xFFFFFFFF,
}

#[repr(C)]
pub enum EdsSaveTo {
    Camera = 1,
    Host = 2,
    Both = Self::Camera as isize | Self::Host as isize,
}

bitflags! {
    #[repr(transparent)]
    pub struct EdsEvfOutputDevice : u32 {
        const TFT = 1;
        const PC = 2;
        const PC_SMALL= 8;
    }
}

impl Default for EdsEvfOutputDevice {
    fn default() -> Self {
        EdsEvfOutputDevice::TFT
    }
}

pub type EdsVoid = std::os::raw::c_void;

pub type EdsChar = std::os::raw::c_char;
pub type EdsBool = std::os::raw::c_int;

pub type EdsBaseRef = *mut std::os::raw::c_void;

pub type EdsCameraRef = EdsBaseRef;
pub type EdsCameraListRef = EdsBaseRef;

pub type EdsStreamRef = EdsBaseRef;

pub type EdsDirectoryItemRef = EdsBaseRef;

pub type EdsEvfImageRef = EdsBaseRef;

pub type EdsCameraCommand = u32;

pub type EdsPropertyId = u32;

pub type EdsObjectEvent = u32;
pub type EdsObjectEventHandler =
    extern "C" fn(event: EdsObjectEvent, object_ref: EdsBaseRef, context: *mut EdsVoid) -> EdsError;

#[repr(C)]
pub struct EdsDeviceInfo {
    pub sz_port_name: [EdsChar; 256],
    pub sz_device_description: [EdsChar; 256],
    pub device_sub_type: u32,
    _reserved: u32,
}

impl EdsDeviceInfo {
    pub fn sz_port_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.sz_port_name.as_ptr()).to_str().unwrap() }
    }

    pub fn sz_device_description(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.sz_device_description.as_ptr())
                .to_str()
                .unwrap()
        }
    }
}

impl Default for EdsDeviceInfo {
    fn default() -> Self {
        Self {
            sz_port_name: [0; 256],
            sz_device_description: [0; 256],
            device_sub_type: Default::default(),
            _reserved: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EdsDirectoryItemInfo {
    pub size: u64,
    pub is_folder: EdsBool,
    pub group_id: u32,
    pub option: u32,
    pub sz_file_name: [EdsChar; 256],

    pub format: u32,
    pub date_time: u32,
}

impl EdsDirectoryItemInfo {
    pub fn sz_file_name(&self) -> &str {
        unsafe { CStr::from_ptr(self.sz_file_name.as_ptr()).to_str().unwrap() }
    }
}

impl Default for EdsDirectoryItemInfo {
    fn default() -> Self {
        Self {
            size: Default::default(),
            is_folder: Default::default(),
            group_id: Default::default(),
            option: Default::default(),
            sz_file_name: [0; 256],
            format: Default::default(),
            date_time: Default::default(),
        }
    }
}

#[repr(C)]
pub struct EdsCapacity {
    pub number_of_free_clusters: u32,
    pub bytes_per_sector: u32,
    pub reset: EdsBool,
}

/// *TODO*: The rest of the quality options
#[repr(C)]
pub enum EdsImageQuality {
    JpegLarge = 0x0010ff0f,         /* Jpeg Large */
    JpegMiddle = 0x0110ff0f,        /* Jpeg Middle */
    JpegMiddle1 = 0x0510ff0f,       /* Jpeg Middle1 */
    JpegMiddle1Fine = 0x0513FF0F,   /* Jpeg Middle1 Fine */
    JpegMiddle1Normal = 0x0512FF0F, /* Jpeg Middle1 Normal */
    JpegMiddle2 = 0x0610ff0f,       /* Jpeg Middle2 */
    JpegMiddle2Fine = 0x0613FF0F,   /* Jpeg Middle2 Fine */
    JpegMiddle2Normal = 0x0612FF0F, /* Jpeg Middle2 Normal */
    JpegSmall = 0x0210ff0f,         /* Jpeg Small */
    JpegSmall1 = 0x0e10ff0f,        /* Jpeg Small1 */
    JpegSmall2 = 0x0f10ff0f,        /* Jpeg Small2 */
    JpegLargeFine = 0x0013ff0f,     /* Jpeg Large Fine */
    JpegLargeNormal = 0x0012ff0f,   /* Jpeg Large Normal */
    JpegMiddleFine = 0x0113ff0f,    /* Jpeg Middle Fine */
    JpegMiddleNormal = 0x0112ff0f,  /* Jpeg Middle Normal */
    JpegSmallFine = 0x0213ff0f,     /* Jpeg Small Fine */
    JpegSmallNormal = 0x0212ff0f,   /* Jpeg Small Normal */
    JpegSmall1Fine = 0x0E13ff0f,    /* Jpeg Small1 Fine */
    JpegSmall1Normal = 0x0E12ff0f,  /* Jpeg Small1 Normal */
    JpegSmall2Fine = 0x0F13ff0f,    /* Jpeg Small2 */
    JpegSmall3 = 0x1013ff0f,        /* Jpeg Small3 */
}

#[cfg_attr(target_os = "macos", link(name = "EDSDK", kind = "framework"))]
#[cfg_attr(target_os = "windows", link(name = "EDSDK"))]
extern "C" {
    pub fn EdsInitializeSDK() -> EdsError;
    pub fn EdsTerminateSDK() -> EdsError;

    pub fn EdsRelease(base_ref: EdsBaseRef) -> EdsError;

    pub fn EdsGetChildCount(base_ref: EdsBaseRef, count: *mut u32) -> EdsError;
    pub fn EdsGetChildAtIndex(
        parent_ref: EdsBaseRef,
        index: u32,
        child_ref: *mut EdsBaseRef,
    ) -> EdsError;

    pub fn EdsGetCameraList(camera_list: *mut EdsCameraListRef) -> EdsError;

    pub fn EdsGetDeviceInfo(camera_ref: EdsCameraRef, device_info: *mut EdsDeviceInfo) -> EdsError;

    pub fn EdsOpenSession(camera_ref: EdsCameraRef) -> EdsError;
    pub fn EdsCloseSession(camera_ref: EdsCameraRef) -> EdsError;

    pub fn EdsGetPropertyData(
        in_ref: EdsBaseRef,
        property_id: EdsPropertyId,
        param: i32,
        size: u32,
        out_data: *mut EdsVoid,
    ) -> EdsError;
    pub fn EdsSetPropertyData(
        in_ref: EdsBaseRef,
        property_id: EdsPropertyId,
        param: i32,
        size: u32,
        data: *const EdsVoid,
    ) -> EdsError;

    pub fn EdsSendCommand(
        camera_ref: EdsCameraRef,
        command: EdsCameraCommand,
        param: i32,
    ) -> EdsError;

    pub fn EdsGetEvent() -> EdsError;
    pub fn EdsSetObjectEventHandler(
        camera_ref: EdsCameraRef,
        event: EdsObjectEvent,
        event_handler: EdsObjectEventHandler,
        context: *mut EdsVoid,
    ) -> EdsError;

    pub fn EdsGetDirectoryItemInfo(
        in_dir_item_ref: EdsDirectoryItemRef,
        out_info: *mut EdsDirectoryItemInfo,
    ) -> EdsError;

    pub fn EdsCreateFileStream(
        in_file_name: *const EdsChar,
        in_create_disposition: EdsFileCreateDisposition,
        in_desired_access: EdsAccess,
        out_stream: *mut EdsStreamRef,
    ) -> EdsError;

    pub fn EdsCreateMemoryStreamFromPointer(
        in_user_buffer: *mut EdsVoid,
        in_buffer_size: u64,
        out_stream: *mut EdsStreamRef,
    ) -> EdsError;

    pub fn EdsDownload(
        in_dir_item_ref: EdsDirectoryItemRef,
        in_read_size: u64,
        in_stream_ref: EdsStreamRef,
    ) -> EdsError;
    pub fn EdsDownloadComplete(in_dir_item_ref: EdsDirectoryItemRef) -> EdsError;

    pub fn EdsSetCapacity(camera_ref: EdsCameraRef, capacity: EdsCapacity) -> EdsError;

    pub fn EdsCreateEvfImageRef(
        in_stream: EdsStreamRef,
        out_evf_image_ref: *mut EdsEvfImageRef,
    ) -> EdsError;
    pub fn EdsDownloadEvfImage(
        in_camera: EdsCameraRef,
        in_evf_image_ref: EdsEvfImageRef,
    ) -> EdsError;
}
