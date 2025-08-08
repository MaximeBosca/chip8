use std::time::Duration;
use sdl3::Sdl;
use crate::game_window::GameWindow;
use crate::{interpreter, load_rom};
use crate::interpreter::{Interpreter, InterpreterVariant};
use crate::screen_config::ScreenConfig;
use crate::state::State;

const FONT: [[u8; 5]; 16] = [
    [0xf0, 0x90, 0x90, 0x90, 0xf0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xf0, 0x10, 0xf0, 0x80, 0xf0], // 2
    [0xf0, 0x10, 0xf0, 0x10, 0xf0], // 3
    [0x90, 0x90, 0xf0, 0x10, 0x10], // 4
    [0xf0, 0x80, 0xf0, 0x10, 0xf0], // 5
    [0xf0, 0x80, 0xf0, 0x90, 0xf0], // 6
    [0xf0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xf0, 0x90, 0xf0, 0x90, 0xf0], // 8
    [0xf0, 0x90, 0xf0, 0x10, 0xf0], // 9
    [0xf0, 0x90, 0xf0, 0x90, 0x90], // A
    [0xe0, 0x90, 0xe0, 0x90, 0xe0], // B
    [0xf0, 0x80, 0x80, 0x80, 0xf0], // C
    [0xe0, 0x90, 0x90, 0x90, 0xe0], // D
    [0xf0, 0x80, 0xf0, 0x80, 0xf0], // E
    [0xf0, 0x80, 0xf0, 0x80, 0x80]  // F
];

const FONT_ADDRESS: u16 = 0x050;
const INTERPRETER_VARIANT: InterpreterVariant = InterpreterVariant::CosmacVip;


pub struct Runner<'a> {
    sdl_context: Sdl,
    state: State,
    game_window: GameWindow<'a>,
    interpreter: Interpreter,
}

impl<'a> Runner<'a> {
    pub fn init(rom_path : &str) -> Self {
        let screen_config = ScreenConfig::default();
        let sdl_context =  sdl3::init().unwrap();
        let mut state = State::new(&screen_config);
        let font_address = FONT_ADDRESS;
        load_rom(&mut state, rom_path);
        load_font(&mut state, FONT, font_address);
        let game_window = GameWindow::new(&sdl_context, screen_config);
        let interpreter = Interpreter::new(INTERPRETER_VARIANT, font_address);
        Self {
            sdl_context,
            state,
            game_window,
            interpreter
        }
    }
    pub fn run(&mut self) {
        'running: loop {
            self.interpreter.game_step(&mut self.state);
            self.game_window.update(&self.state);
            std::thread::sleep(Duration::new(0, 100_000_000));
        }
    }
}

fn load_font(state: &mut State, font: [[u8; 5]; 16], font_addr: u16) {
    let mut index = font_addr as usize;
    for character in font.iter() {
        let end = index + character.len();
        state.ram[index..end].copy_from_slice(character);
        index = end;
    }
}