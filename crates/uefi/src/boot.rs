// SPDX-License-Identifier: MIT

use crate::memory::{MemoryDescriptor, MemoryType};
use crate::prelude::*;
use crate::TableHeader;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct InterfaceType(pub u32);

impl InterfaceType {
    pub const NATIVE: Self = Self(0);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct LocateSearchType(pub u32);

impl LocateSearchType {
    /// Retrieve all the handles in the handle database.
    pub const ALL_HANDLES: Self = Self(0);
    /// Retrieve the next handle fron a RegisterProtocolNotify() event.
    pub const BY_REGISTER_NOTIFY: Self = Self(1);
    /// Retrieve the set of handles from the handle database that support a specified protocol.
    pub const BY_PROTOCOL: Self = Self(2);
}

#[repr(C)]
pub struct BootServices {
    pub Hdr: TableHeader,
    RaiseTpl: unsafe extern "efiapi" fn(NewTpl: Tpl) -> usize,
    RestoreTpl: unsafe extern "efiapi" fn(OldTpl: Tpl),
    pub AllocatePages: unsafe extern "efiapi" fn(
        AllocType: usize,
        MemoryType: MemoryType,
        Pages: usize,
        Memory: &mut usize,
    ) -> Status,
    pub FreePages: unsafe extern "efiapi" fn(Memory: usize, Pages: usize) -> Status,
    pub GetMemoryMap: unsafe extern "efiapi" fn(
        MemoryMapSize: &mut usize,
        MemoryMap: *mut MemoryDescriptor,
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptorVersion: &mut u32,
    ) -> Status,
    pub AllocatePool:
        unsafe extern "efiapi" fn(PoolType: MemoryType, Size: usize, Buffer: &mut usize) -> Status,
    pub FreePool: unsafe extern "efiapi" fn(Buffer: usize) -> Status,
    pub CreateEvent: unsafe extern "efiapi" fn(
        Kind: u32,
        NotifyTpl: Tpl,
        NotifyFunction: unsafe extern "efiapi" fn(Event: Event, Context: usize /* *mut c_void */),
        NotifyContext: usize, /* *mut c_void */
        Event: &mut Event,
    ) -> Status,
    SetTimer: unsafe extern "efiapi" fn(),
    pub WaitForEvent:
        unsafe extern "efiapi" fn(NumberOfEvents: usize, Event: *const Event, Index: &mut usize) -> Status,
    SignalEvent: unsafe extern "efiapi" fn(),
    CloseEvent: unsafe extern "efiapi" fn(),
    CheckEvent: unsafe extern "efiapi" fn(),
    pub InstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: &mut Handle,
        Protocol: &Guid,
        InterfaceType: InterfaceType,
        Interface: usize,
    ) -> Status,
    ReinstallProtocolInterface: unsafe extern "efiapi" fn(),
    pub UninstallProtocolInterface:
        unsafe extern "efiapi" fn(Handle: Handle, Protocol: &Guid, Interface: usize) -> Status,
    pub HandleProtocol:
        unsafe extern "efiapi" fn(Handle: Handle, Protocol: &Guid, Interface: &mut usize) -> Status,
    _rsvd: usize,
    RegisterProtocolNotify: unsafe extern "efiapi" fn(),
    pub LocateHandle: unsafe extern "efiapi" fn(
        SearchType: LocateSearchType,
        Protocol: *const Guid,
        SearchKey: *const usize,
        BufferSize: &mut usize,
        Buffer: *mut Handle,
    ) -> Status,
    LocateDevicePath: unsafe extern "efiapi" fn(),
    InstallConfigurationTable: unsafe extern "efiapi" fn(),
    pub LoadImage: unsafe extern "efiapi" fn(
        BootPolicy: bool,
        ParentImageHandle: Handle,
        DevicePath: usize, /*TODO*/
        SourceBuffer: *const u8,
        SourceSize: usize,
        ImageHandle: &mut Handle,
    ) -> Status,
    pub StartImage: unsafe extern "efiapi" fn(
        ImageHandle: Handle,
        ExitDataSize: &mut usize,
        ExitData: &mut *mut u16,
    ) -> Status,
    pub Exit: unsafe extern "efiapi" fn(
        ImageHandle: Handle,
        ExitStatus: isize,
        ExitDataSize: usize,
        ExitData: *const u16,
    ) -> Status,
    UnloadImage: unsafe extern "efiapi" fn(),
    pub ExitBootServices: unsafe extern "efiapi" fn(ImageHandle: Handle, MapKey: usize) -> Status,
    GetNextMonotonicCount: unsafe extern "efiapi" fn(),
    pub Stall: unsafe extern "efiapi" fn(Microseconds: usize) -> Status,
    pub SetWatchdogTimer: unsafe extern "efiapi" fn(
        Timeout: usize,
        WatchdogCode: u64,
        DataSize: usize,
        WatchdogData: *const u16,
    ) -> Status,
    ConnectController: unsafe extern "efiapi" fn(),
    DisconnectController: unsafe extern "efiapi" fn(),
    OpenProtocol: unsafe extern "efiapi" fn(),
    CloseProtocol: unsafe extern "efiapi" fn(),
    OpenProtocolInformation: unsafe extern "efiapi" fn(),
    pub ProtocolsPerHandle: unsafe extern "efiapi" fn(
        Handle: Handle,
        ProtocolBuffer: *mut Guid,
        ProtocolBufferCount: usize,
    ) -> Status,
    pub LocateHandleBuffer: unsafe extern "efiapi" fn(
        SearchType: LocateSearchType,
        Protocol: *const Guid,
        SearchKey: *const usize,
        NoHandles: &mut usize,
        Buffer: &mut *mut Handle,
    ) -> Status,
    pub LocateProtocol:
        unsafe extern "efiapi" fn(Protocol: &Guid, Registration: usize, Interface: &mut usize) -> Status,
    InstallMultipleProtocolInterfaces: unsafe extern "efiapi" fn(),
    UninstallMultipleProtocolInterfaces: unsafe extern "efiapi" fn(),
    CalculateCrc32: unsafe extern "efiapi" fn(),
    CopyMem: unsafe extern "efiapi" fn(),
    SetMem: unsafe extern "efiapi" fn(),
    pub CreateEventEx: unsafe extern "efiapi" fn(
        Kind: u32,
        NotifyTpl: Tpl,
        NotifyFunction: unsafe extern "efiapi" fn(Event: Event, Context: usize /* *mut c_void */),
        NotifyContext: usize, /* *mut c_void */
        EventGroup: *const Guid,
        Event: &mut Event,
    ) -> Status,
}
