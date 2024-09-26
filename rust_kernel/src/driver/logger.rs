use log::{Metadata, Record};

pub struct Logger{
}

impl log::Log for Logger{
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, _record: &Record) {
    }

    fn flush(&self) {
    }
}