use crate::game_window::controls_writer::game_pad;
use crate::game_window::timer_writer::write_timer;
use crate::screen_config::ScreenConfig;
use crate::state::State;
use sdl3::Sdl;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{Canvas, FRect, ScaleMode, TextureCreator, create_renderer};
use sdl3::surface::Surface;
use sdl3::ttf::Font;
use sdl3::video::{Window, WindowContext};
use std::cmp::max;

mod controls_writer;
mod instruction_writer;
mod registers_writer;
mod stack_writer;
mod timer_writer;

pub const HEADER_FONT_SIZE: f32 = 64.0;
pub const TEXT_FONT_SIZE: f32 = 48.0;

struct ScreenManager {
    screen_config: ScreenConfig,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

pub struct GameWindow<'a> {
    controls_panel: Panel,
    game_panel: Panel,
    header_font: Font<'a>,
    index_panel: Panel,
    instructions_panel: Panel,
    registers_panel: Panel,
    screen_manager: ScreenManager,
    stack_panel: Panel,
    text_font: Font<'a>,
    timer_panel: Panel,
}

impl GameWindow<'_> {
    pub fn new(sdl_context: &Sdl, screen_config: ScreenConfig) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = screen_config.game_window_width() * 2;
        let window_height = screen_config.game_window_height() * 2;

        let window = video_subsystem
            .window("chip8", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = create_renderer(window, None).unwrap();
        canvas.set_draw_color(screen_config.colors.off_color);
        canvas.clear();
        canvas.present();

        let instructions_panel = Panel::new(
            PanelType::Instructions,
            0,
            0,
            window_width / 4,
            window_height,
        );
        let game_panel = Panel::new(
            PanelType::Game,
            instructions_panel.boundaries.right(),
            0,
            window_width / 2,
            window_height / 2,
        );
        let registers_panel = Panel::new(
            PanelType::Registers,
            game_panel.boundaries.right(),
            0,
            window_width / 4,
            window_height,
        );
        let index_panel = Panel::new(
            PanelType::Index,
            game_panel.boundaries.right(),
            0,
            window_width / 4,
            0,
        );
        let timer_panel = Panel::new(
            PanelType::Timer,
            game_panel.boundaries.right(),
            0,
            window_width / 4,
            0,
        );
        let stack_panel = Panel::new(
            PanelType::Stack,
            game_panel.boundaries.right(),
            0,
            window_width / 4,
            0,
        );
        let controls_panel = Panel::new(
            PanelType::Controls,
            instructions_panel.boundaries.right(),
            game_panel.boundaries.bottom(),
            game_panel.boundaries.width(),
            game_panel.boundaries.height(),
        );
        let ttf_context = sdl3::ttf::init().unwrap();
        let header_font = ttf_context
            .load_font("assets/Zolofont.ttf", HEADER_FONT_SIZE)
            .unwrap();
        let text_font = ttf_context
            .load_font("assets/Zolofont.ttf", TEXT_FONT_SIZE)
            .unwrap();
        let texture_creator = canvas.texture_creator();

        let screen_manager = ScreenManager {
            canvas,
            screen_config,
            texture_creator,
        };
        Self {
            screen_manager,
            header_font,
            text_font,
            game_panel,
            instructions_panel,
            registers_panel,
            index_panel,
            timer_panel,
            stack_panel,
            controls_panel,
        }
    }

    pub fn update(&mut self, state: &State) {
        self.screen_manager
            .canvas
            .set_draw_color(self.screen_manager.screen_config.colors.off_color);

        self.screen_manager.canvas.clear();
        self.draw_controls(state);
        self.draw_instructions(state);
        self.draw_registers(state);
        self.draw_index(state);
        self.draw_timer(state);
        self.draw_stack(state);
        self.draw_layout(state);
        self.update_game_screen(state);
        self.screen_manager.canvas.present();
    }

    fn draw_controls(&mut self, state: &State) {
        let remaining_rect = self.write_header(self.controls_panel.clone());
        let drawn_rect = self.screen_manager.write_text(
            controls_writer::write_fn_controls(),
            &self.text_font,
            remaining_rect,
        );
        let remaining_rect = subtract_rect(remaining_rect, drawn_rect, Direction::Up);
        let remaining = self.draw_pressed_keys(state, remaining_rect);
        self.screen_manager.write_text(
            controls_writer::write_game_controls(),
            &self.text_font,
            remaining,
        );
    }

    fn draw_instructions(&mut self, state: &State) {
        let remaining_rect = self.write_header(self.instructions_panel.clone());
        self.screen_manager.write_text(
            &instruction_writer::write_instructions(state),
            &self.text_font,
            remaining_rect,
        );
    }

    fn draw_registers(&mut self, state: &State) {
        let remaining_rect = self.write_header(self.registers_panel.clone());
        let drawn_rect = self.screen_manager.write_text(
            &registers_writer::write_registers(state),
            &self.text_font,
            remaining_rect,
        );
        self.index_panel.boundaries =
            subtract_rect(self.registers_panel.boundaries, drawn_rect, Direction::Up);
    }


    fn draw_layout(&mut self, _state: &State) {
        self.screen_manager
            .canvas
            .set_draw_color(self.screen_manager.screen_config.colors.on_color);
        self.screen_manager
            .canvas
            .draw_rects(&[
                FRect::from(self.instructions_panel.boundaries),
                FRect::from(self.registers_panel.boundaries),
                FRect::from(self.controls_panel.boundaries),
                FRect::from(self.index_panel.boundaries),
                FRect::from(self.stack_panel.boundaries),
            ])
            .unwrap();
    }

    fn update_game_screen(&mut self, state: &State) {
        let mut texture = self
            .screen_manager
            .texture_creator
            .create_texture_target(
                self.screen_manager.screen_config.pixel_format,
                self.screen_manager.screen_config.dimensions.width as u32,
                self.screen_manager.screen_config.dimensions.height as u32,
            )
            .unwrap();
        texture
            .update(
                None,
                &state.screen.pixels,
                self.screen_manager.screen_config.pitch(),
            )
            .unwrap();
        texture.set_scale_mode(ScaleMode::Nearest);
        self.screen_manager
            .canvas
            .copy(&texture, None, self.game_panel.boundaries)
            .unwrap();
    }

    fn write_header(&mut self, panel: Panel) -> Rect {
        let mut drawn_rect =
            self.screen_manager
                .write_text(&panel.header, &self.header_font, panel.boundaries);
        drawn_rect.set_x(panel.boundaries.left());
        drawn_rect.set_width(panel.boundaries.width());
        self.screen_manager
            .canvas
            .set_draw_color(self.screen_manager.screen_config.colors.on_color);
        self.screen_manager
            .canvas
            .draw_rect(FRect::from(drawn_rect))
            .unwrap();
        subtract_rect(panel.boundaries, drawn_rect, Direction::Up)
    }

    fn draw_index(&mut self, state: &State) {
        let remaining_rect = self.write_header(self.index_panel.clone());
        let drawn_rect =
            self.screen_manager
                .write_text(&write_index(state), &self.text_font, remaining_rect);
        self.timer_panel.boundaries =
            subtract_rect(self.index_panel.boundaries, drawn_rect, Direction::Up);
    }

    fn draw_timer(&mut self, state: &State) {
        let remaining_rect = self.write_header(self.timer_panel.clone());
        let drawn_rect =
            self.screen_manager
                .write_text(&write_timer(state), &self.text_font, remaining_rect);
        self.stack_panel.boundaries =
            subtract_rect(self.timer_panel.boundaries, drawn_rect, Direction::Up);
    }
    fn draw_stack(&mut self, state: &State) {
        let remaining_rect = self.write_header(self.stack_panel.clone());
        self.screen_manager.write_text(
            &stack_writer::write_stack(state),
            &self.text_font,
            remaining_rect,
        );
    }

    fn draw_pressed_keys(&mut self, state: &State, rect: Rect) -> Rect {
        let drawn = self.screen_manager.write_text(
            controls_writer::write_hyphen_line(),
            &self.text_font,
            rect,
        );
        let mut remaining_rect = subtract_rect(rect, drawn, Direction::Up);
        let mut drawn = Rect::new(rect.x, rect.y, 0, 0);
        for line in game_pad() {
            let drawn_rect = self.draw_line(line, state, remaining_rect);
            drawn = add_rect(drawn_rect, drawn, Direction::Up);
            remaining_rect = subtract_rect(remaining_rect, drawn_rect, Direction::Up);
        }
        self.screen_manager.write_text(
            controls_writer::write_hyphen_line(),
            &self.text_font,
            remaining_rect,
        );
        subtract_rect(rect, drawn, Direction::Left)
    }

    fn draw_line(&mut self, line: [u8; 4], state: &State, rect: Rect) -> Rect {
        let mut drawn_rect = self.screen_manager.write_text("|", &self.text_font, rect);
        let mut remaining_rect = subtract_rect(rect, drawn_rect, Direction::Left);
        for key in line {
            let color = if state.keypad.is_pressed(key) {
                self.screen_manager.screen_config.colors.alt_color
            } else {
                self.screen_manager.screen_config.colors.on_color
            };
            let drawn = self.screen_manager.write_text_color(
                &format!("{:X}", key),
                &self.text_font,
                remaining_rect,
                color,
                self.screen_manager.screen_config.colors.off_color,
            );
            remaining_rect = subtract_rect(remaining_rect, drawn, Direction::Left);
            drawn_rect = add_rect(drawn, drawn_rect, Direction::Right);
        }
        self.screen_manager
            .write_text("|", &self.text_font, remaining_rect);
        drawn_rect
    }
}

fn write_index(state: &State) -> String {
    format!("I : {:#06X}", state.index)
}

fn draw_text(
    rendered_text: Surface,
    draw_rect: &mut Rect,
    texture_creator: &TextureCreator<WindowContext>,
    canvas: &mut Canvas<Window>,
) -> Rect {
    draw_rect.resize(rendered_text.width(), rendered_text.height());
    let mut texture = texture_creator
        .create_texture_from_surface(rendered_text)
        .unwrap();
    texture.set_scale_mode(ScaleMode::Nearest);
    canvas
        .copy(&texture, None, FRect::from(*draw_rect))
        .unwrap();
    *draw_rect
}

fn render_text<'a>(
    font: &Font,
    on_color: Color,
    off_color: Color,
    text: &str,
    width: i32,
) -> Surface<'a> {
    font.render(text)
        .lcd_wrapped(on_color, off_color, width)
        .unwrap()
}

/*
0 ----x---->
|
y
|
v
 */

fn subtract_rect(starting_rect: Rect, drawn_rect: Rect, direction: Direction) -> Rect {
    let x = match direction {
        Direction::Left => drawn_rect.right(),
        _ => starting_rect.left(),
    };
    let y = match direction {
        Direction::Up => drawn_rect.bottom(),
        _ => starting_rect.top(),
    };
    let width = match direction {
        Direction::Left | Direction::Right => starting_rect.width() - drawn_rect.width(),
        _ => starting_rect.width(),
    };
    let height = match direction {
        Direction::Up | Direction::Down => starting_rect.height() - drawn_rect.height(),
        _ => starting_rect.height(),
    };
    Rect::new(x, y, width, height)
}

fn add_rect(start: Rect, drawn: Rect, direction: Direction) -> Rect {
    let x = match direction {
        Direction::Left => start.left() - drawn.w,
        _ => start.left(),
    };
    let y = match direction {
        Direction::Up => start.top() - drawn.h,
        _ => start.top(),
    };
    let width = match direction {
        Direction::Left | Direction::Right => start.width() + drawn.width(),
        Direction::Up | Direction::Down => max(start.width(), drawn.width()),
    };
    let height = match direction {
        Direction::Up | Direction::Down => start.height() + drawn.height(),
        Direction::Left | Direction::Right => max(start.height(), drawn.height()),
    };
    Rect::new(x, y, width, height)
}

#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ScreenManager {
    fn write_text(&mut self, text: &str, font: &Font, dst: Rect) -> Rect {
        self.write_text_color(
            text,
            font,
            dst,
            self.screen_config.colors.on_color,
            self.screen_config.colors.off_color,
        )
    }
    fn write_text_color(
        &mut self,
        text: &str,
        font: &Font,
        dst: Rect,
        on_color: Color,
        off_color: Color,
    ) -> Rect {
        let mut draw_rect = self.apply_lateral_margin(&dst, true);
        let rendered_text = render_text(font, on_color, off_color, text, draw_rect.width() as i32);
        draw_text(
            rendered_text,
            &mut draw_rect,
            &self.texture_creator,
            &mut self.canvas,
        )
    }
    fn apply_lateral_margin(&self, rect: &Rect, scaled: bool) -> Rect {
        let size = if scaled {
            self.screen_config.scaled_margin()
        } else {
            self.screen_config.unscaled_margin()
        };
        Rect::new(
            rect.x() + size as i32,
            rect.y(),
            rect.width() - (2 * size as u32),
            rect.height(),
        )
    }
}

const REGISTERS_HEADER: &str = "REGISTERS";
const CONTROLS_HEADER: &str = "CONTROLS";
const INSTRUCTIONS_HEADER: &str = "INSTRUCTIONS";
const INDEX_HEADER: &str = "INDEX";
const STACK_HEADER: &str = "STACK";
const TIMER_HEADER: &str = "TIMERS";

enum PanelType {
    Instructions,
    Registers,
    Controls,
    Game,
    Index,
    Stack,
    Timer,
}

#[derive(Clone)]
struct Panel {
    boundaries: Rect,
    header: String,
}

impl Panel {
    fn new(panel_type: PanelType, x: i32, y: i32, width: u32, height: u32) -> Self {
        let header = match panel_type {
            PanelType::Instructions => INSTRUCTIONS_HEADER.to_string(),
            PanelType::Registers => REGISTERS_HEADER.to_string(),
            PanelType::Controls => CONTROLS_HEADER.to_string(),
            PanelType::Index => INDEX_HEADER.to_string(),
            PanelType::Stack => STACK_HEADER.to_string(),
            PanelType::Timer => TIMER_HEADER.to_string(),
            PanelType::Game => "".to_string(),
        };
        Self {
            boundaries: Rect::new(x, y, width, height),
            header,
        }
    }
}
