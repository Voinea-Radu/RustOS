#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester::test_runner)]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader::entry_point;

use crate::kernel::memory::frame_allocator::BootInfoFrameAllocator;
use bootloader::BootInfo;
use x86_64::VirtAddr;

extern crate alloc;

pub mod binaries {
    pub mod builtin_shell_commands;
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
    pub mod memory {
        pub mod frame_allocator;
        pub mod heap_allocator;
    }
    pub mod global_descriptor_table;
    pub mod interrupts;
}
pub mod test {
    pub mod tester;
}

#[cfg(test)]
entry_point!(test_kernel_main);

//noinspection RsUnresolvedPath
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    #[cfg(test)]
    test_main();
    hlt_loop();
}

pub fn init(boot_info: &'static BootInfo) {
    kernel::global_descriptor_table::init();
    kernel::interrupts::IDT.load();
    unsafe {
        kernel::interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = kernel::memory::frame_allocator::init(physical_memory_offset);
    let mut frame_allocator = BootInfoFrameAllocator::new(&boot_info.memory_map);

    kernel::memory::heap_allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
