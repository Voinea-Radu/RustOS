#![no_std]
#![no_main]
extern crate alloc;

use bootloader_api::{entry_point, BootInfo};
use rust_kernel::driver::display::font::Font;
use rust_kernel::driver::display::image::PPMFormat;
use rust_kernel::{hlt_loop, CONFIG};

pub static FONT: &[u8] = include_bytes!("../assets/fonts/noto_sans_mono.ppm");
pub static TROLL1: &[u8] = include_bytes!("../assets/images/troll1.ppm");
pub static TROLL2: &[u8] = include_bytes!("../assets/images/troll2.ppm");

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    rust_kernel::init(boot_info);

    let troll_image_1: PPMFormat = PPMFormat::new(TROLL1);
    troll_image_1.render(0, 0);

    let troll_image_2: PPMFormat = PPMFormat::new(TROLL2);
    troll_image_2.render(troll_image_1.width() + 10, 0);

    let font_image: PPMFormat = PPMFormat::new(FONT);
    font_image.render(0, 0);

    let font: Font = Font::new(font_image);
    font.render(50, 50, 'A');
    font.render(50 + 15 * 1, 50, 'b');
    font.render(50 + 15 * 2, 50, '!');
    font.render(50 + 15 * 3, 50, ',');

    troll_image_2.render_box(200, 200, 200, 200, 200, 200);

    //exit_qemu(Success);
    hlt_loop()
}

