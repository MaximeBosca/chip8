use sdl3::pixels::{Color, PixelFormat, PixelMasks};
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SCALE: usize = 16;

pub const ON_COLOR: Color = Color::GREEN;
pub const OFF_COLOR: Color = Color::BLACK;
pub const ALT_COLOR: Color = Color::RED;

pub const MARGIN: usize = 1;

pub const PIXEL_MASKS: PixelMasks = PixelMasks {
    bpp: 32,
    rmask: 0x000000FF,
    gmask: 0x0000FF00,
    bmask: 0x00FF0000,
    amask: 0xFF000000,
};

pub struct ScreenConfig {
    pub dimensions: ScreenDimensions,
    pub scale: usize,
    pub margin: usize,
    pub bytes_per_pixel: usize,
    pub pixel_format: PixelFormat,
    pub colors: ScreenColors,
}

#[derive(Debug)]
pub struct ScreenColors {
    pub on_color: Color,
    pub off_color: Color,
    pub alt_color: Color,
}

impl ScreenColors {
    pub fn new(on_color: Color, off_color: Color, alt_color: Color) -> Self {
        Self {
            on_color,
            off_color,
            alt_color,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScreenDimensions {
    pub width: usize,
    pub height: usize,
}

impl ScreenDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[allow(dead_code)]
impl ScreenConfig {
    pub fn new(
        dimensions: ScreenDimensions,
        scale: usize,
        margin: usize,
        pixel_masks: PixelMasks,
        colors: ScreenColors,
    ) -> Self {
        ScreenConfig {
            dimensions,
            scale,
            margin,
            bytes_per_pixel: pixel_masks.bpp as usize / 8,
            pixel_format: PixelFormat::from_masks(pixel_masks),
            colors,
        }
    }
    pub fn default() -> Self {
        Self::new(
            ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            SCREEN_SCALE,
            MARGIN,
            PIXEL_MASKS,
            ScreenColors::new(Color::GREEN, Color::BLACK, Color::RED),
        )
    }
    pub fn pitch(&self) -> usize {
        self.dimensions.width * self.bytes_per_pixel
    }
    pub fn off_color_u8(&self) -> Box<[u8]> {
        self.color_to_u8(self.colors.off_color)
    }
    pub fn on_color_u8(&self) -> Box<[u8]> {
        self.color_to_u8(self.colors.on_color)
    }
    pub fn scaled_margin(&self) -> usize {
        self.margin * self.scale
    }
    pub fn unscaled_margin(&self) -> usize {
        self.margin
    }
    pub fn game_window_width(&self) -> u32 {
        (self.dimensions.width * SCREEN_SCALE) as u32
    }
    pub fn game_window_height(&self) -> u32 {
        (self.dimensions.height * SCREEN_SCALE) as u32
    }
    fn color_to_u8(&self, color: Color) -> Box<[u8]> {
        Box::from(color.to_u32(&self.pixel_format).to_le_bytes())
    }
}
