#![no_std]
#![no_main]

mod vga_buffer;
mod statics;

use core::panic::PanicInfo;
use crate::statics::TROLL_MESSAGE;
use crate::vga_buffer::{Color, ColorCode, Writer};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer: Writer = Writer::new(ColorCode::new(Color::Red, Color::Black));
    writer.write_str(TROLL_MESSAGE);

    loop {}
}