use crate::TableHeader;
use crate::capsule::CapsuleHeader;
use crate::guid::Guid;
use crate::memory::{PhysicalAddress, MemoryDescriptor};
use crate::reset::ResetType;
use crate::status::Status;
use crate::time::{Time, TimeCapabilities};

#[repr(C)]
pub struct RuntimeServices {
    pub Hdr: TableHeader,

    pub GetTime: extern "win64" fn(
        Time: &mut Time,
        Capabilities: *mut TimeCapabilities
    ) -> Status,

    pub SetTime: extern "win64" fn(
        Time: &Time
    ) -> Status,

    pub GetWakeupTime: extern "win64" fn(
        Enabled: &mut bool,
        Pending: &mut bool,
        Time: &mut Time
    ) -> Status,

    pub SetWakeupTime: extern "win64" fn(
        Enable: bool,
        Time: *const Time
    ) -> Status,

    SetVirtualAddressMap: extern "win64" fn(
        MemoryMapSize: usize,
        DescriptorSize: usize,
        DescriptorVersion: u32,
        VirtualMap: *const MemoryDescriptor
    ) -> Status,

    pub ConvertPointer: extern "win64" fn(
        DebugDisposition: usize,
        Address: &mut usize
    ) -> Status,

    pub GetVariable: extern "win64" fn(
        VariableName: *const u16,
        VendorGuid: &Guid,
        Attributes: *mut u32,
        DataSize: &mut usize,
        Data: *mut u8
    ) -> Status,

    pub GetNextVariableName: extern "win64" fn(
        VariableNameSize: &mut usize,
        VariableName: *mut u16,
        VendorGuid: &mut Guid
    ) -> Status,

    pub SetVariable: extern "win64" fn(
        VariableName: *const u16,
        VendorGuid: &Guid,
        Attributes: u32,
        DataSize: usize,
        Data: *const u8
    ) -> Status,

    pub GetNextHighMonotonicCount: extern "win64" fn(
        HighCount: &mut u32
    ) -> Status,

    pub ResetSystem: extern "win64" fn(
        ResetType: ResetType,
        ResetStatus: Status,
        DataSize: usize,
        ResetData: *const u8
    ) -> !,

    pub UpdateCapsule: extern "win64" fn(
        CapsuleHeaderArray: *const *const CapsuleHeader,
        CapsuleCount: usize,
        ScatterGatherList: PhysicalAddress
    ) -> Status,

    pub QueryCapsuleCapabilities: extern "win64" fn(
        CapsuleHeaderArray: *const *const CapsuleHeader,
        CapsuleCount: usize,
        MaximumCapsuleSize: &mut u64,
        ResetType: &mut ResetType
    ) -> Status,

    pub QueryVariableInfo: extern "win64" fn(
        Attributes: u32,
        MaximumVariableStorageSize: &mut u64,
        RemainingVariableStorageSize: &mut u64,
        MaximumVariableSize: &mut u64
    ) -> Status,
}
