#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]

use rust_os::{print, println, println_color};
use rust_os::utils::color::Color;
use rust_os::utils::color::ColorCode::LightCyan;
use rust_os::utils::statics::TROLL_MESSAGE;

//noinspection RsUnresolvedPath
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    main();

    loop {}
}

pub fn main() {
    println_color!("{}", TROLL_MESSAGE => Color::new_simple(LightCyan));
    println!("Hello {}", "Pudel Vesel!\n");

    print!("Hello ");
    println!("Pudel Prost!");
    panic!("Pudelul si Daria au iesit la cafea!");
}


#[cfg(not(test))]
#[allow(dead_code)]
fn test_main() {
    // This is here just for RustRover to not complain about it not existing.
    // The function is generated at compile time by the rust compiler for running tests.
}


