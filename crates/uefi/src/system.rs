// SPDX-License-Identifier: MIT

use core::slice;

use crate::boot::BootServices;
use crate::config::ConfigurationTable;
use crate::prelude::*;
use crate::runtime::RuntimeServices;
use crate::text::{TextInput, TextOutput};
use crate::TableHeader;

#[repr(C)]
pub struct SystemTable {
    pub Hdr: TableHeader,
    pub FirmwareVendor: *const u16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: Handle,
    pub ConsoleIn: *mut TextInput,
    pub ConsoleOutHandle: Handle,
    pub ConsoleOut: *mut TextOutput,
    pub ConsoleErrorHandle: Handle,
    pub ConsoleError: *mut TextOutput,
    pub RuntimeServices: *mut RuntimeServices,
    pub BootServices: *mut BootServices,
    Entries: usize,
    ConfigurationTables: *const ConfigurationTable,
}

impl SystemTable {
    pub fn config_tables(&self) -> &'static [ConfigurationTable] {
        unsafe { slice::from_raw_parts(self.ConfigurationTables, self.Entries) }
    }
}
