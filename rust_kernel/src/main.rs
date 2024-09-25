#![no_std]
#![no_main]
extern crate alloc;

use alloc::boxed::Box;
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use bootloader_api::{entry_point, BootInfo};
use rust_kernel::driver::display::frame_buffer::{Color, FrameBufferWriter};
use rust_kernel::driver::display::image::PPMFormat;
use rust_kernel::utils::color::{AnsiColor, AnsiColorType};
use rust_kernel::{hlt_loop, println_serial, println_serial_color, CONFIG};

pub static FONT: &[u8] = include_bytes!("../assets/font/noto_sans_mono.ppm");
pub static TROLL1: &[u8] = include_bytes!("../assets/troll1.ppm");
pub static TROLL2: &[u8] = include_bytes!("../assets/troll2.ppm");

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let physical_memory_offset = boot_info.physical_memory_offset.take().expect("Failed to find physical memory offset");
    let memory_regions = &boot_info.memory_regions;

    rust_kernel::init(physical_memory_offset, memory_regions);

    let data = Box::new(1);
    println_serial!("{}", *data);

    println_serial_color!("hello" => AnsiColor::new_simple(AnsiColorType::Red));

    println_serial!("{}", FONT.len());

    //for (index, font_byte) in FONT.iter().enumerate(){
    //    println_serial!("{index} {}", *font_byte)
    //}

    //print_serial!("{}", boot_info.framebuffer.take().is_none());

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

            println_serial!("W: {}\nH: {}", screen_width, screen_height);

            let mut buffer_writer: FrameBufferWriter = FrameBufferWriter::new(
                frame_buffer.into_buffer(),
                pixel_format,
                bytes_per_pixel,
                screen_width,
                screen_height,
            );

            buffer_writer.draw_rectangle(0, 0, 500, 500, Color::new(0, 0, 0));

            println_serial!("{} {} {} {}", FONT[0], FONT[1], FONT[2], FONT[3]);
            println_serial!("{} {} {}", FONT[12], FONT[13], FONT[14]);
            println_serial!("{} {} {}", FONT[15], FONT[16], FONT[17]);

            let troll_image_1: PPMFormat = PPMFormat::new(TROLL1);

            for y in 0..troll_image_1.height() {
                for x in 0..troll_image_1.width() {
                    buffer_writer.draw_pixel_raw
                    (x, y,
                     troll_image_1.data[y * troll_image_1.width() * 3 + x * 3],
                     troll_image_1.data[y * troll_image_1.width() * 3 + x * 3 + 1],
                     troll_image_1.data[y * troll_image_1.width() * 3 + x * 3 + 2],
                    )
                }
            }

            let troll_image_2_offset_x = troll_image_1.width() + 10;
            let troll_image_2: PPMFormat = PPMFormat::new(TROLL2);

            for y in 0..troll_image_2.height() {
                for x in 0..troll_image_2.width() {
                    buffer_writer.draw_pixel_raw
                    (x + troll_image_2_offset_x, y,
                     troll_image_2.data[y * troll_image_2.width() * 3 + x * 3],
                     troll_image_2.data[y * troll_image_2.width() * 3 + x * 3 + 1],
                     troll_image_2.data[y * troll_image_2.width() * 3 + x * 3 + 2],
                    )
                }
            }
        }
    }

    //exit_qemu(Success);
    hlt_loop()
}

