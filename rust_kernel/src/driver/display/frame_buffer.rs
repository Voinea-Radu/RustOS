use crate::driver::display::cursor::CURSOR;
use crate::utils::color::Color;
use crate::utils::locked::Locked;
use bootloader_api::info::PixelFormat;

pub static FRAME_BUFFER: Locked<FrameBuffer> = Locked::new(FrameBuffer::default());

pub struct FrameBuffer {
    buffer: &'static mut [u8],
    pixel_format: PixelFormat,
    bytes_per_pixel: usize,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub const fn default() -> Self {
        Self {
            buffer: &mut [],
            pixel_format: PixelFormat::Rgb,
            bytes_per_pixel: 0,
            width: 0,
            height: 0,
        }
    }

    pub const fn new(
        frame_buffer: &'static mut [u8],
        pixel_format: PixelFormat,
        bytes_per_pixel: usize,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            buffer: frame_buffer,
            pixel_format,
            bytes_per_pixel,
            width,
            height,
        }
    }

    pub fn update(&mut self, new_data: FrameBuffer) {
        self.buffer = new_data.buffer;
        self.pixel_format = new_data.pixel_format;
        self.bytes_per_pixel = new_data.bytes_per_pixel;
        self.width = new_data.width;
        self.height = new_data.height;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn draw_rectangle(&mut self, x: usize, y: usize, length: usize, height: usize, color: &Color) {
        for x_offset in 0..length {
            for y_offset in 0..height {
                self.draw_pixel_raw(
                    x + x_offset,
                    y + y_offset,
                    color.red,
                    color.green,
                    color.blue,
                );
            }
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.draw_pixel_raw(x, y, color.red, color.green, color.blue)
    }

    pub fn draw_pixel_raw(&mut self, x: usize, y: usize, red: u8, green: u8, blue: u8) {
        let color_bytes: [u8; 4] = self.get_colors_bytes_raw(red, green, blue);

        let real_x: usize = x * self.bytes_per_pixel;
        let real_y: usize = y * self.width * self.bytes_per_pixel;

        let start_index: usize = real_y + real_x;
        let end_index: usize = real_y + real_x + self.bytes_per_pixel;

        if (end_index) >= self.buffer.len() {
            return;
        }

        self.buffer[start_index..end_index].copy_from_slice(&color_bytes[..self.bytes_per_pixel]);
    }

    pub fn get_colors_bytes(&mut self, color: &Color) -> [u8; 4] {
        self.get_colors_bytes_raw(color.red, color.green, color.blue)
    }

    pub fn get_colors_bytes_raw(&mut self, red: u8, green: u8, blue: u8) -> [u8; 4] {
        match self.pixel_format {
            PixelFormat::Rgb => [red, green, blue, 0],
            PixelFormat::Bgr => [blue, green, red, 0],
            _ => panic!("Unknown / Unsupported pixel format in frame buffer."),
        }
    }

    pub fn fill_screen(&mut self, color: &Color) {
        let color_bytes = self.get_colors_bytes(color);

        for chunk in self.buffer.chunks_exact_mut(self.bytes_per_pixel) {
            chunk.copy_from_slice(&color_bytes[..self.bytes_per_pixel]);
        }
    }

    pub fn clear_screen(&mut self) {
        self.buffer.fill(0);
        CURSOR.lock().clear_screen()
    }

    pub fn shift_up(&mut self, shift_by: usize) {
        let start_index = shift_by * self.width * self.bytes_per_pixel;
        let buffer_len = self.buffer.len();

        if start_index >= buffer_len {
            self.buffer.fill(0);
            return;
        }

        let transfer_size = buffer_len - start_index;

        self.buffer.copy_within(start_index..buffer_len, 0);
        self.buffer[transfer_size..buffer_len].fill(0);
    }
}
