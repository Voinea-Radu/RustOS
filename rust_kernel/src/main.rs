#![no_std]
#![no_main]
extern crate alloc;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use bootloader_api::{entry_point, BootInfo};
use rust_kernel::driver::display::frame_buffer::{FrameBufferWriter, FRAME_BUFFER_WRITER};
use rust_kernel::driver::display::image::PPMFormat;
use rust_kernel::{hlt_loop, println_serial, CONFIG};

pub static FONT: &[u8] = include_bytes!("../assets/font/noto_sans_mono.ppm");
pub static TROLL1: &[u8] = include_bytes!("../assets/troll1.ppm");
pub static TROLL2: &[u8] = include_bytes!("../assets/troll2.ppm");

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    rust_kernel::init(boot_info);

    let troll_image_1: PPMFormat = PPMFormat::new(TROLL1);
    troll_image_1.render(0, 0);

    let troll_image_2: PPMFormat = PPMFormat::new(TROLL2);
    troll_image_2.render(troll_image_1.width() + 10, 0);

    let font_image: PPMFormat = PPMFormat::new(FONT);
    font_image.render(0, 0);

    //exit_qemu(Success);
    hlt_loop()
}

