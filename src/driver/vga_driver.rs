use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use crate::driver::vga_driver::ColorCode::{Black, White};
use crate::utils::statics::{BUFFER_HEIGHT, BUFFER_WIDTH};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(Color::new(White, Black)));
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorCode {
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
pub struct Color(u8);

impl Color {
    pub(crate) fn new(foreground: ColorCode, background: ColorCode) -> Color {
        Color((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color: Color,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color: Color,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(_color: Color) -> Writer {
        Writer {
            column_position: 0,
            row_position: 0,
            color: _color,
            buffer: unsafe {
                &mut *(0xb8000 as *mut Buffer)
            },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        #[allow(deprecated)]
        self.write_byte_color(byte, self.color);
    }

    #[deprecated]
    pub fn write_byte_color(&mut self, byte: u8, _color: Color) {
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
                    color: _color,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, string: &str) {
        #[allow(deprecated)]
        self.write_str_color(string, self.color);
    }

    #[deprecated]
    pub fn write_str_color(&mut self, string: &str, color: Color) {
        for byte in string.bytes() {
            match byte {
                #[allow(deprecated)]
                0x20..=0x7e | b'\n' => self.write_byte_color(byte, color),
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
                color: self.color,
            };

            self.buffer.chars[BUFFER_HEIGHT - 1][x].write(char);
        }
    }

    pub fn color(&mut self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[allow(dead_code)]
    pub fn reset_color(&mut self) {
        self.set_color(Color::new(White, Black))
    }
}


impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_str(string);
        Ok(())
    }
}

