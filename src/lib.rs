#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester::test_runner)]

pub mod utils {
    pub mod statics;
    pub mod color;
}
pub mod driver {
    pub mod vga;
    pub mod qemu;
    pub mod serial;
}
pub mod kernel {
    pub mod panic;
}
pub mod test {
    pub mod tester;
}


#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(not(test))]
#[allow(dead_code)]
fn test_main() {
    // This is here just for RustRover to not complain about it not existing.
    // The function is generated at compile time by the rust compiler for running tests.
}
