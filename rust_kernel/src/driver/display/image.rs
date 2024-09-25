use crate::driver::display::frame_buffer::FRAME_BUFFER_WRITER;
use crate::println_serial;

trait ImageFormat {
    fn render();
}

pub struct PPMFormat {
    p_value: u8,
    width: usize,
    height: usize,
    possible_colors: usize,
    pub data: &'static [u8],
}

impl PPMFormat {
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
        let mut frame_buffer_writer = FRAME_BUFFER_WRITER.lock();

        for y_offset in 0..self.height() {
            for x_offset in 0..self.width() {
                frame_buffer_writer.draw_pixel_raw
                (x + x_offset, y + y_offset,
                 self.data[y_offset * self.width() * 3 + x_offset * 3],
                 self.data[y_offset * self.width() * 3 + x_offset * 3 + 1],
                 self.data[y_offset * self.width() * 3 + x_offset * 3 + 2],
                )
            }
        }
    }
}
