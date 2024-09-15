use crate::statics::{BUFFER_HEIGHT, BUFFER_WIDTH};

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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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

                self.buffer.chars[row][col] = ScreenChar {
                    character: byte,
                    color_code: self.color_code,
                };

                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, string: &str) {
        for byte in string.bytes(){
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn print_new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;
    }
}
