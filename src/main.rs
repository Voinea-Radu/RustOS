#![no_std]
#![no_main]

mod vga_buffer;
mod statics;

use core::panic::PanicInfo;
use crate::statics::TROLL_MESSAGE;
use crate::vga_buffer::{WRITER};
use core::fmt::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}: {}", info.location().unwrap(), info.message());
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(WRITER.lock(), "{}", TROLL_MESSAGE).unwrap();
    write!(WRITER.lock(), "Hello {}", "Pudel Vesel!\n").unwrap();
    for i in 0..50{
        write!(WRITER.lock(), "Line {i}\n").unwrap();
    }

    print!("Hello ");
    println!("Pudel Prost");
    panic!("Pudel si Daria au iesit la cafea");

    loop {}
}