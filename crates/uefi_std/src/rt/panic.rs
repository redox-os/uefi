// SPDX-License-Identifier: MIT

#[panic_handler]
pub fn panic(pi: &core::panic::PanicInfo) -> ! {
    print!("SETUP PANIC: {}", pi);

    loop {}
}
