use log::{Metadata, Record};

pub struct Logger{}

impl log::Log for Logger{
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
    }

    fn flush(&self) {
    }
}