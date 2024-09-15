use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use crate::statics::{BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::vga_buffer::Color::{Black, White};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(ColorCode::new(White, Black)));
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub(crate) fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(_color_code: ColorCode) -> Writer {
        Writer {
            column_position: 0,
            row_position: 0,
            color_code: _color_code,
            buffer: unsafe {
                &mut *(0xb8000 as *mut Buffer)
            },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.print_new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.print_new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    character: byte,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn print_new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;

        // Shift the screen up
        if self.row_position >= BUFFER_HEIGHT {
            for y in 1..BUFFER_HEIGHT {
                for x in 0..BUFFER_WIDTH {
                    let char = self.buffer.chars[y][x].read();
                    self.buffer.chars[y - 1][x].write(char)
                }
            }
            self.row_position = BUFFER_HEIGHT - 1;
        }

        // Clear last row
        for x in 0..BUFFER_WIDTH {
            let char = ScreenChar {
                character: 0,
                color_code: self.color_code,
            };

            self.buffer.chars[BUFFER_HEIGHT - 1][x].write(char);
        }
    }
}


impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_str(string);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}