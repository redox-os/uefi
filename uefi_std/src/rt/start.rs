use uefi::Handle;
use uefi::status::Status;
use uefi::system::SystemTable;

#[no_mangle]
pub unsafe extern "win64" fn efi_main(handle: Handle, system_table: &'static mut SystemTable) -> Status {
    extern "C" {
        fn main() -> Status;
    }

    crate::HANDLE = handle;
    crate::SYSTEM_TABLE = system_table;

    uefi_alloc::init(::core::mem::transmute(system_table));

    main()
}
