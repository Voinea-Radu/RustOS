use core::fmt;
use crate::driver::vga_driver::WRITER;

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// TODO Maybe make the macro take in a color
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::kernel::io::_print(format_args!($($arg)*)));
}

// TODO Maybe make the macro take in a color
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}

