#![no_std] // no use for std library
#![no_main] // and no std entry_point, so no runtime to call main function

use core::panic::PanicInfo;

static HELLO: &[u8] = b"heeelo world";

#[no_mangle] // don't change name on compile time
pub extern "C" fn _start() -> ! {
    // custom entry point for the linker
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler] // our own panic handler, because no std
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
