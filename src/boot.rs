//! UEFI uses the EFI Boot Services Table, which contains a table header and pointers to all of the boot
//! services. The definition for this table is shown in the following code fragments. Except for the table
//! header, all elements in the EFI Boot Services Tables are prototypes of function pointers to functions
//! as defined in Section 7. The function pointers in this table are not valid after the operating system
//! has taken control of the platform with a call to EFI_BOOT_SERVICES.ExitBootServices().

use crate::{
    guid::Guid,
    memory::{MemoryDescriptor, MemoryType},
    status::Status,
    Event, Handle, TableHeader,
};

/// Indicates whether Interface is supplied in native form.
#[repr(C)]
pub enum InterfaceType {
    /// Interface is supplied in native form.
    Native,
}

/// Specifies which handle(s) are to be returned.
#[repr(C)]
pub enum LocateSearchType {
    /// Retrieve all the handles in the handle database.
    AllHandles,
    /// SearchKey supplies the Registration value returned by
    /// EFI_BOOT_SERVICES.RegisterProtocolNotify(). The
    /// function returns the next handle that is new for the registration.
    /// Only one handle is returned at a time, starting with the first, and the
    /// caller must loop until no more handles are returned. Protocol is
    /// ignored for this search type.
    ByRegisterNotify,
    /// All handles that support Protocol are returned. SearchKey is ignored
    /// for this search type.
    ByProtocol,
}

/// Contains a table header and pointers to all of the boot services.
#[repr(C)]
pub struct BootServices {
    /// The table header for the EFI Boot Services Table. This header
    /// contains the EFI_BOOT_SERVICES_SIGNATURE and
    /// EFI_BOOT_SERVICES_REVISION values along with the size of
    /// the EFI_BOOT_SERVICES structure and a 32-bit CRC to verify
    /// that the contents of the EFI Boot Services Table are valid.
    pub Hdr: TableHeader,
    /// Raises the task priority level.
    RaiseTpl: extern "win64" fn(NewTpl: usize) -> usize,
    /// Restores/lowers the task priority level.
    RestoreTpl: extern "win64" fn(OldTpl: usize),
    /// Allocates pages of a particular type.
    pub AllocatePages: extern "win64" fn(
        AllocType: usize,
        MemoryType: MemoryType,
        Pages: usize,
        Memory: &mut usize,
    ) -> Status,
    /// Frees allocated pages.
    pub FreePages: extern "win64" fn(Memory: usize, Pages: usize) -> Status,
    /// Returns the current boot services memory map and memory map key.
    pub GetMemoryMap: extern "win64" fn(
        MemoryMapSize: &mut usize,
        MemoryMap: *mut MemoryDescriptor,
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptorVersion: &mut u32,
    ) -> Status,
    /// Allocates a pool of a particular type.
    pub AllocatePool:
        extern "win64" fn(PoolType: MemoryType, Size: usize, Buffer: &mut usize) -> Status,
    /// Frees allocated pool.
    pub FreePool: extern "win64" fn(Buffer: usize) -> Status,
    /// Creates a general-purpose event structure.
    CreateEvent: extern "win64" fn(),
    /// Sets an event to be signaled at a particular time.
    SetTimer: extern "win64" fn(),
    /// Stops execution until an event is signaled.
    pub WaitForEvent:
        extern "win64" fn(NumberOfEvents: usize, Event: *const Event, Index: &mut usize) -> Status,
    /// Signals an event.
    SignalEvent: extern "win64" fn(),
    /// Closes and frees an event structure.
    CloseEvent: extern "win64" fn(),
    /// Checks whether an event is in the signaled state.
    CheckEvent: extern "win64" fn(),
    /// Installs a protocol interface on a device handle.
    pub InstallProtocolInterface: extern "win64" fn(
        Handle: &mut Handle,
        Protocol: &Guid,
        InterfaceType: InterfaceType,
        Interface: usize,
    ) -> Status,
    /// Reinstalls a protocol interface on a device handle.
    ReinstallProtocolInterface: extern "win64" fn(),
    /// Removes a protocol interface from a device handle.
    pub UninstallProtocolInterface:
        extern "win64" fn(Handle: Handle, Protocol: &Guid, Interface: usize) -> Status,
    /// Queries a handle to determine if it supports a specified protocol.
    pub HandleProtocol:
        extern "win64" fn(Handle: Handle, Protocol: &Guid, Interface: &mut usize) -> Status,
    /// Reserved. Must be NULL.
    _rsvd: usize,
    /// Registers an event that is to be signaled whenever an interface is
    /// installed for a specified protocol.
    RegisterProtocolNotify: extern "win64" fn(),
    /// Returns an array of handles that support a specified protocol.
    pub LocateHandle: extern "win64" fn(
        SearchType: LocateSearchType,
        Protocol: &Guid,
        SearchKey: usize,
        BufferSize: &mut usize,
        Buffer: *mut Handle,
    ) -> Status,
    /// Locates all devices on a device path that support a specified
    /// protocol and returns the handle to the device that is closest to
    /// the path.
    LocateDevicePath: extern "win64" fn(),
    /// Adds, updates, or removes a configuration table from the EFI
    /// System Table.
    InstallConfigurationTable: extern "win64" fn(),
    /// Loads an EFI image into memory.
    pub LoadImage: extern "win64" fn(
        BootPolicy: bool,
        ParentImageHandle: Handle,
        DevicePath: usize, /*TODO*/
        SourceBuffer: *const u8,
        SourceSize: usize,
        ImageHandle: &mut Handle,
    ) -> Status,
    /// Transfers control to a loaded image’s entry point.
    pub StartImage:
        extern "win64" fn(ImageHandle: Handle, ExitDataSize: &mut usize, ExitData: &mut *mut u16)
            -> Status,
    /// Exits the image’s entry point.
    pub Exit: extern "win64" fn(
        ImageHandle: Handle,
        ExitStatus: isize,
        ExitDataSize: usize,
        ExitData: *const u16,
    ) -> Status,
    /// Unloads an image.
    UnloadImage: extern "win64" fn(),
    /// Terminates boot services.
    pub ExitBootServices: extern "win64" fn(ImageHandle: Handle, MapKey: usize) -> Status,
    /// Returns a monotonically increasing count for the platform.
    GetNextMonotonicCount: extern "win64" fn(),
    /// Stalls the processor.
    pub Stall: extern "win64" fn(Microseconds: usize) -> Status,
    /// Resets and sets a watchdog timer used during boot services time.
    pub SetWatchdogTimer: extern "win64" fn(
        Timeout: usize,
        WatchdogCode: u64,
        DataSize: usize,
        WatchdogData: *const u16,
    ) -> Status,
    /// Uses a set of precedence rules to find the best set of drivers to
    /// manage a controller.
    ConnectController: extern "win64" fn(),
    /// Informs a set of drivers to stop managing a controller.
    DisconnectController: extern "win64" fn(),
    /// Adds elements to the list of agents consuming a protocol interface.
    OpenProtocol: extern "win64" fn(),
    /// Removes elements from the list of agents consuming a protocol
    /// interface.
    CloseProtocol: extern "win64" fn(),
    /// Retrieve the list of agents that are currently consuming a
    /// protocol interface.
    OpenProtocolInformation: extern "win64" fn(),
    /// Retrieves the list of protocols installed on a handle. The return
    /// buffer is automatically allocated.
    pub ProtocolsPerHandle:
        extern "win64" fn(Handle: Handle, ProtocolBuffer: *mut Guid, ProtocolBufferCount: usize)
            -> Status,
    /// Retrieves the list of handles from the handle database that meet
    /// the search criteria. The return buffer is automatically allocated.
    LocateHandleBuffer: extern "win64" fn(
        SearchType: LocateSearchType,
        Protocol: &Guid,
        SearchKey: usize,
        NoHandles: &mut usize,
        Buffer: &mut *mut Handle,
    ),
    /// Finds the first handle in the handle database the supports the requested protocol.
    pub LocateProtocol:
        extern "win64" fn(Protocol: &Guid, Registration: usize, Interface: &mut usize) -> Status,
    /// Installs one or more protocol interfaces onto a handle.
    InstallMultipleProtocolInterfaces: extern "win64" fn(),
    /// Uninstalls one or more protocol interfaces from a handle.
    UninstallMultipleProtocolInterfaces: extern "win64" fn(),
    /// Computes and returns a 32-bit CRC for a data buffer.
    CalculateCrc32: extern "win64" fn(),
    /// Copies the contents of one buffer to another buffer.
    CopyMem: extern "win64" fn(),
    /// Fills a buffer with a specified value.
    SetMem: extern "win64" fn(),
    /// Creates an event structure as part of an event group.
    CreateEventEx: extern "win64" fn(),
}
