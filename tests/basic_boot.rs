#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::hlt_loop;
use rust_os::test::tester::test_fail_with_error;

//noinspection RsUnresolvedPath
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_fail_with_error(info);
}

#[test_case]
fn simple_test() {
    // This is here just to make sure integration tests work
    assert_eq!(1, 1);
}
