use core::fmt;
use crate::driver::vga_driver::{Color, WRITER};

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn _print_color(args: fmt::Arguments, color: Color) {
    use core::fmt::Write;
    let old_color = WRITER.lock().color();

    WRITER.lock().set_color(color);
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().set_color(old_color);
}

/**
print!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        crate::kernel::io::_print(format_args!($($arg)*));
    };
}

/**
println!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => {
        crate::print!("{}\n", format_args!($($arg)*));
    };
}

/**
    print_color!("Example {} {} {}", 1, 2, 3 => Color::new(LightRed, Black));
**/
#[macro_export]
macro_rules! print_color {
    ($fmt:expr => $color:expr) => {
        crate::kernel::io::_print_color(format_args!($fmt), $color);
    };
    ($fmt:expr, $($arg:expr),* => $color:expr) => {
        crate::kernel::io::_print_color(format_args!($fmt, $($arg),*), $color);
    };

}

/**
println_color!("Example {} {} {}", 1, 2, 3 => Color::new(LightRed, Black));
**/
#[macro_export]
macro_rules! println_color {
    ($fmt:expr => $color:expr) => {
        crate::print_color!(concat!($fmt, "\n") => $color);
    };
    ($fmt:expr, $($arg:expr),* => $color:expr) => {
        crate::print_color!(concat!($fmt, "\n"), $($arg),* => $color);
    };
}


