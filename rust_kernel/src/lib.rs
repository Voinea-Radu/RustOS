#![no_std]
#![no_main]

use crate::utils::color::{AnsiColorType, AnsiColor};
use core::panic::PanicInfo;

pub const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

pub mod driver {
    pub mod display {
        pub mod frame_buffer;
    }
    pub mod logger;
    pub mod qemu;
    pub mod serial;
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