#![allow(unused_imports)]
#![allow(dead_code)]

use crate::driver::vga_driver::ColorCode::{Black, LightCyan, LightGreen, LightRed};
use crate::driver::vga_driver::{Color, WRITER};
use crate::utils::statics::WELCOME_MESSAGE;
use crate::{print, println, println_color};
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_color!("Fail" => Color::new(LightRed, Black));
    println_color!("Error at: {}: {}", info.location().unwrap(), info.message() => Color::new(LightRed, Black));
    WRITER.lock().reset_color();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn() -> bool]) {
    println_color!("{}", WELCOME_MESSAGE => Color::new(LightCyan, Black));

    println!("Running {} tests", tests.len());
    for (index, test) in tests.iter().enumerate() {
        print!("Running test {}... ", index);
        test();
        println_color!("OK" => Color::new(LightGreen, Black));
    }

    loop {} // DO NOT DELETE THIS. For some reason this needs to be here even tho there is another one in panic and in main
}

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
    assert_eq!(1, 2);
    true
}