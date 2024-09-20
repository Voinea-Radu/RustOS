#![allow(unused_imports)]
#![allow(dead_code)]

use crate::driver::qemu::{exit_qemu, QemuExitCode};
use crate::utils::color::Color;
use crate::utils::color::ColorCode::{Green, LightCyan, LightGreen, LightRed, Yellow};
use crate::utils::statics::WELCOME_MESSAGE;
use crate::{print, print_serial, println, println_color, println_serial, println_serial_color};
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    test_fail_with_error(info)
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println_color!("{}", WELCOME_MESSAGE => Color::new_simple(LightCyan));

    pre_tests_run(tests.len());
    for test in tests {
        run_test(*test);
        test_pass();
    }

    all_tests_pass()
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("Running test {}... ", core::any::type_name::<T>());
        print_serial!("Running test {}... ", core::any::type_name::<T>());
        self();
    }
}

pub fn run_test(test: &dyn Testable) {
    test.run();
}

pub fn pre_tests_run(len: usize) {
    println!("\nRunning {} tests", len);
    println_serial!("\nRunning {} tests", len);
}

pub fn test_pass() {
    println_color!("OK" => Color::new_simple(LightGreen));
    println_serial_color!("OK" => Color::new_simple(LightGreen));
}

pub fn all_tests_pass() -> ! {
    println_color!("All tests finished successfully." => Color::new_simple(Green));
    println_serial_color!("All tests finished successfully." => Color::new_simple(Green));

    exit_qemu(QemuExitCode::Success)
}

pub fn test_fail() {
    println_color!("Fail" => Color::new_simple(LightRed));
    println_serial_color!("Fail" => Color::new_simple(LightRed));

    println_color!("One or more test(s) failed.\n\
    You should be able to find the test logs in the console that you have started qemu with." => Color::new_simple(Yellow));
    println_serial_color!("One or more test(s) failed." => Color::new_simple(Yellow));
}

pub fn test_fail_with_error(info: &PanicInfo) -> ! {
    println_color!("Fail" => Color::new_simple(LightRed));
    println_serial_color!("Fail" => Color::new_simple(LightRed));

    println_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new_simple(LightRed));
    println_serial_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new_simple(LightRed));

    println_color!("One or more test(s) failed.\n\
    You should be able to find the test logs in the console that you have started qemu with." => Color::new_simple(Yellow));
    println_serial_color!("One or more test(s) failed." => Color::new_simple(Yellow));

    exit_qemu(QemuExitCode::Fail)
}

#[test_case]
fn simple_assertion() {
    // This is here just to make sure tests implementation works
    assert_eq!(1, 1);
}