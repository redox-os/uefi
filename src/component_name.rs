use crate::Handle;
use crate::status::Status;

#[repr(C)]
pub struct ComponentName {
    pub GetDriverName: extern "win64" fn(
        &ComponentName,
        Language: *const u8,
        DriverName: &mut *mut u16
    ) -> Status,
    pub GetControllerName: extern "win64" fn(
        &ComponentName,
        ControllerHandle: Handle,
        ChildHandle: Handle,
        Language: *const u8,
        ControllerName: &mut *mut u16,
    ) -> Status,
    pub SupportedLanguages: *const u8,
}
