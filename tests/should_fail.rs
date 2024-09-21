#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::hlt_loop;
use rust_os::test::tester::{all_tests_pass, test_pass};

//noinspection RsUnresolvedPath
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    test_main();

    hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_pass();
    all_tests_pass()
}

#[test_case]
fn failed_assertion() {
    // This is here just to make sure integration tests work
    assert_eq!(1, 0);
}
