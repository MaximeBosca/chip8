extern crate sdl3;

use std::time::Duration;
use crate::screen::{Screen, COLOR_BLACK, COLOR_GREEN, COLOR_WHITE};
use crate::stack::Stack;

mod screen;
mod stack;

struct State {
    ram: [u8; 4096],
    stack: Stack,
    program_counter: u16,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    screen: Screen,
}
impl State {
    fn new() -> Self {
        Self {
            ram: [0; 4096],
            stack: Stack::new(),
            program_counter: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            screen: Screen::new(),
        }
    }
}
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
fn main() {
    let mut state = State::new();
    game_loop(&mut state);
}

fn game_loop(state: &mut State) {
    let mut line = 0;
    let mut column = 0;
    state.screen.on_color(COLOR_GREEN);
    state.screen.clear();
    for character in FONT.iter() {
        for (i,byte) in character.iter().enumerate() {
            state.screen.draw_byte(byte, column, line + i);
        }
        column += 9;
        if column >= state.screen.width() - 9 {
            column = 0;
            line += 6;
        }
    }
    //state.screen.flip_pixel(1,1);
    state.screen.update();
    'running: loop {
        ::std::thread::sleep(Duration::new(0, 100_000_000));
    }
}
