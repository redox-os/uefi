#![no_std]
#![feature(prelude_import)]

/* This section was addapted from the Rust Standard Library, and is licensed accordingly
 * https://github.com/rust-lang/rust/blob/master/src/libstd/lib.rs
 * {
 */

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::*;

#[allow(unused_imports)]
#[macro_use]
extern crate alloc as alloc_crate;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and re-exports
pub use alloc_crate::borrow;
pub use alloc_crate::boxed;
pub use alloc_crate::fmt;
pub use alloc_crate::format;
pub use alloc_crate::rc;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::vec;
pub use core::any;
pub use core::arch;
pub use core::array;
pub use core::cell;
pub use core::char;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::hint;
pub use core::i128;
pub use core::i16;
pub use core::i32;
pub use core::i64;
pub use core::i8;
pub use core::isize;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::option;
pub use core::pin;
pub use core::ptr;
pub use core::result;
pub use core::u128;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use core::u8;
pub use core::usize;

// Re-export macros defined in libcore.
#[allow(deprecated, deprecated_in_future)]
pub use core::{
    // Stable
    assert_eq,
    assert_ne,
    debug_assert,
    debug_assert_eq,
    debug_assert_ne,
    r#try,
    // Unstable
    todo,
    unimplemented,
    unreachable,
    write,
    writeln,
};

// Re-export built-in macros defined through libcore.
pub use core::prelude::rust_2021::{
    // Stable
    assert,
    cfg,
    column,
    compile_error,
    concat,
    // Unstable
    env,
    file,
    include,
    include_bytes,
    include_str,
    line,
    module_path,
    option_env,
    stringify,
};

/* } */

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
pub mod proto;
pub mod shell;
pub mod vars;

static mut HANDLE: uefi::Handle = uefi::Handle(0);
static mut SYSTEM_TABLE: *mut uefi::system::SystemTable = 0 as *mut uefi::system::SystemTable;

pub fn handle() -> uefi::Handle {
    unsafe { HANDLE }
}

pub fn system_table() -> &'static uefi::system::SystemTable {
    unsafe { &*SYSTEM_TABLE }
}

pub unsafe fn system_table_mut() -> &'static mut uefi::system::SystemTable {
    &mut *SYSTEM_TABLE
}
