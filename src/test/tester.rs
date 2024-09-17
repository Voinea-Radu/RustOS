#![allow(unused_imports)]
#![allow(dead_code)]

use crate::driver::qemu::{exit_qemu, QemuExitCode};
use crate::utils::statics::WELCOME_MESSAGE;
use crate::{print, println, println_color, print_serial, println_serial, println_serial_color};
use core::panic::PanicInfo;
use crate::utils::color::Color;
use crate::utils::color::ColorCode::{LightCyan, LightGreen, LightRed, Yellow};

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_color!("Fail" => Color::new_simple(LightRed));
    println_serial_color!("Fail" => Color::new_simple(LightRed));
    println_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new_simple(LightRed));
    println_serial_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new_simple(LightRed));

    println_color!("One or more test(s) failed. This window will close automatically in 10s.\n\
    You should be able to find the test logs in the console that you have started qemu with" => Color::new_simple(Yellow));
    println_serial_color!("One or more test(s) failed. This window will close automatically in 10s.\n\
    You should be able to find the test logs in the console that you have started qemu with" => Color::new_simple(Yellow));

    // TODO Add a better way
    // Sleep for 10s (only for 5 GHz CPU)
    for _ in 0..50_000_000{
    }

    exit_qemu(QemuExitCode::Fail)
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn() -> bool]) {
    println_color!("{}", WELCOME_MESSAGE => Color::new_simple(LightCyan));

    println!("Running {} tests", tests.len());
    println_serial!("Running {} tests", tests.len());
    for (index, test) in tests.iter().enumerate() {
        print!("Running test {}... ", index);
        print_serial!("Running test {}... ", index);
        test();
        println_color!("OK" => Color::new_simple(LightGreen));
        println_serial_color!("OK" => Color::new_simple(LightGreen));
    }

    println_color!("All tests finished successfully. This window will close automatically in 10s" => Color::new_simple(Yellow));
    println_serial_color!("All tests finished successfully. This window will close automatically in 10s" => Color::new_simple(Yellow));

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
