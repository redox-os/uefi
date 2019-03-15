use uefi::Handle;
use uefi::status::Status;
use uefi::system::SystemTable;

use crate::{HANDLE, UEFI};

#[no_mangle]
pub unsafe extern "win64" fn _start(handle: Handle, uefi: &'static mut SystemTable) -> Status {
    extern "C" {
        fn main() -> Status;
    }

    HANDLE = handle;
    UEFI = uefi;

    uefi_alloc::init(::core::mem::transmute(&mut *UEFI));

    main()
}
