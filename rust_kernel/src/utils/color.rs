#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiColor {
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

impl AnsiColor {
    pub fn as_foreground(&self) -> &str {
        match self {
            AnsiColor::Reset => "\x1B[0m",
            AnsiColor::Default => "\x1B[39m",
            AnsiColor::Black => "\x1B[30m",
            AnsiColor::Blue => "\x1B[34m",
            AnsiColor::Green => "\x1B[32m",
            AnsiColor::Cyan => "\x1B[36m",
            AnsiColor::Red => "\x1B[31m",
            AnsiColor::Magenta => "\x1B[35m",
            AnsiColor::Brown => "\x1B[33m",
            AnsiColor::LightGray => "\x1B[37m",
            AnsiColor::DarkGray => "\x1B[90m",
            AnsiColor::LightBlue => "\x1B[94m",
            AnsiColor::LightGreen => "\x1B[92m",
            AnsiColor::LightCyan => "\x1B[96m",
            AnsiColor::LightRed => "\x1B[91m",
            AnsiColor::Pink => "\x1B[95m",
            AnsiColor::Yellow => "\x1B[93m",
            AnsiColor::White => "\x1B[97m",
        }
    }

    pub fn as_background(&self) -> &str {
        match self {
            AnsiColor::Reset => "\x1B[0m",
            AnsiColor::Default => "\x1B[49m",
            AnsiColor::Black => "\x1B[40m",
            AnsiColor::Blue => "\x1B[44m",
            AnsiColor::Green => "\x1B[42m",
            AnsiColor::Cyan => "\x1B[46m",
            AnsiColor::Red => "\x1B[41m",
            AnsiColor::Magenta => "\x1B[45m",
            AnsiColor::Brown => "\x1B[43m",
            AnsiColor::LightGray => "\x1B[47m",
            AnsiColor::DarkGray => "\x1B[100m",
            AnsiColor::LightBlue => "\x1B[104m",
            AnsiColor::LightGreen => "\x1B[102m",
            AnsiColor::LightCyan => "\x1B[106m",
            AnsiColor::LightRed => "\x1B[101m",
            AnsiColor::Pink => "\x1B[105m",
            AnsiColor::Yellow => "\x1B[103m",
            AnsiColor::White => "\x1B[107m",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub foreground: AnsiColor,
    pub background: AnsiColor,
}

impl Color {
    pub fn new(foreground: AnsiColor, background: AnsiColor) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn new_simple(foreground: AnsiColor) -> Self {
        Self {
            foreground,
            background: AnsiColor::Default,
        }
    }

    pub fn new_reset() -> Self {
        Self {
            foreground: AnsiColor::Reset,
            background: AnsiColor::Reset,
        }
    }

    pub fn get_foreground(&self) ->&str{
        self.foreground.as_foreground()
    }

    pub fn get_background(&self) -> &str{
        self.background.as_background()
    }
}
