use ::TableHeader;
use status::Status;
use time::{Time, TimeCapabilities};

#[repr(C)]
pub struct RuntimeServices {
    pub Hdr: TableHeader,
    pub GetTime: extern "win64" fn(Time: &mut Time, Capabilities: *mut TimeCapabilities) -> Status,
    pub SetTime: extern "win64" fn(Time: &Time) -> Status,
}
