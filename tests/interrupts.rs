#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::test::tester::test_fail_with_error;
use rust_os::{hlt_loop, init, println};

entry_point!(test_kernel_main);

//noinspection RsUnresolvedPath
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_fail_with_error(info);
}

#[test_case]
fn test_breakpoint() {
    // Testing the int3 (breakpoint) exception handling.
    println!("Raising a breakpoint interrupt (int3)");
    x86_64::instructions::interrupts::int3();
}
