#![no_std]
#![no_main]

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use bootloader_api::{entry_point, BootInfo};
use rust_kernel::driver::display::frame_buffer::{Color, FrameBufferWriter};
use rust_kernel::utils::color::{AnsiColor, AnsiColorType};
use rust_kernel::{hlt_loop, println_serial, println_serial_color, CONFIG};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println_serial_color!("hello" => AnsiColor::new_simple(AnsiColorType::Red));

    //print_serial!("{}", boot_info.framebuffer.take().is_none());

    match boot_info.framebuffer.take() {
        None => {
            println_serial!("framebuffer not found");
        }
        Some(frame_buffer) => {
            let frame_buffer_info: FrameBufferInfo = frame_buffer.info();
            let width: usize = frame_buffer_info.width;
            let height: usize = frame_buffer_info.height;
            let bytes_per_pixel: usize = frame_buffer_info.bytes_per_pixel;
            let pixel_format: PixelFormat = frame_buffer_info.pixel_format;

            println_serial!("W: {}\nH: {}", width, height);

            let mut buffer_writer: FrameBufferWriter = FrameBufferWriter::new(
                frame_buffer.into_buffer(),
                pixel_format,
                bytes_per_pixel,
                width,
                height,
            );

            buffer_writer.draw_rectangle(0, 0, 100, 100, Color::new(0,0,0))
        }
    }

    //exit_qemu(Success);
    hlt_loop()
}

