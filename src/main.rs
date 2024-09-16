#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester::test_runner)]

mod utils {
    pub mod statics;
}
mod driver {
    pub mod vga_driver;
    pub mod qemu;
    pub mod serial;
}
mod kernel {
    pub mod panic;
    pub mod io;
}
mod test {
    pub mod tester;
}

use crate::driver::qemu::{exit_qemu, QemuExitCode};
use crate::driver::vga_driver::Color;
use crate::driver::vga_driver::ColorCode::{Black, LightCyan};
use crate::utils::statics::TROLL_MESSAGE;

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
    println_color!("{}", TROLL_MESSAGE => Color::new(LightCyan, Black));
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


