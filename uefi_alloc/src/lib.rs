#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(try_trait)]
#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ops::Try;
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
        if (uefi.as_ref().BootServices.AllocatePool)(
            MemoryType::EfiLoaderData,
            layout.size(),
            &mut ptr,
        )
        .into_result()
        .is_ok()
        {
            ptr as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let uefi = UEFI.expect("__rust_deallocate: uefi not initialized");
        let _ = (uefi.as_ref().BootServices.FreePool)(ptr as usize);
    }
}
