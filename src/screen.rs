extern crate sdl3;
use sdl3::pixels::{Color, PixelFormat, PixelMasks};
use sdl3::render::{create_renderer, Canvas, ScaleMode};
use sdl3::video::Window;
use bit_iter::BitIter;

const WINDOW_WIDTH: usize = 64;
const WINDOW_HEIGHT: usize = 32;
const WINDOW_SCALE: usize = 16;

pub const COLOR_BLACK: Color = Color::RGBA(0, 0, 0, 255);
pub const COLOR_WHITE: Color = Color::RGBA(255, 255, 255, 255); // A B G R
pub const COLOR_GREEN: Color = Color::RGBA(0, 255, 0, 255);

const PIXEL_MASKS: PixelMasks = PixelMasks {
    bpp: 32,
    rmask: 0x000000FF,
    gmask: 0x0000FF00,
    bmask: 0x00FF0000,
    amask: 0xFF000000,
};

const BYTES_PER_PIXEL: usize = PIXEL_MASKS.bpp as usize / 8;

pub struct Screen {
    canvas: Canvas<Window>,
    width: usize,
    height: usize,
    on_color: Color,
    off_color: Color,
    pixels: [u8; WINDOW_WIDTH * WINDOW_HEIGHT * BYTES_PER_PIXEL],
    pixel_format: PixelFormat,
}

impl Screen {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window("chip8",
                                            (WINDOW_WIDTH*WINDOW_SCALE) as u32,
                                            (WINDOW_HEIGHT*WINDOW_SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();
        let mut canvas = create_renderer(window, None).unwrap();
        canvas.set_scale(WINDOW_SCALE as f32, WINDOW_SCALE as f32).unwrap();
        canvas.set_draw_color(COLOR_BLACK);
        canvas.clear();
        canvas.present();
        Self {
            canvas,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            on_color: COLOR_GREEN,
            off_color: COLOR_BLACK,
            pixels: [0; WINDOW_WIDTH*WINDOW_HEIGHT*BYTES_PER_PIXEL],
            pixel_format: PixelFormat::from_masks(PIXEL_MASKS),
        }
    }
    pub(crate) fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool{
        let mut flipped_off = false;
        for (i, byte) in sprite.iter().enumerate() {
            flipped_off = flipped_off | self.draw_byte(byte, x, y + i)
        }
        self.update();
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
    
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.off_color);
        self.canvas.clear();
    }
    
    fn update(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(
            self.pixel_format,
            self.width as u32,
            self.height as u32).unwrap();
        texture.update(None,&self.pixels,self.width*BYTES_PER_PIXEL).unwrap();
        texture.set_scale_mode(ScaleMode::Nearest);
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    pub fn width(&self) -> usize {
        self.width
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

