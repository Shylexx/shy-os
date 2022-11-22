#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// ENTRY POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!, Here are some numbers: {}, {}", 420, 69 / 3);
    panic!("Panic!");
    loop {}
}
