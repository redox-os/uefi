#![no_std]

#[macro_use]
mod macros;

pub mod io;
pub mod math;
pub mod rt;

#[global_allocator]
static ALLOCATOR: uefi_alloc::Allocator = uefi_alloc::Allocator;

pub static mut HANDLE: uefi::Handle = uefi::Handle(0);
pub static mut UEFI: *mut uefi::system::SystemTable = 0 as *mut uefi::system::SystemTable;
