use sdl3::pixels::{Color, PixelFormat, PixelMasks};
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub const COLOR_BLACK: Color = Color::BLACK;
pub const COLOR_WHITE: Color = Color::WHITE;
pub const COLOR_GREEN: Color = Color::GREEN;

pub const PIXEL_MASKS: PixelMasks = PixelMasks {
    bpp: 32,
    rmask: 0x000000FF,
    gmask: 0x0000FF00,
    bmask: 0x00FF0000,
    amask: 0xFF000000,
};


pub struct ScreenConfig {
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
    pub pixel_format : PixelFormat,
    pub on_color: Color,
    pub off_color: Color,
}
impl ScreenConfig {
    pub fn new(width: usize, height: usize, pixel_masks: PixelMasks, on_color: Color, off_color: Color) -> Self {
        ScreenConfig {
            width,
            height,
            bytes_per_pixel: pixel_masks.bpp as usize / 8,
            pixel_format: PixelFormat::from_masks(pixel_masks),
            on_color,
            off_color,
        }
    }
    pub fn default() -> Self {
        Self::new(SCREEN_WIDTH, SCREEN_HEIGHT, PIXEL_MASKS, COLOR_GREEN, COLOR_BLACK)
    }
    pub fn pitch(&self) -> usize {
        self.width * self.bytes_per_pixel
    }
    pub fn off_color_u8(&self) -> Box<[u8]> {
        self.color_to_u8(self.off_color)
    }
    pub fn on_color_u8(&self) -> Box<[u8]> {
        self.color_to_u8(self.on_color)
    }
    fn color_to_u8(&self, color: Color) -> Box<[u8]> {
        Box::from(color.to_u32(&self.pixel_format).to_le_bytes())
    }


}
