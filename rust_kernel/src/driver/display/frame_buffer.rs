use bootloader_api::info::PixelFormat;

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    pixel_format: PixelFormat,
    bytes_per_pixel: usize,
    width: usize,
    height: usize,
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

impl FrameBufferWriter {
    pub fn new(
        framebuffer: &'static mut [u8],
        pixel_format: PixelFormat, bytes_per_pixel: usize,
        width: usize, height: usize,
    ) -> Self {
        Self {
            framebuffer,
            pixel_format,
            bytes_per_pixel,
            width,
            height,
        }
    }

    pub fn draw_rectangle(&mut self, x: usize, y: usize, length: usize, height: usize, color: Color) {
        for x_offset in 0..length {
            for y_offset in 0..height {
                self.draw_pixel_raw(x + x_offset, y + y_offset, color.red, color.green, color.blue);
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
            _ => panic!("Unknown / Unsupported pixel format in frame buffer.")
        };

        for (byte_index, byte) in color_bytes.iter().enumerate() {
            let real_x: usize = x * self.bytes_per_pixel;
            let real_y: usize = y * self.width * self.bytes_per_pixel;
            self.framebuffer[real_y + real_x + byte_index] = *byte;
        }
    }
}