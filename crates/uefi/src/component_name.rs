// SPDX-License-Identifier: MIT

use crate::prelude::*;

#[repr(C)]
pub struct ComponentName {
    pub GetDriverName:
        unsafe extern "efiapi" fn(This: *mut Self, Language: *const u8, DriverName: *mut *mut u16) -> Status,
    pub GetControllerName: unsafe extern "efiapi" fn(
        This: *mut Self,
        ControllerHandle: Handle,
        ChildHandle: Handle,
        Language: *const u8,
        ControllerName: *mut *mut u16,
    ) -> Status,
    pub SupportedLanguages: *const u8,
}

impl ComponentName {
    pub const GUID: Guid = guid!("6a7a5cff-e8d9-4f70-bada-75ab3025ce14");
}
