#![feature(allocator_api)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ops::{ControlFlow, Try};
use core::ptr::{self, NonNull};
use uefi::memory::MemoryType;
use uefi::system::SystemTable;

static mut UEFI: Option<NonNull<SystemTable>> = None;

pub unsafe fn init(table: &'static mut SystemTable) {
    UEFI = NonNull::new(table);
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let uefi = UEFI.expect("__rust_allocate: uefi not initialized");
        let mut ptr = 0;
        let res = (uefi.as_ref().BootServices.AllocatePool)(
            MemoryType::EfiLoaderData,
            layout.size(),
            &mut ptr,
        )
        .branch();

        match res {
            ControlFlow::Continue(ptr) => ptr as *mut u8,
            ControlFlow::Break(_) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let uefi = UEFI.expect("__rust_deallocate: uefi not initialized");
        let _ = (uefi.as_ref().BootServices.FreePool)(ptr as usize);
    }
}
