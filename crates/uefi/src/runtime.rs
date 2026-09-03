// SPDX-License-Identifier: MIT

use crate::capsule::CapsuleHeader;
use crate::memory::{MemoryDescriptor, PhysicalAddress};
use crate::prelude::*;
use crate::reset::ResetType;
use crate::time::{Time, TimeCapabilities};
use crate::TableHeader;

#[repr(C)]
pub struct RuntimeServices {
    pub Hdr: TableHeader,

    pub GetTime: unsafe extern "efiapi" fn(Time: *mut Time, Capabilities: *mut TimeCapabilities) -> Status,

    pub SetTime: unsafe extern "efiapi" fn(Time: *const Time) -> Status,

    pub GetWakeupTime:
        unsafe extern "efiapi" fn(Enabled: *mut bool, Pending: *mut bool, Time: *mut Time) -> Status,

    pub SetWakeupTime: unsafe extern "efiapi" fn(Enable: bool, Time: *const Time) -> Status,

    pub SetVirtualAddressMap: unsafe extern "efiapi" fn(
        MemoryMapSize: usize,
        DescriptorSize: usize,
        DescriptorVersion: u32,
        VirtualMap: *const MemoryDescriptor,
    ) -> Status,

    pub ConvertPointer: unsafe extern "efiapi" fn(DebugDisposition: usize, Address: *mut usize) -> Status,

    pub GetVariable: unsafe extern "efiapi" fn(
        VariableName: *const u16,
        VendorGuid: *const Guid,
        Attributes: *mut u32,
        DataSize: *mut usize,
        Data: *mut u8,
    ) -> Status,

    pub GetNextVariableName: unsafe extern "efiapi" fn(
        VariableNameSize: *mut usize,
        VariableName: *mut u16,
        VendorGuid: *mut Guid,
    ) -> Status,

    pub SetVariable: unsafe extern "efiapi" fn(
        VariableName: *const u16,
        VendorGuid: *const Guid,
        Attributes: u32,
        DataSize: usize,
        Data: *const u8,
    ) -> Status,

    pub GetNextHighMonotonicCount: unsafe extern "efiapi" fn(HighCount: *mut u32) -> Status,

    pub ResetSystem: unsafe extern "efiapi" fn(
        ResetType: ResetType,
        ResetStatus: Status,
        DataSize: usize,
        ResetData: *const u8,
    ) -> !,

    pub UpdateCapsule: unsafe extern "efiapi" fn(
        CapsuleHeaderArray: *const *const CapsuleHeader,
        CapsuleCount: usize,
        ScatterGatherList: PhysicalAddress,
    ) -> Status,

    pub QueryCapsuleCapabilities: unsafe extern "efiapi" fn(
        CapsuleHeaderArray: *const *const CapsuleHeader,
        CapsuleCount: usize,
        MaximumCapsuleSize: *mut u64,
        ResetType: *mut ResetType,
    ) -> Status,

    pub QueryVariableInfo: unsafe extern "efiapi" fn(
        Attributes: u32,
        MaximumVariableStorageSize: *mut u64,
        RemainingVariableStorageSize: *mut u64,
        MaximumVariableSize: *mut u64,
    ) -> Status,
}
