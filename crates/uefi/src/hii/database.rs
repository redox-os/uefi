// SPDX-License-Identifier: MIT

use crate::prelude::*;
use super::HiiHandle;
use super::package::{HiiPackageHeader, HiiPackageKind, HiiPackageListHeader};

/// `EFI_HII_DATABASE_NOTIFY`
pub type HiiDatabaseNotify = unsafe extern "efiapi" fn(
    PackageKind: HiiPackageKind,
    PackageGuid: &Guid,
    Package: &HiiPackageHeader,
    Handle: HiiHandle,
    NotifyKind: HiiDatabaseNotifyType,
) -> Status;

/// `EFI_HII_DATABASE_NOTIFY_TYPE`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct HiiDatabaseNotifyType(pub usize);

impl HiiDatabaseNotifyType {
    pub const NEW_PACK: Self = Self(1 << 0);
    pub const REMOVE_PACK: Self = Self(1 << 1);
    pub const EXPORT_PACK: Self = Self(1 << 2);
    pub const ADD_PACK: Self = Self(1 << 3);
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
    pub NewPackageList: unsafe extern "efiapi" fn(
        This: *mut Self,
        PackageList: *const HiiPackageListHeader,
        DriverHandle: Handle,
        Handle: *mut HiiHandle,
    ) -> Status,

    /// Removes a package list from the HII database
    pub RemovePackageList: unsafe extern "efiapi" fn(This: *mut Self, Handle: HiiHandle) -> Status,

    /// Update a package list in the HII database
    pub UpdatePackageList: unsafe extern "efiapi" fn(
        This: *mut Self,
        Handle: HiiHandle,
        PackageList: *const HiiPackageListHeader,
    ) -> Status,

    /// Determines the handles that are currently active in the database
    pub ListPackageLists: unsafe extern "efiapi" fn(
        This: *mut Self,
        PackageKind: HiiPackageKind,
        PackageGuid: *const Guid,
        HandleBufferLength: *mut usize,
        Handle: *mut HiiHandle,
    ) -> Status,

    /// Exports the contents of one or all package lists in the HII database into a buffer
    pub ExportPackageLists: unsafe extern "efiapi" fn(
        This: *mut Self,
        Handle: HiiHandle,
        BufferSize: *mut usize,
        Buffer: *mut HiiPackageListHeader,
    ) -> Status,

    /// Registers a notification function for HII database-related events
    pub RegisterPackageNotify: unsafe extern "efiapi" fn(
        This: *mut Self,
        PackageKind: HiiPackageKind,
        PackageGuid: *const Guid,
        PackageNotifyFn: HiiDatabaseNotify,
        NotifyKind: HiiDatabaseNotifyType,
        NotifyHandle: *mut Handle,
    ) -> Status,

    /// Removes the specified HII database package-related notification
    pub UnregisterPackageNotify:
        unsafe extern "efiapi" fn(This: *mut Self, NotificationHandle: Handle) -> Status,

    /// Retrieves a list of the keyboard layouts in the system
    pub FindKeyboardLayouts: unsafe extern "efiapi" fn(
        This: *mut Self,
        KeyGuidBufferLength: *mut u16,
        KeyGuidBuffer: *mut Guid,
    ) -> Status,

    /// Retrieves the requested keyboard layout
    pub GetKeyboardLayout: unsafe extern "efiapi" fn(
        This: *mut Self,
        KeyGuid: *const Guid,
        KeyboardLayoutLength: *mut u16,
        KeyboardLayout: *mut HiiKeyboardLayout,
    ) -> Status,

    /// Sets the currently active keyboard layout
    pub SetKeyboardLayout: unsafe extern "efiapi" fn(This: *mut Self, KeyGuid: *const Guid) -> Status,

    /// Return the EFI handle associated with a package list
    pub GetPackageListHandle: unsafe extern "efiapi" fn(
        This: *mut Self,
        PackageListHandle: HiiHandle,
        DriverHandle: *mut Handle,
    ) -> Status,
}

impl HiiDatabase {
    pub const GUID: Guid = guid!("ef9fc172-a1b2-4693-b327-6d32fc416042");
}
