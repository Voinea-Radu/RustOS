#![no_std]
#![no_main]

use crate::memory::{frame_allocator, heap_allocator};
use crate::utils::color::{AnsiColor, AnsiColorType};
use core::panic::PanicInfo;
use bootloader_api::config::Mapping;
use bootloader_api::info::MemoryRegions;
use x86_64::VirtAddr;
use crate::memory::frame_allocator::BootInfoFrameAllocator;

pub const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    // config.kernel_stack_size = 100 * 1024; // 100 KiB
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

pub mod driver {
    pub mod display {
        pub mod frame_buffer;
        pub mod image;
    }
    pub mod logger;
    pub mod qemu;
    pub mod serial;
}
pub mod memory {
    pub mod allocator {
        pub mod fixed_size_block;
    }
    pub mod frame_allocator;
    pub mod heap_allocator;
}
pub mod utils {
    pub mod color;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_serial_color!(
        "Panicked at {}: {}",
        info.location().unwrap(),
        info.message()
        => AnsiColor::new_simple(AnsiColorType::LightRed)
    );

    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init(physical_memory_offset:u64, memory_map: &'static MemoryRegions) {
    let physical_memory_offset = VirtAddr::new(physical_memory_offset);
    let mut mapper = frame_allocator::init(physical_memory_offset);
    let mut frame_allocator = BootInfoFrameAllocator::new(memory_map);

    heap_allocator::init(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");
}