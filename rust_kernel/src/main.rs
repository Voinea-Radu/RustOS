#![no_std]
#![no_main]
extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use log::info;
use rust_kernel::driver::display::font::{Font, CURSOR};
use rust_kernel::driver::display::image::PPMFormat;
use rust_kernel::{hlt_loop, CONFIG};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    rust_kernel::init(boot_info);

    info!("Hello");
    info!("Line");
    info!("Line");
    info!("Line");
    info!("Line");
    info!("Line");
    info!("Line");
    info!("Line");

    //exit_qemu(Success);
    hlt_loop()
}
