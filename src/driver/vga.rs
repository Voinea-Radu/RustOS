use crate::utils::color::Color;
use crate::utils::color::ColorCode::{Black, White};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(Color::new(White, Black)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    // 0-255. https://en.wikipedia.org/wiki/Code_page_437
    pub character: u8,
    // BBBBFFFF
    // F is the foreground color
    // B is the background color
    pub color: u8,
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub color: Color,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(_color: Color) -> Writer {
        Writer {
            column_position: 0,
            row_position: 0,
            color: _color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        #[allow(deprecated)]
        self.write_byte_color(byte, self.color);
    }

    #[deprecated]
    pub fn write_byte_color(&mut self, byte: u8, color: Color) {
        match byte {
            b'\n' => self.print_new_line(),
            0x8 => self.handle_backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.print_new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    character: byte,
                    color: color.get_vga_color(),
                });

                self.column_position += 1;
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        let row = self.row_position;
        let col = self.column_position;

        if col == 0 {
            return;
        }

        self.buffer.chars[row][col - 1].write(ScreenChar {
            character: 0,
            color: self.color.get_vga_color(),
        });

        self.column_position -= 1;
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
                0x20..=0x7e | b'\n' | 0x8 => self.write_byte_color(byte, color),
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
                color: self.color.get_vga_color(),
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

    pub fn clear(&mut self) {
        self.column_position = 0;
        self.row_position = 0;
        for x in 0..BUFFER_WIDTH {
            for y in 0..BUFFER_HEIGHT {
                let char = ScreenChar {
                    character: 0,
                    color: self.color.get_vga_color(),
                };

                self.buffer.chars[y][x].write(char);
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_str(string);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments, color: Option<Color>) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        let old_color = WRITER.lock().color();

        match color {
            None => {
                WRITER.lock().write_fmt(args).unwrap();
            }
            Some(color) => {
                WRITER.lock().set_color(color);
                WRITER.lock().write_fmt(args).unwrap();
                WRITER.lock().set_color(old_color);
            }
        }
    })
}

/**
print!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::driver::vga::_print(format_args!($($arg)*), None)
    };
}

/**
println!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:expr),*) => {
        $crate::print!("{}\n", format_args!($($arg),*))
    };
}

/**
print_color!("Example {} {} {}", 1, 2, 3 => Color::new(LightRed, Black));
**/
#[macro_export]
macro_rules! print_color {
    ($($arg:expr),* => $color:expr) => {
        $crate::driver::vga::_print(format_args!($($arg),*), Some($color))
    };
}

/**
println_color!("Example {} {} {}", 1, 2, 3 => Color::new(LightRed, Black));
**/
#[macro_export]
macro_rules! println_color {
    () => {
        $crate::print!("\n")
    };
    ($($arg:expr),* => $color:expr) => {
        $crate::print_color!("{}\n", format_args!($($arg),*) => $color)
    };
}
