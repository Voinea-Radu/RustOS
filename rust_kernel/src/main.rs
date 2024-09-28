#![no_std]
#![no_main]
extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use log::{error, info};
use rust_kernel::driver::display::frame_buffer::{Color, FRAME_BUFFER};
use rust_kernel::{hlt_loop, CONFIG};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    rust_kernel::init(boot_info);

    for index in 1..=1000{
        info!("Line{}", index);
    }

    //exit_qemu(Success);
    hlt_loop()
}
