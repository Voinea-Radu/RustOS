use crate::println_serial;
use core::fmt::Write;
use log::{set_logger, set_max_level, LevelFilter, Metadata, Record};
use crate::driver::display::cursor::CURSOR;

pub static LOGGER: Logger = Logger::new();

pub struct Logger {}

impl Logger {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn init() {
        set_logger(&LOGGER).expect("Failed to init the logger");
        set_max_level(LevelFilter::Trace)
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        writeln!(CURSOR.lock(), "[{:5}]: {}", record.level(), record.args()).unwrap();
        println_serial!("[{:5}]: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
