pub use uefi::prelude::*;

/* This section was addapted from the Rust Standard Library, and is licensed accordingly
 * https://raw.githubusercontent.com/rust-lang/rust/master/src/libstd/prelude/v1.rs
 * {
 */

// Re-exported core operators
#[doc(no_inline)]
pub use core::marker::{Send, Sized, Sync, Unpin};
#[doc(no_inline)]
pub use core::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[doc(no_inline)]
pub use core::mem::drop;

// Re-exported types and traits
#[doc(no_inline)]
pub use core::convert::{AsMut, AsRef, From, Into};
#[doc(no_inline)]
pub use core::iter::{DoubleEndedIterator, ExactSizeIterator};
#[doc(no_inline)]
pub use core::iter::{Extend, IntoIterator, Iterator};
#[doc(no_inline)]
pub use core::option::Option::{self, None, Some};

// The file so far is equivalent to src/libcore/prelude/v1.rs,
// and below to src/liballoc/prelude.rs.
// Those files are duplicated rather than using glob imports
// because we want docs to show these re-exports as pointing to within `std`.

#[doc(no_inline)]
pub use alloc_crate::borrow::ToOwned;
#[doc(no_inline)]
pub use alloc_crate::boxed::Box;
#[doc(no_inline)]
pub use alloc_crate::format;
#[doc(no_inline)]
pub use alloc_crate::string::{String, ToString};
#[doc(no_inline)]
pub use alloc_crate::vec;
#[doc(no_inline)]
pub use alloc_crate::vec::Vec;

/* } */
