extern crate sdl3;
use crate::screen_config::ScreenConfig;
use bit_iter::BitIter;

pub struct Screen {
    pub width: usize,
    pub height: usize,
    bytes_per_pixel: usize,
    on_color: Box<[u8]>,
    off_color: Box<[u8]>,
    pub pixels: Vec<u8>,
}

impl Screen {
    pub fn new(config: &ScreenConfig) -> Self {
        Self {
            width: config.width,
            height: config.height,
            bytes_per_pixel: config.bytes_per_pixel,
            on_color: config.on_color_u8(),
            off_color: config.off_color_u8(),
            pixels: vec![0u8; config.width * config.height * config.bytes_per_pixel],
        }
    }
    pub(crate) fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut flipped_off = false;
        for (i, byte) in sprite.iter().enumerate() {
            flipped_off = flipped_off | self.draw_byte(byte, x, y + i)
        }
        flipped_off
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = y * self.width + x;
        self.flip_pixel(index)
    }

    fn flip_pixel(&mut self, index: usize) -> bool {
        let mut flipped_off = false;
        let begin = index * self.bytes_per_pixel;
        let end = (index + 1) * self.bytes_per_pixel;
        let pixel_color = &mut self.pixels[begin..end];
        if pixel_color == &*self.on_color {
            flipped_off = true;
            pixel_color.copy_from_slice(&*self.off_color)
        } else {
            pixel_color.copy_from_slice(&self.on_color)
        }
        flipped_off
    }

    pub fn draw_byte(&mut self, byte: &u8, x: usize, y: usize) -> bool {
        let mut flipped_off = false;
        for index in BitIter::from(*byte) {
            flipped_off = flipped_off | self.draw_pixel(x + 7 - index, y);
        }
        flipped_off
    }

    pub fn clear(&mut self) {
        let mut i = 0;
        while i <= self.pixels.len() - self.bytes_per_pixel {
            let begin = i;
            let end = i + self.bytes_per_pixel;
            self.pixels[begin..end].copy_from_slice(&self.off_color);
            i += self.bytes_per_pixel;
        }
    }
}
