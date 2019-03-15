/* This section was addapted from the Rust Standard Library, and is licensed accordingly
 * https://raw.githubusercontent.com/rust-lang/rust/master/src/libstd/prelude/v1.rs
 * {
 */

// Re-exported core operators
#[doc(no_inline)]
pub use crate::marker::{Copy, Send, Sized, Sync, Unpin};
#[doc(no_inline)]
pub use crate::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[doc(no_inline)]
pub use crate::mem::drop;

// Re-exported types and traits
#[doc(no_inline)]
pub use crate::clone::Clone;
#[doc(no_inline)]
pub use crate::cmp::{PartialEq, PartialOrd, Eq, Ord};
#[doc(no_inline)]
pub use crate::convert::{AsRef, AsMut, Into, From};
#[doc(no_inline)]
pub use crate::default::Default;
#[doc(no_inline)]
pub use crate::iter::{Iterator, Extend, IntoIterator};
#[doc(no_inline)]
pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
#[doc(no_inline)]
pub use crate::option::Option::{self, Some, None};
#[doc(no_inline)]
pub use crate::result::Result::{self, Ok, Err};


// The file so far is equivalent to src/libcore/prelude/v1.rs,
// and below to src/liballoc/prelude.rs.
// Those files are duplicated rather than using glob imports
// because we want docs to show these re-exports as pointing to within `std`.


#[doc(no_inline)]
pub use crate::boxed::Box;
#[doc(no_inline)]
pub use crate::borrow::ToOwned;
#[doc(no_inline)]
pub use crate::slice::SliceConcatExt;
#[doc(no_inline)]
pub use crate::string::{String, ToString};
#[doc(no_inline)]
pub use crate::vec::Vec;

/* } */
