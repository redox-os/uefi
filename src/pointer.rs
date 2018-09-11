//! This section defines the Simple Pointer Protocol and a detailed description of the
//! EFI_SIMPLE_POINTER_PROTOCOL. The intent of this section is to specify a simple method for
//! accessing pointer devices. This would include devices such as mice and trackballs.
//!
//! The EFI_SIMPLE_POINTER_PROTOCOL allows information about a pointer device to be retrieved.
//! This would include the status of buttons and the motion of the pointer device since the last time it
//! was accessed. This protocol is attached the device handle of a pointer device, and can be used for
//! input from the user in the preboot environment.

use crate::{status::Status, Event};

/// The following data values in the EFI_SIMPLE_POINTER_MODE interface are read-only and are
/// changed by using the appropriate interface functions.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SimplePointerMode {
    /// The resolution of the pointer device on the x-axis in counts/mm. If 0,
    ///then the pointer device does not support an x-axis.
    pub ResolutionX: u64,
    /// The resolution of the pointer device on the y-axis in counts/mm. If 0,
    /// then the pointer device does not support a y-axis.
    pub ResolutionY: u64,
    /// The resolution of the pointer device on the z-axis in counts/mm. If 0,
    /// then the pointer device does not support a z-axis.
    pub ResolutionZ: u64,
    /// TRUE if a left button is present on the pointer device. Otherwise FALSE.
    pub LeftButton: bool,
    /// TRUE if a right button is present on the pointer device. Otherwise FALSE.
    pub RightButton: bool,
}

/// The current state of a pointer device.
///
/// This includes information on the buttons associated with the pointer device and the distance
/// that each of the axes associated with the pointer device has been moved.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SimplePointerState {
    /// The signed distance in counts that the pointer device has been
    /// moved along the x-axis. The actual distance moved is
    /// RelativeMovementX / ResolutionX millimeters. If the ResolutionX
    /// field of the EFI_SIMPLE_POINTER_MODE structure is 0, then this
    /// pointer device does not support an x-axis, and this field must be
    /// ignored.
    pub RelativeMovementX: i32,
    /// The signed distance in counts that the pointer device has been
    /// moved along the y-axis. The actual distance moved is
    /// RelativeMovementY / ResolutionY millimeters. If the ResolutionY
    /// field of the EFI_SIMPLE_POINTER_MODE structure is 0, then this
    /// pointer device does not support a y-axis, and this field must be
    /// ignored.
    pub RelativeMovementY: i32,
    /// The signed distance in counts that the pointer device has been
    /// moved along the z-axis. The actual distance moved is
    /// RelativeMovementZ / ResolutionZ millimeters. If the ResolutionZ
    /// field of the EFI_SIMPLE_POINTER_MODE structure is 0, then this
    /// pointer device does not support a z-axis, and this field must be
    /// ignored.
    pub RelativeMovementZ: i32,
    /// If TRUE, then the left button of the pointer device is being pressed.
    /// If FALSE , then the left button of the pointer device is not being
    /// pressed. If the LeftButton field of the EFI_SIMPLE_POINTER_MODE
    /// structure is FALSE, then this field is not valid, and must be ignored.
    pub LeftButton: bool,
    /// If TRUE, then the right button of the pointer device is being pressed.
    /// If FALSE, then the right button of the pointer device is not being
    /// pressed. If the RightButton field of the EFI_SIMPLE_POINTER_MODE
    /// structure is FALSE, then this field is not valid, and must be ignored.
    pub RightButton: bool,
}

/// Provides services that allow information about a pointer device to be retrieved.
#[repr(C)]
pub struct SimplePointer {
    /// Resets the pointer device.
    pub Reset: extern "win64" fn(&mut SimplePointer, ExtendedVerification: bool) -> Status,
    /// Retrieves the current state of the pointer device.
    pub GetState: extern "win64" fn(&mut SimplePointer, State: &mut SimplePointerState) -> Status,
    /// Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for input from the pointer device.
    pub WaitForInput: Event,
    /// Pointer to EFI_SIMPLE_POINTER_MODE data.
    pub Mode: &'static mut SimplePointerMode,
}
