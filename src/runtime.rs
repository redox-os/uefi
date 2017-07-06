use super::TableHeader;
use super::time::{Time, TimeCapabilities};

#[repr(C)]
pub struct RuntimeServices {
    Hdr: TableHeader,
    pub GetTime: extern "win64" fn(Time: &mut Time, Capabilities: *mut TimeCapabilities),
    pub SetTime: extern "win64" fn(Time: &Time),
}
