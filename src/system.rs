use core::slice;

use crate::{Handle, TableHeader};
use crate::boot::BootServices;
use crate::config::ConfigurationTable;
use crate::runtime::RuntimeServices;
use crate::text::{TextInput, TextOutput};

#[repr(C)]
pub struct SystemTable {
    pub Hdr: TableHeader,
    pub FirmwareVendor: *const u16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: Handle,
    pub ConsoleIn: &'static mut TextInput,
    pub ConsoleOutHandle: Handle,
    pub ConsoleOut: &'static mut TextOutput,
    pub ConsoleErrorHandle: Handle,
    pub ConsoleError: &'static mut TextOutput,
    pub RuntimeServices: &'static mut RuntimeServices,
    pub BootServices: &'static mut BootServices,
    Entries: usize,
    ConfigurationTables: *const ConfigurationTable
}

impl SystemTable {
    pub fn config_tables(&self) -> &'static [ConfigurationTable] {
        unsafe {
            slice::from_raw_parts(self.ConfigurationTables, self.Entries)
        }
    }
}
