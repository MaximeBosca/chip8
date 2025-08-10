use std::result;
use std::time::Duration;
use sdl3::{EventPump, Sdl};
use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Scancode};
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
const INTERPRETER_VARIANT: InterpreterVariant = InterpreterVariant::Chip48;

pub enum ExitStatus {
    Quit,
    Reset,
    Error(String),
}

struct RunState {
    running: bool,
    step: bool,
}

pub struct Runner<'a> {
    state: State,
    game_window: GameWindow<'a>,
    interpreter: Interpreter,
    event_pump: EventPump,
    run_state: RunState,
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
        let event_pump = sdl_context.event_pump().unwrap();
        let run_state = RunState { running: false, step: false };
        Self {
            state,
            game_window,
            interpreter,
            event_pump,
            run_state,
        }
    }
    pub fn run(&mut self) -> ExitStatus {
        'running: loop {
            let mut pressed_keys : Vec<u8> = Vec::new();
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::KeyDown{scancode: Some(key),..} => {
                        let result = handle_key_press(key, &mut self.run_state, &mut pressed_keys);
                        if let Some(status) = result {
                            return status;
                        }
                    },
                    Event::Quit { .. } => {return ExitStatus::Quit},
                    _ => {},
                }
            }

            if self.run_state.should_continue() {
                self.interpreter.game_step(&mut self.state, &pressed_keys);
            }
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

fn handle_key_press(scancode: Scancode, run_state: &mut RunState, pressed_keys: &mut Vec<u8>)
    -> Option<ExitStatus> {
    match scancode {
        Scancode::F1 => {
            run_state.running = !run_state.running;
        }
        Scancode::F2 => {
            run_state.step = true;
        }
        Scancode::F3 => {
            return Some(ExitStatus::Reset)
        }
        Scancode::F4 => {
            return Some(ExitStatus::Quit)
        }
        _ => {
            let key = game_key(scancode);
            if let Some(u8_key) = key {
                pressed_keys.push(u8_key);
            }
        }
    }
    None
}

fn game_key(scancode: Scancode) -> Option<u8> {
    match scancode {
        Scancode::_1 => Some(0x01),
        Scancode::_2 => Some(0x02),
        Scancode::_3 => Some(0x03),
        Scancode::_4 => Some(0x0C),
        Scancode::Q  => Some(0x04),
        Scancode::W  => Some(0x05),
        Scancode::E  => Some(0x06),
        Scancode::R  => Some(0x0D),
        Scancode::A  => Some(0x07),
        Scancode::S  => Some(0x08),
        Scancode::D  => Some(0x09),
        Scancode::F  => Some(0x0E),
        Scancode::Z  => Some(0x0A),
        Scancode::X  => Some(0x00),
        Scancode::C  => Some(0x0B),
        Scancode::V  => Some(0x0F),
        _ => None,
    }
}

impl RunState {
    fn should_continue(&mut self) -> bool {
        if self.step {
            self.step = false;
            return true
        }
        self.running
    }
}