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
fn panic(info: &PanicInfo) -> ! {
    println_color!("Fail" => Color::new_simple(LightRed));
    println_serial_color!("Fail" => Color::new_simple(LightRed));
    println_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new_simple(LightRed));
    println_serial_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new_simple(LightRed));

    // println_color!("One or more test(s) failed. This window will close automatically in 10s.\n\
    // You should be able to find the test logs in the console that you have started qemu with.\n" => Color::new_simple(Yellow));
    // println_serial_color!("One or more test(s) failed. This window will close automatically in 10s.\n" => Color::new_simple(Yellow));

    println_color!("One or more test(s) failed.\n\
    You should be able to find the test logs in the console that you have started qemu with." => Color::new_simple(Yellow));
    println_serial_color!("One or more test(s) failed." => Color::new_simple(Yellow));

    // TODO Add a better way
    // Sleep for 10s (only for 5 GHz CPU)
    // for _ in 0..50_000_000 {}

    exit_qemu(QemuExitCode::Fail)
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println_color!("{}", WELCOME_MESSAGE => Color::new_simple(LightCyan));

    println!("\nRunning {} tests", tests.len());
    println_serial!("\nRunning {} tests", tests.len());
    for test in tests {
        test.run();
    }

    println_color!("All tests finished successfully." => Color::new_simple(Green));
    println_serial_color!("All tests finished successfully." => Color::new_simple(Green));

    // println_color!("All tests finished successfully. This window will close automatically in 10s\n" => Color::new_simple(Yellow));
    // println_serial_color!("All tests finished successfully. The qemu window will close automatically in 10s\n" => Color::new_simple(Yellow));

    // TODO Add a better way
    // Sleep for 10s (only for 5 GHz CPU)
    // for _ in 0..50_000_000 {}

    exit_qemu(QemuExitCode::Success)
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
        println_color!("OK" => Color::new_simple(LightGreen));
        println_serial_color!("OK" => Color::new_simple(LightGreen));
    }
}

#[test_case]
fn simple_assertion() {
    // This is here just to make sure tests implementation works
    assert_eq!(1, 1);
}
