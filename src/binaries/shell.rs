use crate::driver::keyboard::KeyboardListener;
use crate::{print, println};
use pc_keyboard::KeyCode;

pub struct Shell {
    internal_buffer: [char; 256],
    internal_buffer_index: usize,
}

impl KeyboardListener for Shell {
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Return => {
                self.handle_return();
                return;
            }
            _ => {}
        }
    }

    fn handle_char(&mut self, char: char) {
        if char == '\n' {
            self.handle_return();
            return;
        }

        if char as u8 == 0x8{
            self.handle_backspace();
            return;
        }

        self.internal_buffer[self.internal_buffer_index] = char;
        self.internal_buffer_index += 1;
        print!("{}", char)
    }
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            internal_buffer: [0 as char; 256],
            internal_buffer_index: 0,
        }
    }

    pub fn handle_backspace(&mut self) {
        self.internal_buffer_index -= 1;
    }

    pub fn handle_return(&mut self) {
        print!("\nExecuting: ");
        for index in 0..self.internal_buffer_index {
            let char = self.internal_buffer[index];
            print!("{}", char)
        }
        self.internal_buffer_index = 0;
        println!()
    }
}

