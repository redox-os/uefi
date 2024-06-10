#![no_std]

extern crate alloc as alloc_crate;

#[macro_use]
mod macros;

// Re-export uefi and uefi_alloc
pub use uefi;
pub use uefi_alloc;

// Runtime support
pub mod rt;

// Public modules
pub mod exec;
pub mod ffi;
pub mod fs;
pub mod io;
pub mod loaded_image;
pub mod pointer;
pub mod prelude;
pub mod proto;
pub mod shell;
pub mod vars;

use uefi::prelude::*;

static mut HANDLE: Handle = Handle(0);
static mut SYSTEM_TABLE: *mut SystemTable = 0 as *mut SystemTable;

pub fn handle() -> Handle {
    unsafe { HANDLE }
}

pub fn system_table() -> &'static SystemTable {
    unsafe { &*SYSTEM_TABLE }
}

pub unsafe fn system_table_mut() -> &'static mut SystemTable {
    &mut *SYSTEM_TABLE
}
