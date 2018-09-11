//! UEFI uses the EFI Runtime Services Table, which contains a table header and pointers to all of the
//! runtime services. The definition for this table is shown in the following code fragments. Except for
//! the table header, all elements in the EFI Runtime Services Tables are prototypes of function
//! pointers to functions as defined in Section 8. Unlike the EFI Boot Services Table, this table, and the
//! function pointers it contains are valid after the UEFI OS loader and OS have taken control of the
//! platform with a call to EFI_BOOT_SERVICES.ExitBootServices(). If a call to SetVirtualAddressMap()
//! is made by the OS, then the function pointers in this table are fixed up
//! to point to the new virtually mapped entry points.

use crate::{
    capsule::CapsuleHeader,
    guid::Guid,
    memory::{MemoryDescriptor, PhysicalAddress},
    reset::ResetType,
    status::Status,
    time::{Time, TimeCapabilities},
    TableHeader,
};

/// Contains a table header and pointers to all of the runtime services.
#[repr(C)]
pub struct RuntimeServices {
    pub Hdr: TableHeader,
    /// Returns the current time and date information, and the time-keeping capabilities of the hardware
    /// platform.
    pub GetTime: extern "win64" fn(Time: &mut Time, Capabilities: *mut TimeCapabilities) -> Status,
    /// Sets the current local time and date information.
    pub SetTime: extern "win64" fn(Time: &Time) -> Status,
    /// Returns the current wakeup alarm clock setting.
    pub GetWakeupTime:
        extern "win64" fn(Enabled: &mut bool, Pending: &mut bool, Time: &mut Time) -> Status,
    /// Sets the system wakeup alarm clock time.
    pub SetWakeupTime: extern "win64" fn(Enable: bool, Time: *const Time) -> Status,
    /// Changes the runtime addressing mode of EFI firmware from physical to virtual.
    SetVirtualAddressMap: extern "win64" fn(
        MemoryMapSize: usize,
        DescriptorSize: usize,
        DescriptorVersion: u32,
        VirtualMap: *const MemoryDescriptor,
    ) -> Status,
    /// Determines the new virtual address that is to be used on subsequent memory accesses.
    pub ConvertPointer: extern "win64" fn(DebugDisposition: usize, Address: &mut usize) -> Status,
    /// Returns the value of a variable.
    pub GetVariable: extern "win64" fn(
        VariableName: *const u16,
        VendorGuid: &Guid,
        Attributes: *mut u32,
        DataSize: &mut usize,
        Data: *mut u8,
    ) -> Status,
    /// Enumerates the current variable names.
    pub GetNextVariableName: extern "win64" fn(
        VariableNameSize: &mut usize,
        VariableName: *mut u16,
        VendorGuid: &mut Guid,
    ) -> Status,
    /// Sets the value of a variable.
    pub SetVariable: extern "win64" fn(
        VariableName: *const u16,
        VendorGuid: &Guid,
        Attributes: u32,
        DataSize: usize,
        Data: *const u8,
    ) -> Status,
    /// Returns the next high 32 bits of the platform’s monotonic counter.
    pub GetNextHighMonotonicCount: extern "win64" fn(HighCount: &mut u32) -> Status,
    /// Resets the entire platform. If the platform supports EFI_RESET_NOTIFICATION_PROTOCOL,
    /// then prior to completing the reset of the platform, all of the pending notifications must be called.
    pub ResetSystem: extern "win64" fn(
        ResetType: ResetType,
        ResetStatus: Status,
        DataSize: usize,
        ResetData: *const u8,
    ) -> !,
    /// Passes capsules to the firmware with both virtual and physical mapping. Depending on the
    /// intended consumption, the firmware may process the capsule immediately. If the payload should
    /// persist across a system reset, the reset value returned from EFI_QueryCapsuleCapabilities
    /// must be passed into ResetSystem() and will cause the capsule to be processed by the firmware
    /// as part of the reset process.
    pub UpdateCapsule: extern "win64" fn(
        CapsuleHeaderArray: *const *const CapsuleHeader,
        CapsuleCount: usize,
        ScatterGatherList: PhysicalAddress,
    ) -> Status,
    /// Returns if the capsule can be supported via UpdateCapsule().
    pub QueryCapsuleCapabilities:
        extern "win64" fn(
            CapsuleHeaderArray: *const *const CapsuleHeader,
            CapsuleCount: usize,
            MaximumCapsuleSize: &mut u64,
            ResetType: &mut ResetType,
        ) -> Status,
    /// Returns information about the EFI variables.
    pub QueryVariableInfo: extern "win64" fn(
        Attributes: u32,
        MaximumVariableStorageSize: &mut u64,
        RemainingVariableStorageSize: &mut u64,
        MaximumVariableSize: &mut u64,
    ) -> Status,
}
