use crate::driver::display::font::Font;
use crate::driver::display::frame_buffer::FRAME_BUFFER;
use crate::utils::color::Color;
use crate::utils::locked::Locked;
use core::fmt;

pub static CURSOR: Locked<Cursor> = Locked::new(Cursor::default());

pub struct Cursor {
    font: Font,
    x: usize,
    y: usize,
}

impl Cursor {
    pub const fn new(font: Font) -> Self {
        Self { font, x: 0, y: 0 }
    }

    pub const fn default() -> Self {
        Self {
            font: Font::default(),
            x: 0,
            y: 0,
        }
    }

    pub fn render_str(&mut self, string: &str, color: &Color) {
        for char in string.chars() {
            self.render(char, color);
        }
    }

    pub fn render(&mut self, char: char, color: &Color) {
        if char == '\n' {
            self.render_new_line();
            return;
        }

        self.font.render(self.x * self.font.width(), self.y * self.font.height(), char, color);

        self.x += 1;

        if self.x >= self.font.max_count_on_width() {
            self.render_new_line();
        }
    }

    pub fn render_new_line(&mut self) {
        self.x = 0;
        self.y += 1;

        let max_y: usize = self.font.max_count_on_height();

        if self.y >= max_y {
            FRAME_BUFFER.lock().shift_up(self.font.height());

            self.y -= 1;
        }
    }

    pub fn clear_screen(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn update(&mut self, new_data: Cursor) {
        self.x = new_data.x;
        self.y = new_data.y;
        self.font = new_data.font;
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.render_str(string, &Color::new(255, 255, 0));
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments, color: Option<Color>) {
    use core::fmt::Write;
    let mut cursor = CURSOR.lock();

    x86_64::instructions::interrupts::without_interrupts(|| {
        match color {
            None => cursor.write_fmt(args).unwrap(),
            Some(color) => cursor.render_str(args.as_str().unwrap(), &color),
        }
    })
}

/**
print!("Example {} {} {}", 1, 2, 3);
**/
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::driver::display::cursor::_print(format_args!($($arg)*), None)
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
        $crate::driver::display::cursor::_print(format_args!($($arg),*), Some($color))
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
