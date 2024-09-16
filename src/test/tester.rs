use crate::driver::vga_driver::ColorCode::{Black, LightCyan, LightGreen, LightRed};
use crate::driver::vga_driver::{Color, WRITER};
use crate::utils::statics::WELCOME_MESSAGE;
use crate::{print, println};
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    WRITER.lock().color(Color::new(LightRed, Black));
    println!("Fail");
    println!("Error at: {}: {}\n", info.location().unwrap(), info.message());
    WRITER.lock().reset_color();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn() -> bool]) {
    WRITER.lock().write_str_color(WELCOME_MESSAGE, Color::new(LightCyan, Black));
    print!("\n\n");

    println!("Running {} tests", tests.len());
    for (index, test) in tests.iter().enumerate() {
        print!("Running test {}... ", index);
        test();
        WRITER.lock().write_str_color("OK\n", Color::new(LightGreen, Black));
    }
}

#[test_case]
fn trivial_assertion() -> bool {
    assert_eq!(1, 1);
    true
}

#[test_case]
fn trivial_assertion_2() -> bool {
    assert_eq!(1, 1);
    true
}

#[test_case]
fn trivial_assertion_3() -> bool {
    assert_eq!(1, 1);
    true
}