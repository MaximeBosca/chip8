use sdl3::render::{create_renderer, Canvas, ScaleMode};
use sdl3::video::Window;
use crate::screen::Screen;
use crate::screen_config::ScreenConfig;

const SCREEN_SCALE: usize = 16;

pub struct GameWindow {
    canvas: Canvas<Window>,
    screen_config: ScreenConfig,
}

impl GameWindow {
    pub fn new(screen_config: ScreenConfig) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chip8",
                                            (screen_config.width * SCREEN_SCALE) as u32,
                                            (screen_config.height * SCREEN_SCALE) as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = create_renderer(window, None).unwrap();
        canvas.set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32).unwrap();
        canvas.set_draw_color(screen_config.off_color);
        canvas.clear();
        canvas.present();
        Self {
            canvas,
            screen_config
        }
    }

    pub fn update(&mut self, screen: &Screen) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(
            self.screen_config.pixel_format,
            self.screen_config.width as u32,
            self.screen_config.height as u32).unwrap();
        texture.update(None,&screen.pixels,self.screen_config.pitch()).unwrap();
        texture.set_scale_mode(ScaleMode::Nearest);
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}