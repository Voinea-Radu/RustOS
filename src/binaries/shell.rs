use crate::binaries::builtin_shell_commands::{
    add_command_handler, clear_command_handler, daria_command_handler, nop_command_handler,
    pudel_command_handler,
};
use crate::driver::keyboard::KeyboardListener;
use crate::{print, println};
use alloc::boxed::Box;
use alloc::string::String;
use hashbrown::HashMap;
use pc_keyboard::KeyCode;

pub enum ShellError {
    InvalidArgumentsError,
}

pub struct Shell {
    internal_buffer: [char; 256],
    internal_buffer_index: usize,

    commands: HashMap<String, Box<dyn Fn(String) -> Result<(), ShellError> + Send + Sync>>,
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
        match char as u8 {
            b'\n' => {
                self.handle_return();
            }
            0x8 => {
                self.handle_backspace();
            }
            _ => {
                if self.internal_buffer_index == self.internal_buffer.len() {
                    return;
                }

                self.internal_buffer[self.internal_buffer_index] = char;
                self.internal_buffer_index += 1;
                print!("{}", char)
            }
        }
    }
}

impl Shell {
    pub fn new() -> Self {
        let mut shell: Shell = Shell {
            internal_buffer: [0 as char; 256],
            internal_buffer_index: 0,
            commands: HashMap::default(),
        };

        shell.register_builtin_commands();

        shell
    }

    pub fn register_builtin_commands(&mut self) {
        self.register_command(String::from(""), Box::new(nop_command_handler));
        self.register_command(String::from("pudel"), Box::new(pudel_command_handler));
        self.register_command(String::from("daria"), Box::new(daria_command_handler));
        self.register_command(String::from("add"), Box::new(add_command_handler));
        self.register_command(String::from("clear"), Box::new(clear_command_handler));
    }

    pub fn register_command(
        &mut self,
        command_name: String,
        command_handler: Box<dyn Fn(String) -> Result<(), ShellError> + Send + Sync>,
    ) {
        self.commands.insert(command_name, command_handler);
    }

    pub fn handle_backspace(&mut self) {
        if self.internal_buffer_index == 0 {
            return;
        }

        self.internal_buffer_index -= 1;
        print!("{}", 0x8 as char);
    }

    pub fn handle_return(&mut self) {
        println!();

        let full_command: String = self.internal_buffer[0..self.internal_buffer_index]
            .iter()
            .collect();

        match full_command.split(" ").next() {
            Some(command) => match self.commands.get(command) {
                Some(handler) => match handler(full_command) {
                    Ok(_) => {}
                    Err(error) => match error {
                        ShellError::InvalidArgumentsError => println!("Invalid argument(s)"),
                    },
                },
                None => {
                    println!("Unknown command `{}`", command);
                }
            },
            None => {}
        }

        print!("> ");
        self.internal_buffer_index = 0;
    }
}
