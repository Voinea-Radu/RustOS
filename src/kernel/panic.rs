#![allow(dead_code)]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use crate::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic at {}: {}", info.location().unwrap(), info.message());
    loop {}
}