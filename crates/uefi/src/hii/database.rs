use crate::prelude::*;

use super::package::{HiiPackageHeader, HiiPackageKind, HiiPackageListHeader};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct HiiHandle(pub usize);

pub type HiiDatabaseNotify = extern "efiapi" fn(
    PackageKind: HiiPackageKind,
    PackageGuid: &Guid,
    Package: &HiiPackageHeader,
    Handle: HiiHandle,
    NotifyKind: HiiDatabaseNotifyKind,
) -> Status;

#[repr(usize)]
pub enum HiiDatabaseNotifyKind {
    NewPack = 1,
    RemovePack = 2,
    ExportPack = 4,
    AddPack = 8,
}

#[repr(C)]
pub struct HiiKeyboardLayout {
    /// The length of the current keyboard layout
    pub LayoutLength: u16,
    /// The unique ID associated with this keyboard layout
    pub Guid: Guid,
    /// An offset location of the string which describes this keyboard layout, as a DescriptionStringBundle
    pub LayoutDescriptorStringOffset: u32,
    /// The number of Descriptor entries in this layout
    pub DescriptorCount: u8,
    //Descriptors: [KeyDescriptor; DescriptorCount]
}

#[repr(C)]
pub struct HiiDatabase {
    /// Adds the packages in the package list to the HII database
    pub NewPackageList: extern "efiapi" fn(
        &HiiDatabase,
        PackageList: &HiiPackageListHeader,
        DriverHandle: Handle,
        Handle: &mut HiiHandle,
    ) -> Status,

    /// Removes a package list from the HII database
    pub RemovePackageList: extern "efiapi" fn(&HiiDatabase, Handle: HiiHandle) -> Status,

    /// Update a package list in the HII database
    pub UpdatePackageList: extern "efiapi" fn(
        &HiiDatabase,
        Handle: HiiHandle,
        PackageList: &HiiPackageListHeader,
    ) -> Status,

    /// Determines the handles that are currently active in the database
    pub ListPackageLists: extern "efiapi" fn(
        &HiiDatabase,
        PackageKind: HiiPackageKind,
        PackageGuid: &Guid,
        HandleBufferLength: &mut usize,
        Handle: *mut HiiHandle,
    ) -> Status,

    /// Exports the contents of one or all package lists in the HII database into a buffer
    pub ExportPackageLists: extern "efiapi" fn(
        &HiiDatabase,
        Handle: HiiHandle,
        BufferSize: &mut usize,
        Buffer: &mut HiiPackageListHeader,
    ) -> Status,

    /// Registers a notification function for HII database-related events
    pub RegisterPackageNotify: extern "efiapi" fn(
        &HiiDatabase,
        PackageKind: HiiPackageKind,
        PackageGuid: &Guid,
        PackageNotifyFn: HiiDatabaseNotify,
        NotifyKind: HiiDatabaseNotifyKind,
        NotifyHandle: &mut Handle,
    ) -> Status,

    /// Removes the specified HII database package-related notification
    pub UnregisterPackageNotify:
        extern "efiapi" fn(&HiiDatabase, NotificationHandle: Handle) -> Status,

    /// Retrieves a list of the keyboard layouts in the system
    pub FindKeyboardLayouts: extern "efiapi" fn(
        &HiiDatabase,
        KeyGuidBufferLength: &mut u16,
        KeyGuidBuffer: *mut Guid,
    ) -> Status,

    /// Retrieves the requested keyboard layout
    pub GetKeyboardLayout: extern "efiapi" fn(
        &HiiDatabase,
        KeyGuid: &Guid,
        KeyboardLayoutLength: &mut u16,
        KeyboardLayout: *mut HiiKeyboardLayout,
    ) -> Status,

    /// Sets the currently active keyboard layout
    pub SetKeyboardLayout: extern "efiapi" fn(&HiiDatabase, KeyGuid: &Guid) -> Status,

    /// Return the EFI handle associated with a package list
    pub GetPackageListHandle: extern "efiapi" fn(
        &HiiDatabase,
        PackageListHandle: HiiHandle,
        DriverHandle: &mut Handle,
    ) -> Status,
}

impl HiiDatabase {
    pub const GUID: Guid = guid!("ef9fc172-a1b2-4693-b327-6d32fc416042");
}
