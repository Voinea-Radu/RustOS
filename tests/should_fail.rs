#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::test::tester::{all_tests_pass, test_pass};
use rust_os::{hlt_loop, init};

entry_point!(test_kernel_main);

//noinspection RsUnresolvedPath
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
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
