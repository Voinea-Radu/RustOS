#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester::test_runner)]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};

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
    pub mod allocator;
    pub mod global_descriptor_table;
    pub mod interrupts;
    pub mod memory;
}
pub mod test {
    pub mod tester;
}

#[cfg(test)]
entry_point!(test_kernel_main);

//noinspection RsUnresolvedPath
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();

    #[cfg(test)]
    test_main();

    hlt_loop()
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
