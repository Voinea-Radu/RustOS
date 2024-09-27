use crate::driver::display::font::CURSOR;
use crate::utils::locked::Locked;
use bootloader_api::info::PixelFormat;

pub static FRAME_BUFFER_WRITER: Locked<FrameBufferWriter> =
    Locked::new(FrameBufferWriter::default());

pub struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

pub struct FrameBufferWriter {
    frame_buffer: &'static mut [u8],
    pixel_format: PixelFormat,
    bytes_per_pixel: usize,
    width: usize,
    height: usize,
}

impl FrameBufferWriter {
    pub const fn default() -> Self {
        Self {
            frame_buffer: &mut [],
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
            frame_buffer,
            pixel_format,
            bytes_per_pixel,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        length: usize,
        height: usize,
        color: Color,
    ) {
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

    pub fn draw_horizontal_line(&mut self, x: usize, y: usize, length: usize, color: Color) {
        self.draw_rectangle(x, y, length, 1, color);
    }

    pub fn draw_vertical_line(&mut self, x: usize, y: usize, height: usize, color: Color) {
        self.draw_rectangle(x, y, 1, height, color);
    }

    #[deprecated] // Use the #draw_pixel_raw method in order to reduce on copies of the color
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.draw_pixel_raw(x, y, color.red, color.green, color.blue)
    }

    pub fn draw_pixel_raw(&mut self, x: usize, y: usize, red: u8, green: u8, blue: u8) {
        let color_bytes: [u8; 4] = match self.pixel_format {
            PixelFormat::Rgb => [red, green, blue, 0],
            PixelFormat::Bgr => [blue, green, red, 0],
            _ => panic!("Unknown / Unsupported pixel format in frame buffer."),
        };

        let real_x: usize = x * self.bytes_per_pixel;
        let real_y: usize = y * self.width * self.bytes_per_pixel;

        if (real_y + real_x + self.bytes_per_pixel) >= self.frame_buffer.len() {
            return;
        }

        self.frame_buffer[(real_y + real_x)..(real_y + real_x + self.bytes_per_pixel)]
            .copy_from_slice(&color_bytes[..self.bytes_per_pixel]);
    }

    pub fn update(&mut self, new_data: FrameBufferWriter) {
        self.frame_buffer = new_data.frame_buffer;
        self.pixel_format = new_data.pixel_format;
        self.bytes_per_pixel = new_data.bytes_per_pixel;
        self.width = new_data.width;
        self.height = new_data.width;
    }

    pub fn get_colors_bytes(&mut self, color: Color) -> [u8; 4] {
        match self.pixel_format {
            PixelFormat::Rgb => [color.red, color.green, color.blue, 0],
            PixelFormat::Bgr => [color.blue, color.green, color.red, 0],
            _ => panic!("Unknown / Unsupported pixel format in frame buffer."),
        }
    }

    pub fn fill_screen(&mut self, color: Color) {
        let color_bytes = self.get_colors_bytes(color);

        let mut start: usize = 0;

        for _ in 0..self.frame_buffer.len() / self.bytes_per_pixel {
            self.frame_buffer[start..(start + self.bytes_per_pixel)].copy_from_slice(&color_bytes[..self.bytes_per_pixel]);
            start += self.bytes_per_pixel;
        }
    }

    pub fn clear_screen(&mut self) {
        for index in 0..self.frame_buffer.len() {
            self.frame_buffer[index] = 0;
        }
        CURSOR.lock().clear_screen()
    }

    pub fn rainbow(&mut self) {
        loop {
            self.fill_screen(Color::new(255, 0, 0));
            self.fill_screen(Color::new(0, 255, 0));
            self.fill_screen(Color::new(0, 0, 255));
        }
    }
}
