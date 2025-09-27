use crate::cli::ColorValue;
use sdl3::pixels::{Color, PixelFormat, PixelMasks};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const MARGIN: usize = 16;

pub const PIXEL_MASKS: PixelMasks = PixelMasks {
    bpp: 32,
    rmask: 0x000000FF,
    gmask: 0x0000FF00,
    bmask: 0x00FF0000,
    amask: 0xFF000000,
};

pub struct ScreenConfig {
    pub window_dimensions: Dimensions,
    pub screen_dimensions: Dimensions,
    pub colors: Colors,
    pub bytes_per_pixel: usize,
    pub pixel_format: PixelFormat,
}

#[derive(Debug)]
pub struct Colors {
    pub on_color: Color,
    pub off_color: Color,
    pub alt_color: Color,
}

impl Colors {
    pub fn new(on_color: ColorValue, off_color: ColorValue, alt_color: ColorValue) -> Self {
        Self {
            on_color: to_sdl_color(on_color),
            off_color: to_sdl_color(off_color),
            alt_color: to_sdl_color(alt_color),
        }
    }
}

fn to_sdl_color(color: ColorValue) -> Color {
    match color {
        ColorValue::Green => Color::GREEN,
        ColorValue::Red => Color::RED,
        ColorValue::Yellow => Color::YELLOW,
        ColorValue::Blue => Color::BLUE,
        ColorValue::Magenta => Color::MAGENTA,
        ColorValue::Cyan => Color::CYAN,
        ColorValue::White => Color::WHITE,
        ColorValue::Black => Color::BLACK,
        ColorValue::Grey => Color::GREY,
        ColorValue::Gray => Color::GRAY,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[allow(dead_code)]
impl ScreenConfig {
    pub fn new(window_dimensions: Dimensions, colors: Colors) -> Self {
        ScreenConfig {
            window_dimensions,
            screen_dimensions: Dimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            bytes_per_pixel: PIXEL_MASKS.bpp as usize / 8,
            pixel_format: PixelFormat::from_masks(PIXEL_MASKS),
            colors,
        }
    }
    pub fn pitch(&self) -> usize {
        self.screen_dimensions.width * self.bytes_per_pixel
    }
    pub fn off_color_u8(&self) -> Box<[u8]> {
        self.color_to_u8(self.colors.off_color)
    }
    pub fn on_color_u8(&self) -> Box<[u8]> {
        self.color_to_u8(self.colors.on_color)
    }
    pub fn margin(&self) -> usize {
        MARGIN
    }
    fn color_to_u8(&self, color: Color) -> Box<[u8]> {
        Box::from(color.to_u32(&self.pixel_format).to_le_bytes())
    }
}
