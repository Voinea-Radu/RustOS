use crate::driver::display::frame_buffer::FRAME_BUFFER;
use crate::driver::display::image::{AssetAtlas, PPMFormat};
use crate::utils::color::Color;

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
        let frame_buffer_writer = FRAME_BUFFER.lock();
        frame_buffer_writer.width() / self.width()
    }

    pub fn max_count_on_height(&self) -> usize {
        let frame_buffer_writer = FRAME_BUFFER.lock();
        frame_buffer_writer.height() / self.height()
    }

    pub fn render(&self, x: usize, y: usize, char: char, color: &Color) {
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

    fn render_box(&self, x: usize, y: usize, box_x: usize, box_y: usize, box_width: usize, box_height: usize, color: &Color) {
        self.image.render_box(x, y, box_x, box_y, box_width, box_height, color)
    }
}

