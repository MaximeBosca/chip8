use crate::Config;
use crate::audio_player::AudioPlayer;
use crate::game_window::GameWindow;
use crate::interpreter::{Interpreter, InterpreterVariant};
use crate::screen_config::{MARGIN, PIXEL_MASKS, ScreenConfig};
use crate::state::State;
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Scancode;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use std::{fs, io::Read};
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
    [0xf0, 0x80, 0xf0, 0x80, 0x80], // F
];

const FONT_ADDRESS: u16 = 0x050;
pub const INTERPRETER_VARIANT: InterpreterVariant = InterpreterVariant::Chip48;

const TICK_INTERVAL: Duration = Duration::new(1 / 700, 0);
const TIMER_INTERVAL: Duration = Duration::new(1 / 60, 0);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExitStatus {
    Quit,
    Reset,
}

struct RunState {
    running: bool,
    step: bool,
}

fn load_rom(state: &mut State, path: PathBuf) {
    let mut f = fs::File::open(path).expect("File not found");
    let _ = f
        .read(&mut state.ram[0x200..])
        .expect("Error loading ROM into RAM");
    state.program_counter = 0x200u16;
}

pub struct Runner<'a> {
    audio_player: AudioPlayer,
    event_pump: EventPump,
    game_window: GameWindow<'a>,
    interpreter: Interpreter,
    next_timer_tick: Duration,
    run_state: RunState,
    state: State,
}

impl<'a> Runner<'a> {
    pub fn init(config: Config) -> Self {
        let screen_config = ScreenConfig::new(
            config.dimensions,
            config.screen_scale,
            MARGIN,
            PIXEL_MASKS,
            config.colors,
        );
        let sdl_context = sdl3::init().unwrap();
        let mut state = State::new(&screen_config);
        let font_address = FONT_ADDRESS;
        load_rom(&mut state, config.rom_path);
        load_font(&mut state, FONT, font_address);
        let game_window = GameWindow::new(&sdl_context, screen_config);
        let interpreter = Interpreter::new(config.ivariant, font_address);
        let audio_player = AudioPlayer::new(&sdl_context);
        let event_pump = sdl_context.event_pump().unwrap();
        let run_state = RunState {
            running: false,
            step: false,
        };
        Self {
            state,
            game_window,
            interpreter,
            event_pump,
            run_state,
            next_timer_tick: Duration::new(0, 0),
            audio_player,
        }
    }
    pub fn run(&mut self) {
        'running: loop {
            let start = SystemTime::now();
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::KeyDown {
                        scancode: Some(key),
                        ..
                    } => {
                        let result = handle_key_press(&mut self.run_state, key);
                        if let Some(status) = result {
                            match status {
                                ExitStatus::Quit => break 'running,
                                ExitStatus::Reset => continue 'running,
                            }
                        }
                        game_key_down(&mut self.state, key);
                    }
                    Event::KeyUp {
                        scancode: Some(key),
                        ..
                    } => {
                        game_key_up(&mut self.state, key);
                    }
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }
            let mut should_decrement = false;
            if self.run_state.should_continue() {
                self.interpreter.game_step(&mut self.state);
                should_decrement = true;
            }
            self.play_sound(should_decrement);
            self.game_window.update(&self.state);
            self.sleep(start, should_decrement);
        }
    }

    fn sleep(&mut self, start: SystemTime, should_decrement: bool) {
        let elapsed = start.elapsed().unwrap_or(Duration::new(0, 0));
        let to_sleep = TICK_INTERVAL
            .checked_sub(elapsed)
            .unwrap_or(Duration::new(0, 0));
        self.next_timer_tick = self
            .next_timer_tick
            .checked_sub(elapsed)
            .unwrap_or_else(|| self.decrease_timer(should_decrement));
        std::thread::sleep(to_sleep);
    }

    fn decrease_timer(&mut self, should_decrement: bool) -> Duration {
        if should_decrement {
            self.state.decrease_timers();
        }
        TIMER_INTERVAL
    }

    fn play_sound(&self, is_playing: bool) {
        if !is_playing || self.state.sound_timer == 0 {
            self.audio_player.pause_sound()
        } else {
            self.audio_player.play_sound()
        }
    }
}
fn handle_key_press(run_state: &mut RunState, scancode: Scancode) -> Option<ExitStatus> {
    match scancode {
        Scancode::F1 => {
            run_state.running = !run_state.running;
        }
        Scancode::F2 => {
            run_state.step = true;
        }
        Scancode::F3 => return Some(ExitStatus::Reset),
        Scancode::F4 => return Some(ExitStatus::Quit),
        _ => {}
    }
    None
}

fn game_key_down(state: &mut State, code: Scancode) {
    let result = game_key(code);
    if let Some(key) = result {
        state.keypad.press_key(key);
    }
}
fn game_key_up(state: &mut State, code: Scancode) {
    let result = game_key(code);
    if let Some(key) = result {
        state.keypad.release_key(key);
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

fn game_key(scancode: Scancode) -> Option<u8> {
    match scancode {
        Scancode::_1 => Some(0x01),
        Scancode::_2 => Some(0x02),
        Scancode::_3 => Some(0x03),
        Scancode::_4 => Some(0x0C),
        Scancode::Q => Some(0x04),
        Scancode::W => Some(0x05),
        Scancode::E => Some(0x06),
        Scancode::R => Some(0x0D),
        Scancode::A => Some(0x07),
        Scancode::S => Some(0x08),
        Scancode::D => Some(0x09),
        Scancode::F => Some(0x0E),
        Scancode::Z => Some(0x0A),
        Scancode::X => Some(0x00),
        Scancode::C => Some(0x0B),
        Scancode::V => Some(0x0F),
        _ => None,
    }
}

impl RunState {
    fn should_continue(&mut self) -> bool {
        if self.step {
            self.step = false;
            return true;
        }
        self.running
    }
}
