use sdl3::pixels::{Color, PixelFormat};
use sdl3::render::{create_renderer, Canvas, ScaleMode};
use sdl3::video::Window;
use crate::screen;
use crate::screen::{Screen, PIXEL_MASKS, SCREEN_HEIGHT, SCREEN_WIDTH};

pub const COLOR_BLACK: Color = Color::RGBA(0, 0, 0, 255);
pub const COLOR_WHITE: Color = Color::RGBA(255, 255, 255, 255); // A B G R
pub const COLOR_GREEN: Color = Color::RGBA(0, 255, 0, 255);

const SCREEN_SCALE: usize = 16;

pub struct GameWindow {
    canvas: Canvas<Window>,
}

impl GameWindow {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip8",
                                            (screen::SCREEN_WIDTH * SCREEN_SCALE) as u32,
                                            (screen::SCREEN_HEIGHT * SCREEN_SCALE) as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = create_renderer(window, None).unwrap();
        canvas.set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32).unwrap();
        canvas.set_draw_color(COLOR_BLACK);
        canvas.clear();
        canvas.present();
        Self {
            canvas,
        }
    }

    pub fn update(&mut self, screen: &Screen) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(
            PixelFormat::from_masks(PIXEL_MASKS),
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32).unwrap();
        texture.update(None,&screen.pixels,SCREEN_WIDTH * screen::BYTES_PER_PIXEL).unwrap();
        texture.set_scale_mode(ScaleMode::Nearest);
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}