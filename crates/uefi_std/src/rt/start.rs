use crate::prelude::*;

#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(
    handle: Handle,
    system_table: &'static mut SystemTable,
) -> Status {
    extern "C" {
        fn main() -> Status;
    }

    crate::HANDLE = handle;
    crate::SYSTEM_TABLE = system_table;

    uefi_alloc::init(::core::mem::transmute(system_table));

    main()
}
