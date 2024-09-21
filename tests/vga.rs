#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::fmt::Write;
use core::panic::PanicInfo;
use rust_os::driver::vga::WRITER;
use rust_os::test::tester::test_fail_with_error;
use rust_os::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    test_main();

    hlt_loop()
}

#[cfg(not(test))]
#[allow(dead_code)]
fn test_main() {
    // This is here just for RustRover to not complain about it not existing.
    // The function is generated at compile time by the rust compiler for running tests.
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_fail_with_error(info);
}

#[test_case]
fn test_println_simple() {
    // Testing if println does panic
    println!("test_println_simple");
}

#[test_case]
fn test_println_multiple() {
    // Testing if println does panic if the pane needs to scroll down
    for _ in 0..100 {
        println!("test_println_multiple");
    }
}

#[test_case]
fn test_println_buffer() {
    // Testing if println does add the contents correctly into the vga buffer
    let string = "test_println_buffer";
    let mut success = true;

    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", string).expect("Writing to VGA failed");

        let row = writer.row_position - 1; // last row as we did a \n

        for (index, char) in string.chars().enumerate() {
            let buffer_char: char = writer.buffer.chars[row][index].read().character as char;

            if char != buffer_char {
                success = false;
                break;
            }
        }
    });

    if !success {
        panic!("Buffer does not contain the correct data");
    }
}
