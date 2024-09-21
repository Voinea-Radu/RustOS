use crate::driver::keyboard::KeyboardListener;
use crate::print;
use pc_keyboard::KeyCode;

pub struct Shell {}

impl KeyboardListener for Shell {
    fn on_key(&self, _key: KeyCode) {}

    fn on_char(&self, char: char) {
        print!("{}", char)
    }
}

impl Shell {
    pub fn new() -> Shell {
        Shell {}
    }
}

