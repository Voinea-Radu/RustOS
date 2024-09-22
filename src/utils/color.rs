#![allow(dead_code)]

use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorCode {
    Reset,
    Default,
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Foreground {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Background {}

impl ColorCode {
    pub fn get_vga_code(&self) -> u8 {
        match self {
            ColorCode::Black => 0,
            ColorCode::Default => 0,
            ColorCode::Reset => 0,
            ColorCode::Blue => 1,
            ColorCode::Green => 2,
            ColorCode::Cyan => 3,
            ColorCode::Red => 4,
            ColorCode::Magenta => 5,
            ColorCode::Brown => 6,
            ColorCode::LightGray => 7,
            ColorCode::DarkGray => 8,
            ColorCode::LightBlue => 9,
            ColorCode::LightGreen => 10,
            ColorCode::LightCyan => 11,
            ColorCode::LightRed => 12,
            ColorCode::Pink => 13,
            ColorCode::Yellow => 14,
            ColorCode::White => 15,
        }
    }

    pub fn get_ansi_code_foreground(&self) -> &str {
        match self {
            ColorCode::Reset => "\x1B[0m",
            ColorCode::Default => "\x1B[39m",
            ColorCode::Black => "\x1B[30m",
            ColorCode::Blue => "\x1B[34m",
            ColorCode::Green => "\x1B[32m",
            ColorCode::Cyan => "\x1B[36m",
            ColorCode::Red => "\x1B[31m",
            ColorCode::Magenta => "\x1B[35m",
            ColorCode::Brown => "\x1B[33m",
            ColorCode::LightGray => "\x1B[37m",
            ColorCode::DarkGray => "\x1B[90m",
            ColorCode::LightBlue => "\x1B[94m",
            ColorCode::LightGreen => "\x1B[92m",
            ColorCode::LightCyan => "\x1B[96m",
            ColorCode::LightRed => "\x1B[91m",
            ColorCode::Pink => "\x1B[95m",
            ColorCode::Yellow => "\x1B[93m",
            ColorCode::White => "\x1B[97m",
        }
    }

    pub fn get_ansi_code_background(&self) -> &str {
        match self {
            ColorCode::Reset => "\x1B[0m",
            ColorCode::Default => "\x1B[49m",
            ColorCode::Black => "\x1B[40m",
            ColorCode::Blue => "\x1B[44m",
            ColorCode::Green => "\x1B[42m",
            ColorCode::Cyan => "\x1B[46m",
            ColorCode::Red => "\x1B[41m",
            ColorCode::Magenta => "\x1B[45m",
            ColorCode::Brown => "\x1B[43m",
            ColorCode::LightGray => "\x1B[47m",
            ColorCode::DarkGray => "\x1B[100m",
            ColorCode::LightBlue => "\x1B[104m",
            ColorCode::LightGreen => "\x1B[102m",
            ColorCode::LightCyan => "\x1B[106m",
            ColorCode::LightRed => "\x1B[101m",
            ColorCode::Pink => "\x1B[105m",
            ColorCode::Yellow => "\x1B[103m",
            ColorCode::White => "\x1B[107m",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    foreground: ColorCode,
    background: ColorCode,
}

impl Color {
    pub fn new(_foreground: ColorCode, _background: ColorCode) -> Color {
        Color {
            foreground: _foreground,
            background: _background,
        }
    }

    pub fn new_simple(_foreground: ColorCode) -> Color {
        Self::new(_foreground, ColorCode::Default)
    }

    pub fn reset_ansi_color() -> String {
        let mut output = String::new();

        output.push_str(ColorCode::Reset.get_ansi_code_foreground());
        output.push_str(ColorCode::Reset.get_ansi_code_background());

        output
    }

    pub fn get_vga_color(&self) -> u8 {
        (self.background.get_vga_code()) << 4 | (self.foreground.get_vga_code())
    }

    pub fn get_ansi_color(&self) -> String {
        let mut output = String::new();

        output.push_str(self.foreground.get_ansi_code_foreground());
        output.push_str(self.background.get_ansi_code_background());

        output
    }
}
