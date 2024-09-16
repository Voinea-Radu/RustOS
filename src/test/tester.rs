#![allow(unused_imports)]
#![allow(dead_code)]

use crate::driver::qemu::{exit_qemu, QemuExitCode};
use crate::driver::vga_driver::ColorCode::{Black, LightCyan, LightGreen, LightRed, Yellow};
use crate::driver::vga_driver::{Color, WRITER};
use crate::utils::statics::WELCOME_MESSAGE;
use crate::{print, println, println_color, print_serial, println_serial};
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_color!("Fail" => Color::new(LightRed, Black));
    println_serial!("Fail");
    println_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new(LightRed, Black));
    println_serial!("Error at: {}: {}", info.location().unwrap(), info.message());

    println_color!("One or more test(s) failed. This window will close automatically in 10s.\n\
    You should be able to find the test logs in the console that you have started qemu with" => Color::new(Yellow, Black));

    // TODO Add a better way
    // Sleep for 10s (only for 5 GHz CPU)
    for _ in 0..50_000_000{
    }

    exit_qemu(QemuExitCode::Fail)
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn() -> bool]) {
    println_color!("{}", WELCOME_MESSAGE => Color::new(LightCyan, Black));

    println!("Running {} tests", tests.len());
    println_serial!("Running {} tests", tests.len());
    for (index, test) in tests.iter().enumerate() {
        print!("Running test {}... ", index);
        print_serial!("Running test {}... ", index);
        test();
        println_color!("OK" => Color::new(LightGreen, Black));
        println_serial!("OK");
    }

    println_color!("All tests finished successfully. This window will close automatically in 10s" => Color::new(Yellow, Black));

    // TODO Add a better way
    // Sleep for 10s (only for 5 GHz CPU)
    for _ in 0..50_000_000{
    }

    exit_qemu(QemuExitCode::Success)
}

// TODO Remove these at some point
// =============================== EXAMPLE TESTS ===============================
#[test_case]
fn trivial_assertion() -> bool {
    assert_eq!(1, 1);
    true
}

#[test_case]
fn trivial_assertion_2() -> bool {
    assert_eq!(1, 1);
    true
}

#[test_case]
fn trivial_assertion_3() -> bool {
    // If you haven't figured out already this test is supposed to fail.
    // Do you even know Rust?
    assert_eq!(1, 2);
    true
}
// =============================== EXAMPLE TESTS ===============================
