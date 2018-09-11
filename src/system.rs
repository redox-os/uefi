//! UEFI uses the EFI System Table, which contains pointers to the runtime and boot services tables.
//! The definition for this table is shown in the following code fragments. Except for the table header,
//! all elements in the service tables are pointers to functions as defined in Section 7 and Section 8.
//! Prior to a call to EFI_BOOT_SERVICES.ExitBootServices(), all of the fields of the EFI System
//! Table are valid. After an operating system has taken control of the platform with a call to
//! ExitBootServices(), only the Hdr, FirmwareVendor, FirmwareRevision,
//! RuntimeServices, NumberOfTableEntries, and ConfigurationTable fields are valid.

use core::slice;

use crate::{
    boot::BootServices,
    config::ConfigurationTable,
    runtime::RuntimeServices,
    text::{TextInput, TextOutput},
    Handle, TableHeader,
};

/// Contains pointers to the runtime and boot services tables.
#[repr(C)]
pub struct SystemTable {
    /// The table header for the EFI System Table. This header contains the
    /// EFI_SYSTEM_TABLE_SIGNATURE and
    /// EFI_SYSTEM_TABLE_REVISION values along with the size of
    /// the EFI_SYSTEM_TABLE structure and a 32-bit CRC to verify
    /// that the contents of the EFI System Table are valid.
    pub Hdr: TableHeader,
    /// A pointer to a null terminated string that identifies the vendor
    /// that produces the system firmware for the platform.
    pub FirmwareVendor: *const u16,
    /// A firmware vendor specific value that identifies the revision of
    /// the system firmware for the platform.
    pub FirmwareRevision: u32,
    /// The handle for the active console input device. This handle must
    /// support EFI_SIMPLE_TEXT_INPUT_PROTOCOL and
    /// EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.
    pub ConsoleInHandle: Handle,
    /// A reference to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    /// interface that is associated with `ConsoleInHandle`.
    pub ConsoleIn: &'static mut TextInput,
    /// The handle for the active console output device. This handle
    /// must support the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
    pub ConsoleOutHandle: Handle,
    /// A reference to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
    /// interface that is associated with `ConsoleOutHandle`.
    pub ConsoleOut: &'static mut TextOutput,
    /// The handle for the active standard error console device. This
    /// handle must support the
    /// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
    pub ConsoleErrorHandle: Handle,
    /// A reference to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
    /// interface that is associated with `ConsoleErrorHandle`.
    pub ConsoleError: &'static mut TextOutput,
    /// A reference to the EFI Runtime Services Table. See Section 4.5.
    pub RuntimeServices: &'static mut RuntimeServices,
    /// A reference to the EFI Boot Services Table. See Section 4.4.
    pub BootServices: &'static mut BootServices,
    /// The number of system configuration tables in the buffer
    /// `ConfigurationTables`.
    Entries: usize,
    /// A pointer to the system configuration tables. The number of
    /// entries in the table is `Entries`.
    ConfigurationTables: *const ConfigurationTable,
}

impl SystemTable {
    /// Returns a slice to all the configuration tables available.
    pub fn config_tables(&self) -> &'static [ConfigurationTable] {
        // This is safe under the assumption that the firmware supplied valid values.
        unsafe { slice::from_raw_parts(self.ConfigurationTables, self.Entries) }
    }
}
