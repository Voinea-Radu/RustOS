#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester::test_runner)]
#![feature(abi_x86_interrupt)]

pub mod binaries {
    pub mod shell;
}
pub mod utils {
    pub mod color;
    pub mod statics;
}
pub mod driver {
    pub mod keyboard;
    pub mod qemu;
    pub mod serial;
    pub mod vga;
}
pub mod kernel {
    pub mod global_descriptor_table;
    pub mod interrupts;
}
pub mod test {
    pub mod tester;
}

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    init();

    #[cfg(test)]
    test_main();

    hlt_loop()
}

#[cfg(not(test))]
#[allow(dead_code)]
fn test_main() {
    // This is here just for RustRover to not complain about it not existing.
    // The function is generated at compile time by the rust compiler for running tests.
}

pub fn init() {
    kernel::global_descriptor_table::init();
    kernel::interrupts::IDT.load();
    unsafe {
        kernel::interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
