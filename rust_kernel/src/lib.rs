#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

use crate::driver::display::cursor::{Cursor, CURSOR};
use crate::driver::display::font::Font;
use crate::driver::display::frame_buffer::{FrameBuffer, FRAME_BUFFER};
use crate::driver::display::image::PPMFormat;
use crate::driver::logger::Logger;
use crate::memory::frame_allocator::BootInfoFrameAllocator;
use crate::memory::{frame_allocator, heap_allocator};
use crate::utils::color::{AnsiColor, AnsiColorType};
use bootloader_api::config::Mapping;
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::VirtAddr;

#[cfg(feature = "uefi")]
use crate::driver::interrupts::controller::apic;
#[cfg(feature = "bios")]
use crate::driver::interrupts::controller::pic;

pub static FONT_DATA: &[u8] = include_bytes!("../assets/fonts/noto_sans_mono.ppm");
pub static TROLL1_DATA: &[u8] = include_bytes!("../assets/images/troll1.ppm");
pub static TROLL2_DATA: &[u8] = include_bytes!("../assets/images/troll2.ppm");

pub const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    // config.kernel_stack_size = 100 * 1024; // 100 KiB
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

pub mod bins {
    pub mod shell;
}
pub mod cpu {
    pub mod gdt;
}
pub mod driver {
    pub mod display {
        pub mod cursor;
        pub mod font;
        pub mod frame_buffer;
        pub mod image;
    }
    pub mod interrupts {
        pub mod controller {
            pub mod apic;
            pub mod pic;
        }
        pub mod interrupts_handlers;
    }
    pub mod keyboard;
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
    pub mod locked;
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
    // Memory
    {
        let physical_memory_offset = VirtAddr::new(
            boot_info
                .physical_memory_offset
                .take()
                .expect("Failed to find physical memory offset"),
        );
        let mut mapper: OffsetPageTable<'static> = frame_allocator::init(physical_memory_offset);
        let mut frame_allocator = BootInfoFrameAllocator::new(&boot_info.memory_regions);

        heap_allocator::init(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

        let rsdp: Option<u64> = boot_info.rsdp_addr.take();

        #[cfg(feature = "bios")]
        pic::init();

        #[cfg(feature = "uefi")]
        unsafe {
            apic::init(rsdp.expect("Failed to get RSDP address") as usize, physical_memory_offset, &mut mapper, &mut frame_allocator);
        }
    }

    // VGA
    {
        match boot_info.framebuffer.take() {
            None => {
                println_serial!("framebuffer not found in boot_info");
            }
            Some(frame_buffer) => {
                let frame_buffer_info: FrameBufferInfo = frame_buffer.info();
                let screen_width: usize = frame_buffer_info.width;
                let screen_height: usize = frame_buffer_info.height;
                let bytes_per_pixel: usize = frame_buffer_info.bytes_per_pixel;
                let pixel_format: PixelFormat = frame_buffer_info.pixel_format;

                FRAME_BUFFER.lock().update(FrameBuffer::new(
                    frame_buffer.into_buffer(),
                    pixel_format,
                    bytes_per_pixel,
                    screen_width,
                    screen_height,
                ));
            }
        }

        FRAME_BUFFER.lock().clear_screen();
        CURSOR.lock().update(Cursor::new(Font::new(PPMFormat::new(FONT_DATA))));
        Logger::init();
    }
}
