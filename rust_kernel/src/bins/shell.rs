use pc_keyboard::KeyCode;
use crate::driver::display::cursor::CURSOR;
use crate::print_serial;

pub struct Shell {}

impl Shell {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn on_key(&self, key: KeyCode){
        use core::fmt::Write;

        write!(CURSOR.lock(), "{:?}", key).unwrap();
        print_serial!( "{:?}", key);
    }

    pub fn on_char(&self, char:char){
        use core::fmt::Write;

        write!(CURSOR.lock(), "{}", char).unwrap();
        print_serial!( "{}", char);
    }
}