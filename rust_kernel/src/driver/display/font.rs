use crate::driver::display::frame_buffer::{Color, FRAME_BUFFER_WRITER};
use crate::driver::display::image::{AssetAtlas, PPMFormat};
use crate::utils::locked::Locked;
use core::fmt;

pub static CURSOR: Locked<Cursor> = Locked::new(Cursor::default());

pub const ASCII_TABLE_START: usize = 32;
pub const ASCII_TABLE_END: usize = 126;

pub struct Font {
    width: usize,
    height: usize,
    image: PPMFormat,
}

impl Font {
    pub const fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            image: PPMFormat::default(),
        }
    }

    pub fn new(image: PPMFormat) -> Self {
        let character_count = ASCII_TABLE_END - ASCII_TABLE_START + 1;

        Self {
            height: image.height() / character_count,
            width: image.width(),
            image,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn max_count_on_width(&self) -> usize {
        let frame_buffer_writer = FRAME_BUFFER_WRITER.lock();
        frame_buffer_writer.width() / self.width()
    }

    pub fn max_count_on_height(&self) -> usize {
        let frame_buffer_writer = FRAME_BUFFER_WRITER.lock();
        frame_buffer_writer.height() / self.height()
    }

    pub fn render(&self, x: usize, y: usize, char: char, color: Color) {
        self.render_asset(x, y, 0, char as usize - ASCII_TABLE_START, color)
    }
}

impl AssetAtlas for Font {
    fn get_asset_width(&self) -> usize {
        self.width
    }

    fn get_asset_height(&self) -> usize {
        self.height
    }

    fn render_box(&self, x: usize, y: usize, box_x: usize, box_y: usize, box_width: usize, box_height: usize, color: Color) {
        self.image.render_box(x, y, box_x, box_y, box_width, box_height, color)
    }
}

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

    pub fn render(&mut self, char: char, color: Color) {
        if char == '\n' {
            self.render_new_line();
            return;
        }

        self.font.render(self.x * self.font.width, self.y * self.font.height, char, color);

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
            // TODO Shift the screen up
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
        for char in string.chars() {
            self.render(char, Color::new(255, 255, 0));
        }

        Ok(())
    }
}
