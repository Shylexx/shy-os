#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use shy_os::println;

// ENTRY POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    shy_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    shy_os::hlt_loop();
}

// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    shy_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    shy_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
