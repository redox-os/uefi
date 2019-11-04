/* This section was addapted from the Rust Standard Library, and is licensed accordingly
 * https://raw.githubusercontent.com/rust-lang/rust/master/src/libstd/prelude/v1.rs
 * {
 */

 // Re-exported core operators
 #[doc(no_inline)]
 pub use crate::marker::{Send, Sized, Sync, Unpin};
 #[doc(no_inline)]
 pub use crate::ops::{Drop, Fn, FnMut, FnOnce};

 // Re-exported functions
 #[doc(no_inline)]
 pub use crate::mem::drop;

 // Re-exported types and traits
 #[doc(no_inline)]
 pub use crate::convert::{AsRef, AsMut, Into, From};
 #[doc(no_inline)]
 pub use crate::iter::{Iterator, Extend, IntoIterator};
 #[doc(no_inline)]
 pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
 #[doc(no_inline)]
 pub use crate::option::Option::{self, Some, None};
 #[doc(no_inline)]
 pub use crate::result::Result::{self, Ok, Err};

 // Re-exported built-in macros
 #[doc(no_inline)]
 pub use core::prelude::v1::{
     asm,
     assert,
     cfg,
     column,
     compile_error,
     concat,
     concat_idents,
     env,
     file,
     format_args,
     format_args_nl,
     global_asm,
     include,
     include_bytes,
     include_str,
     line,
     log_syntax,
     module_path,
     option_env,
     stringify,
     trace_macros,
 };

 // FIXME: Attribute and derive macros are not documented because for them rustdoc generates
 // dead links which fail link checker testing.
 #[allow(deprecated)]
 #[doc(hidden)]
 pub use core::prelude::v1::{
     Clone,
     Copy,
     Debug,
     Default,
     Eq,
     Hash,
     Ord,
     PartialEq,
     PartialOrd,
     RustcDecodable,
     RustcEncodable,
     bench,
     global_allocator,
     test,
     test_case,
 };

 // The file so far is equivalent to src/libcore/prelude/v1.rs,
 // and below to src/liballoc/prelude.rs.
 // Those files are duplicated rather than using glob imports
 // because we want docs to show these re-exports as pointing to within `std`.


 #[doc(no_inline)]
 pub use crate::boxed::Box;
 #[doc(no_inline)]
 pub use crate::borrow::ToOwned;
 #[doc(no_inline)]
 pub use crate::string::{String, ToString};
 #[doc(no_inline)]
 pub use crate::vec::Vec;

/* } */
