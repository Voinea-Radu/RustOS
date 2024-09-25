#![no_std]
#![no_main]
extern crate alloc;

use crate::memory::{frame_allocator, heap_allocator};
use crate::utils::color::{AnsiColor, AnsiColorType};
use core::panic::PanicInfo;
use bootloader_api::BootInfo;
use bootloader_api::config::Mapping;
use bootloader_api::info::{FrameBufferInfo, MemoryRegions, PixelFormat};
use x86_64::VirtAddr;
use crate::driver::display::frame_buffer::{FrameBufferWriter, FRAME_BUFFER_WRITER};
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

pub fn init(boot_info: &'static mut BootInfo) {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset.take().expect("Failed to find physical memory offset"));
    let mut mapper = frame_allocator::init(physical_memory_offset);
    let mut frame_allocator = BootInfoFrameAllocator::new(&boot_info.memory_regions);

    heap_allocator::init(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    match boot_info.framebuffer.take() {
        None => {
            println_serial!("framebuffer not found");
        }
        Some(frame_buffer) => {
            let frame_buffer_info: FrameBufferInfo = frame_buffer.info();
            let screen_width: usize = frame_buffer_info.width;
            let screen_height: usize = frame_buffer_info.height;
            let bytes_per_pixel: usize = frame_buffer_info.bytes_per_pixel;
            let pixel_format: PixelFormat = frame_buffer_info.pixel_format;

            let buffer_writer: FrameBufferWriter = FrameBufferWriter::new(
                frame_buffer.into_buffer(),
                pixel_format,
                bytes_per_pixel,
                screen_width,
                screen_height,
            );
            FRAME_BUFFER_WRITER.lock().update(buffer_writer);
        }
    }
}