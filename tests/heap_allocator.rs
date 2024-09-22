#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::kernel::memory::heap_allocator::HEAP_SIZE;
use rust_os::test::tester::test_fail_with_error;
use rust_os::{hlt_loop, init};

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
fn simple_allocation() {
    let heap_value_1 = Box::new(10);
    let heap_value_2 = Box::new("simple_allocation");

    assert_eq!(*heap_value_1, 10);
    assert_eq!(*heap_value_2, "simple_allocation");
}

#[test_case]
fn large_allocation() {
    let n = 10000;

    let mut vector: Vec<u64> = Vec::new();

    for i in 0..n {
        vector.push(i);
    }

    assert_eq!(vector.iter().sum::<u64>(), (n - 1) * n / 2)
}

#[test_case]
fn maximum_allocation() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn maximum_allocation_with_long_life() {
    let long_life = Box::new(1);

    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }

    assert_eq!(*long_life, 1);
}

