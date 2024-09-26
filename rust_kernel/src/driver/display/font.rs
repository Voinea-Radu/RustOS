use crate::driver::display::image::{AssetAtlas, PPMFormat};

pub struct Font {
    font_width: usize,
    font_height: usize,
    image: PPMFormat,
}

impl Font {
    pub fn new(image: PPMFormat) -> Self {
        let character_count = 126 - 33 + 1;

        Self {
            font_height: image.height() / character_count,
            font_width: image.width(),
            image,
        }
    }

    pub fn render(&self, x: usize, y: usize, char: char) {
        self.render_asset(x, y, 0, char as usize - 33)
    }
}

impl AssetAtlas for Font {
    fn get_asset_width(&self) -> usize {
        self.font_width
    }

    fn get_asset_height(&self) -> usize {
        self.font_height
    }

    fn render_box(&self, x: usize, y: usize, box_x: usize, box_y: usize, box_width: usize, box_height: usize) {
        self.image.render_box(x, y, box_x, box_y, box_width, box_height)
    }
}