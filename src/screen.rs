extern crate sdl3;
use sdl3::pixels::{Color, PixelFormat, PixelMasks};
use sdl3::rect::Point;
use sdl3::render::{create_renderer, Canvas, ScaleMode, TextureAccess};
use sdl3::video::Window;
use bit_iter::BitIter;

const WINDOW_WIDTH: usize = 62;
const WINDOW_HEIGHT: usize = 32;
const WINDOW_SCALE: usize = 16;

pub const COLOR_BLACK: Color = Color::RGB(0, 0, 0);
pub const COLOR_WHITE: Color = Color::RGB(255, 255, 255);
pub const COLOR_GREEN: Color = Color::RGB(0, 255, 0);

pub struct Screen {
    canvas: Canvas<Window>,
    width: usize,
    height: usize,
    on_color: Color,
    off_color: Color,
    pixels: [u8; WINDOW_WIDTH * WINDOW_HEIGHT], // TODO : Fix the bit representation of pixels in the array for texture
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
        //canvas.texture_creator().create_texture()
        canvas.set_scale(WINDOW_SCALE as f32, WINDOW_SCALE as f32).unwrap();
        canvas.set_draw_color(COLOR_BLACK);
        canvas.clear();
        canvas.present();
        Self {
            canvas,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            on_color: COLOR_WHITE,
            off_color: COLOR_BLACK,
            pixels: [0; WINDOW_WIDTH*WINDOW_HEIGHT]
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, is_on: bool) {
        self.canvas.set_draw_color(match is_on {
            true => self.on_color,
            false => self.off_color,
        });
        self.canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        self.pixels[y * self.width + x] = is_on as u8 * 255;
    }
    
    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y * self.width + x] = (self.pixels[y * self.width + x]==0) as u8 * 255;
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(None, self.width as u32, self.height as u32).unwrap();
        texture.update(None,&self.pixels,self.width).unwrap();
        texture.set_scale_mode(ScaleMode::Nearest);
        println!("{:?}",texture.format());
        self.canvas.copy(&texture, None, None).unwrap();
    }
    
    pub fn draw_byte(&mut self, byte: &u8, x: usize, y: usize) {
        for index in BitIter::from(*byte) {
            self.draw_pixel(x + 8 - index, y, true);
        }
    }
    
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.off_color);
        self.canvas.clear();
    }
    
    pub fn update(&mut self) {
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

