use crate::memory::{MemoryDescriptor, MemoryType};
use crate::prelude::*;
use crate::TableHeader;

#[repr(C)]
pub enum InterfaceType {
    Native,
}

#[repr(C)]
pub enum LocateSearchType {
    /// Retrieve all the handles in the handle database.
    AllHandles,
    /// Retrieve the next handle fron a RegisterProtocolNotify() event.
    ByRegisterNotify,
    /// Retrieve the set of handles from the handle database that support a specified protocol.
    ByProtocol,
}

#[repr(C)]
pub struct BootServices {
    pub Hdr: TableHeader,
    RaiseTpl: extern "efiapi" fn(NewTpl: Tpl) -> usize,
    RestoreTpl: extern "efiapi" fn(OldTpl: Tpl),
    pub AllocatePages: extern "efiapi" fn(
        AllocType: usize,
        MemoryType: MemoryType,
        Pages: usize,
        Memory: &mut usize,
    ) -> Status,
    pub FreePages: extern "efiapi" fn(Memory: usize, Pages: usize) -> Status,
    pub GetMemoryMap: extern "efiapi" fn(
        MemoryMapSize: &mut usize,
        MemoryMap: *mut MemoryDescriptor,
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptorVersion: &mut u32,
    ) -> Status,
    pub AllocatePool:
        extern "efiapi" fn(PoolType: MemoryType, Size: usize, Buffer: &mut usize) -> Status,
    pub FreePool: extern "efiapi" fn(Buffer: usize) -> Status,
    pub CreateEvent: extern "efiapi" fn(
        Kind: u32,
        NotifyTpl: Tpl,
        NotifyFunction: extern "efiapi" fn(Event: Event, Context: usize /* *mut c_void */),
        NotifyContext: usize, /* *mut c_void */
        Event: &mut Event,
    ) -> Status,
    SetTimer: extern "efiapi" fn(),
    pub WaitForEvent:
        extern "efiapi" fn(NumberOfEvents: usize, Event: *const Event, Index: &mut usize) -> Status,
    SignalEvent: extern "efiapi" fn(),
    CloseEvent: extern "efiapi" fn(),
    CheckEvent: extern "efiapi" fn(),
    pub InstallProtocolInterface: extern "efiapi" fn(
        Handle: &mut Handle,
        Protocol: &Guid,
        InterfaceType: InterfaceType,
        Interface: usize,
    ) -> Status,
    ReinstallProtocolInterface: extern "efiapi" fn(),
    pub UninstallProtocolInterface:
        extern "efiapi" fn(Handle: Handle, Protocol: &Guid, Interface: usize) -> Status,
    pub HandleProtocol:
        extern "efiapi" fn(Handle: Handle, Protocol: &Guid, Interface: &mut usize) -> Status,
    _rsvd: usize,
    RegisterProtocolNotify: extern "efiapi" fn(),
    pub LocateHandle: extern "efiapi" fn(
        SearchType: LocateSearchType,
        Protocol: &Guid,
        SearchKey: usize,
        BufferSize: &mut usize,
        Buffer: *mut Handle,
    ) -> Status,
    LocateDevicePath: extern "efiapi" fn(),
    InstallConfigurationTable: extern "efiapi" fn(),
    pub LoadImage: extern "efiapi" fn(
        BootPolicy: bool,
        ParentImageHandle: Handle,
        DevicePath: usize, /*TODO*/
        SourceBuffer: *const u8,
        SourceSize: usize,
        ImageHandle: &mut Handle,
    ) -> Status,
    pub StartImage: extern "efiapi" fn(
        ImageHandle: Handle,
        ExitDataSize: &mut usize,
        ExitData: &mut *mut u16,
    ) -> Status,
    pub Exit: extern "efiapi" fn(
        ImageHandle: Handle,
        ExitStatus: isize,
        ExitDataSize: usize,
        ExitData: *const u16,
    ) -> Status,
    UnloadImage: extern "efiapi" fn(),
    pub ExitBootServices: extern "efiapi" fn(ImageHandle: Handle, MapKey: usize) -> Status,
    GetNextMonotonicCount: extern "efiapi" fn(),
    pub Stall: extern "efiapi" fn(Microseconds: usize) -> Status,
    pub SetWatchdogTimer: extern "efiapi" fn(
        Timeout: usize,
        WatchdogCode: u64,
        DataSize: usize,
        WatchdogData: *const u16,
    ) -> Status,
    ConnectController: extern "efiapi" fn(),
    DisconnectController: extern "efiapi" fn(),
    OpenProtocol: extern "efiapi" fn(),
    CloseProtocol: extern "efiapi" fn(),
    OpenProtocolInformation: extern "efiapi" fn(),
    pub ProtocolsPerHandle: extern "efiapi" fn(
        Handle: Handle,
        ProtocolBuffer: *mut Guid,
        ProtocolBufferCount: usize,
    ) -> Status,
    LocateHandleBuffer: extern "efiapi" fn(
        SearchType: LocateSearchType,
        Protocol: &Guid,
        SearchKey: usize,
        NoHandles: &mut usize,
        Buffer: &mut *mut Handle,
    ),
    pub LocateProtocol:
        extern "efiapi" fn(Protocol: &Guid, Registration: usize, Interface: &mut usize) -> Status,
    InstallMultipleProtocolInterfaces: extern "efiapi" fn(),
    UninstallMultipleProtocolInterfaces: extern "efiapi" fn(),
    CalculateCrc32: extern "efiapi" fn(),
    CopyMem: extern "efiapi" fn(),
    SetMem: extern "efiapi" fn(),
    pub CreateEventEx: extern "efiapi" fn(
        Kind: u32,
        NotifyTpl: Tpl,
        NotifyFunction: extern "efiapi" fn(Event: Event, Context: usize /* *mut c_void */),
        NotifyContext: usize, /* *mut c_void */
        EventGroup: *const Guid,
        Event: &mut Event,
    ) -> Status,
}
