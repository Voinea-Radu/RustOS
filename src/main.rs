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
    rust_os::init();

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
    // panic!("Pudelul si Daria au iesit la cafea!");

    println!("Raising an interrupt (int3)");
    x86_64::instructions::interrupts::int3();
}
