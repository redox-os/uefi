// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.

use core::prelude::rust_2024::alloc_error_handler;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn rust_begin_panic(pi: &::core::panic::PanicInfo) -> ! {
    print!("SETUP PANIC: {}", pi);

    loop {}
}

#[alloc_error_handler]
#[no_mangle]
pub fn rust_oom(layout: ::core::alloc::Layout) -> ! {
    println!(
        "SETUP OOM: {} bytes aligned to {} bytes\n",
        layout.size(),
        layout.align()
    );

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}
