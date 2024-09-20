#![allow(dead_code)]
#![allow(unused_imports)]

use crate::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic at {}: {}", info.location().unwrap(), info.message());
    loop {}
}
