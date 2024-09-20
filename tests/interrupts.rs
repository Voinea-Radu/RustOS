#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    test_main();

    loop {}
}

#[cfg(not(test))]
#[allow(dead_code)]
fn test_main() {
    // This is here just for RustRover to not complain about it not existing.
    // The function is generated at compile time by the rust compiler for running tests.
}

#[test_case]
fn test_breakpoint() {
    // Testing the int3 (breakpoint) exception handling.
    println!("Raising a breakpoint interrupt (int3)");
    x86_64::instructions::interrupts::int3();
}
