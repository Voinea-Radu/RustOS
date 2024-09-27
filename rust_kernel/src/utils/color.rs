#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiColorType {
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

impl AnsiColorType {
    pub fn as_foreground(&self) -> &str {
        match self {
            AnsiColorType::Reset => "\x1B[0m",
            AnsiColorType::Default => "\x1B[39m",
            AnsiColorType::Black => "\x1B[30m",
            AnsiColorType::Blue => "\x1B[34m",
            AnsiColorType::Green => "\x1B[32m",
            AnsiColorType::Cyan => "\x1B[36m",
            AnsiColorType::Red => "\x1B[31m",
            AnsiColorType::Magenta => "\x1B[35m",
            AnsiColorType::Brown => "\x1B[33m",
            AnsiColorType::LightGray => "\x1B[37m",
            AnsiColorType::DarkGray => "\x1B[90m",
            AnsiColorType::LightBlue => "\x1B[94m",
            AnsiColorType::LightGreen => "\x1B[92m",
            AnsiColorType::LightCyan => "\x1B[96m",
            AnsiColorType::LightRed => "\x1B[91m",
            AnsiColorType::Pink => "\x1B[95m",
            AnsiColorType::Yellow => "\x1B[93m",
            AnsiColorType::White => "\x1B[97m",
        }
    }

    pub fn as_background(&self) -> &str {
        match self {
            AnsiColorType::Reset => "\x1B[0m",
            AnsiColorType::Default => "\x1B[49m",
            AnsiColorType::Black => "\x1B[40m",
            AnsiColorType::Blue => "\x1B[44m",
            AnsiColorType::Green => "\x1B[42m",
            AnsiColorType::Cyan => "\x1B[46m",
            AnsiColorType::Red => "\x1B[41m",
            AnsiColorType::Magenta => "\x1B[45m",
            AnsiColorType::Brown => "\x1B[43m",
            AnsiColorType::LightGray => "\x1B[47m",
            AnsiColorType::DarkGray => "\x1B[100m",
            AnsiColorType::LightBlue => "\x1B[104m",
            AnsiColorType::LightGreen => "\x1B[102m",
            AnsiColorType::LightCyan => "\x1B[106m",
            AnsiColorType::LightRed => "\x1B[101m",
            AnsiColorType::Pink => "\x1B[105m",
            AnsiColorType::Yellow => "\x1B[103m",
            AnsiColorType::White => "\x1B[107m",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnsiColor {
    pub foreground: AnsiColorType,
    pub background: AnsiColorType,
}

impl AnsiColor {
    pub fn new(foreground: AnsiColorType, background: AnsiColorType) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn new_simple(foreground: AnsiColorType) -> Self {
        Self {
            foreground,
            background: AnsiColorType::Default,
        }
    }

    pub fn new_reset() -> Self {
        Self {
            foreground: AnsiColorType::Reset,
            background: AnsiColorType::Reset,
        }
    }

    pub fn get_foreground(&self) -> &str {
        self.foreground.as_foreground()
    }

    pub fn get_background(&self) -> &str {
        self.background.as_background()
    }
}
