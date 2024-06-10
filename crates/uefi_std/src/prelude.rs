pub use uefi::prelude::*;

// These common items exist in alloc and are exported by the std prelude.
// https://doc.rust-lang.org/stable/std/prelude/index.html

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
