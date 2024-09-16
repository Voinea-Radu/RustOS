use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use crate::utils::statics::SERIAL_PORT_1;

lazy_static! {
    pub static ref SERIAL_1: Mutex<SerialPort> = {
        let mut serial_port = unsafe{
            SerialPort::new(SERIAL_PORT_1)
        };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => {
        $crate::driver::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println_serial {
    () => ($crate::print_serial!("\n"));
    ($fmt:expr) => ($crate::print_serial!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_serial!(concat!($fmt, "\n"), $($arg)*));
}