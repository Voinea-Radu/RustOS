#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use rust_kernel::utils::color::{AnsiColor, Color};
use rust_kernel::{println_serial_color, CONFIG};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    println_serial_color!("hello" => Color::new_simple(AnsiColor::Red));

    //exit_qemu(Success);
    loop {}
}

