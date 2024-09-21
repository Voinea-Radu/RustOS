#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]

use core::panic::PanicInfo;
use rust_os::utils::color::Color;
use rust_os::utils::color::ColorCode::LightCyan;
use rust_os::utils::statics::TROLL_MESSAGE;
use rust_os::{hlt_loop, print, println, println_color, test};

pub mod kernel {
    pub mod panic;
}

//noinspection RsUnresolvedPath
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    main();

    hlt_loop()
}

pub fn main() {
    println_color!("{}", TROLL_MESSAGE => Color::new_simple(LightCyan));
    println!("Hello {}", "Pudel Vesel!\n");

    print!("Hello ");
    println!("Pudel Prost!");

    // panic!("Pudelul si Daria au iesit la cafea!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(
        "Panicked at {}: {}",
        info.location().unwrap(),
        info.message()
    );
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test::tester::test_fail_with_error(info);
}
