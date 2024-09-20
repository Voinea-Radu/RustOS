use crate::utils::color::Color;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

pub const SERIAL_PORT_1: u16 = 0x3F8; // First serial port address

lazy_static! {
    pub static ref SERIAL_1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL_PORT_1) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn _print(args: fmt::Arguments, color: Option<Color>) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        match color {
            None => {
                SERIAL_1
                    .lock()
                    .write_fmt(format_args!(
                        "{}{}{}",
                        Color::reset_color().get_ansi_color(),
                        args,
                        Color::reset_color().get_ansi_color()
                    ))
                    .expect("Printing to serial port failed");
            }
            Some(color) => {
                SERIAL_1
                    .lock()
                    .write_fmt(format_args!(
                        "{}{}{}",
                        color.get_ansi_color(),
                        args,
                        Color::reset_color().get_ansi_color()
                    ))
                    .expect("Printing to serial port failed");
            }
        }
    });
}

/**
print!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => {
        $crate::driver::serial::_print(format_args!($($arg)*), None);
    };
}

/**
println!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! println_serial {
    () => {
        $crate::print!("\n");
    };
    ($($arg:expr),*) => {
        $crate::print_serial!("{}\n", format_args!($($arg),*));
    };
}

/**
print_color!("Example {} {} {}", 1, 2, 3 => Color::new(LightRed, Black));
**/
#[macro_export]
macro_rules! print_serial_color {
    ($($arg:expr),* => $color:expr) => {
        $crate::driver::serial::_print(format_args!($($arg),*), Some($color));
    };
}

/**
println_color!("Example {} {} {}", 1, 2, 3 => Color::new(LightRed, Black));
**/
#[macro_export]
macro_rules! println_serial_color {
    () => {
        $crate::print!("\n");
    };
    ($($arg:expr),* => $color:expr) => {
        $crate::print_serial_color!("{}\n", format_args!($($arg),*) => $color);
    };
}
