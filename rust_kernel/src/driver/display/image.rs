use crate::driver::display::frame_buffer::{ FRAME_BUFFER};
use core::cmp::min;
use crate::utils::color::Color;

pub struct PPMFormat {
    #[allow(unused)]
    p_value: u8,
    width: usize,
    height: usize,
    possible_colors: usize,
    pub data: &'static [u8],
}

impl PPMFormat {
    pub const fn default() -> Self {
        Self {
            p_value: 0,
            width: 0,
            height: 0,
            possible_colors: 0,
            data: &[],
        }
    }

    pub fn new(data: &'static [u8]) -> Self {
        let mut cursor: usize = 0;
        cursor += 1; // Skip 'P'

        let (p_value, cursor) = Self::read_number(data, cursor, '\n');

        if p_value != 6 {
            panic!("Image format PPM P{p_value} not supported.")
        }

        let (width, cursor) = Self::read_number(data, cursor, ' ');
        let (height, cursor) = Self::read_number(data, cursor, '\n');
        let (possible_colors, cursor) = Self::read_number(data, cursor, '\n');

        let data = &data[cursor..data.len()];

        Self {
            p_value: p_value as u8,
            width,
            height,
            possible_colors,
            data,
        }
    }

    /**
    @return (number, new_cursor)
    */
    fn read_number(data: &'static [u8], cursor: usize, end_char: char) -> (usize, usize) {
        let mut cursor = cursor;

        let start = cursor;
        while data[cursor] != end_char as u8 {
            cursor += 1;
        }

        let mut number: usize = 0;

        for digit_index in start..cursor {
            number *= 10;
            number += (data[digit_index] - ('0' as u8)) as usize
        }

        cursor += 1; // Skip the end_char

        (number, cursor)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn possible_colors(&self) -> usize {
        self.possible_colors
    }

    pub fn render(&self, x: usize, y: usize) {
        let mut frame_buffer_writer = FRAME_BUFFER.lock();

        for y_offset in 0..self.height() {
            for x_offset in 0..self.width() {
                frame_buffer_writer.draw_pixel_raw(
                    x + x_offset,
                    y + y_offset,
                    self.data[y_offset * self.width() * 3 + x_offset * 3],
                    self.data[y_offset * self.width() * 3 + x_offset * 3 + 1],
                    self.data[y_offset * self.width() * 3 + x_offset * 3 + 2],
                )
            }
        }
    }

    pub fn render_box(&self, x: usize, y: usize, box_x: usize, box_y: usize, box_width: usize, box_height: usize, color: &Color) {
        let mut frame_buffer_writer = FRAME_BUFFER.lock();

        for y_offset in 0..min(self.height(), box_height) {
            for x_offset in 0..min(self.width(), box_width) {
                let first_byte_index: usize = (y_offset + box_y) * 3 * self.width() + (x_offset + box_x) * 3;

                let red = Self::apply_hue(self.data[first_byte_index + 0], color.red);
                let green = Self::apply_hue(self.data[first_byte_index + 1], color.green);
                let blue = Self::apply_hue(self.data[first_byte_index + 2], color.blue);

                frame_buffer_writer.draw_pixel_raw(
                    x + x_offset, y + y_offset,
                    red,
                    green,
                    blue,
                )
            }
        }
    }

    #[inline]
    pub fn apply_hue(base: u8, color: u8) -> u8 {
        if color == 255 {
            return base;
        }
        (base as usize * color as usize / 255) as u8
    }
}
pub trait AssetAtlas {
    fn get_asset_width(&self) -> usize;
    fn get_asset_height(&self) -> usize;
    fn render_box(&self, x: usize, y: usize, box_x: usize, box_y: usize, box_width: usize, box_height: usize, color:& Color);

    /**
    @arg x - the x position to render at
    @arg y - the y position to render at
    @arg local_x - local_x'th row in the AssetAtlas
    @arg local_y - local_y'th column in the AssetAtlas
    **/
    fn render_asset(&self, x: usize, y: usize, local_x: usize, local_y: usize, color: &Color) {
        self.render_box(
            x,
            y,
            local_x * self.get_asset_width(),
            local_y * self.get_asset_height(),
            self.get_asset_width(),
            self.get_asset_height(),
            color,
        );
    }
}

