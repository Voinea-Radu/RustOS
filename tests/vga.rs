#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os::driver::vga::WRITER;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    test_main();

    loop {}
}

#[cfg(not(test))]
#[allow(dead_code)]
fn test_main() {
    // This is here just for RustRover to not complain about it not existing.
    // The function is generated at compile time by the rust compiler for running tests.
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
    println!();
    let string = "test_println_buffer";
    println!("{}", string);
    let row = WRITER.lock().row_position - 1; // last row as we did a \n
    for (index, char) in string.chars().enumerate() {
        let buffer_char = WRITER.lock().buffer.chars[row][index].read().character;
        assert_eq!(char as u8, buffer_char);
    }
}
