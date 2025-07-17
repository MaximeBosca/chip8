extern crate sdl3;
use sdl3::pixels::{Color, PixelFormat, PixelMasks};
use sdl3::render::{Canvas, ScaleMode};
use sdl3::video::Window;
use bit_iter::BitIter;

pub(crate) const SCREEN_WIDTH: usize = 64;
pub(crate) const SCREEN_HEIGHT: usize = 32;

pub const PIXEL_MASKS: PixelMasks = PixelMasks {
    bpp: 32,
    rmask: 0x000000FF,
    gmask: 0x0000FF00,
    bmask: 0x00FF0000,
    amask: 0xFF000000,
};

pub(crate) const BYTES_PER_PIXEL: usize = PIXEL_MASKS.bpp as usize / 8;

pub struct Screen {
    width: usize,
    height: usize,
    on_color: Color,
    off_color: Color,
    pub(crate) pixels: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL],
    pixel_format: PixelFormat,
}

impl Screen {
    pub fn new(on_color: Color, off_color: Color) -> Self {
        Self {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            on_color,
            off_color,
            pixels: [0; SCREEN_WIDTH * SCREEN_HEIGHT * BYTES_PER_PIXEL],
            pixel_format: PixelFormat::from_masks(PIXEL_MASKS),
        }
    }
    pub(crate) fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool{
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
        let index = (y * self.width + x) * BYTES_PER_PIXEL;
        self.flip_pixel(index)
    }

    fn flip_pixel(&mut self, index: usize) -> bool {
        let mut flipped_off = false;
        let mut r = self.pixels[index];
        let mut g = self.pixels[index+1];
        let mut b = self.pixels[index+2];
        let mut a = self.pixels[index+3];
        if (r,g,b,a) == self.on_color.rgba() {
            (r,g,b,a) = self.off_color.rgba();
            flipped_off = true;
        } else {
            (r,g,b,a) = self.on_color.rgba();
        }
        self.pixels[index] = r;
        self.pixels[index+1] = g;
        self.pixels[index+2] = b;
        self.pixels[index+3] = a;
        flipped_off
    }
    
    pub fn draw_byte(&mut self, byte: &u8, x: usize, y: usize) -> bool{
        let mut flipped_off = false;
        for index in BitIter::from(*byte) {
            flipped_off = flipped_off | self.draw_pixel(x + 7 - index, y);
        }
        flipped_off
    }

    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn clear(&mut self) {
        let mut i = 0;
        let (r, g, b, a) = self.off_color.rgba();
        while i < self.pixels.len() {
            self.pixels[i] = r;
            self.pixels[i+1] = g;
            self.pixels[i+2] = b;
            self.pixels[i+3] = a;
            i += BYTES_PER_PIXEL;
        }
    }
    
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn on_color(&mut self, color: Color) {
        self.on_color = color;
    }
    pub fn off_color(&mut self, color: Color) {
        self.off_color = color;
    }
}

