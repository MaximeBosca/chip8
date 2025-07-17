extern crate sdl3;

use std::fs;
use std::io::Read;
use crate::game_window::GameWindow;
use crate::state::State;

mod screen;
mod stack;
mod instruction;
mod game_window;
mod state;
mod interpreter;

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

fn main() {
    let mut state = State::new();
    load_font(&mut state, FONT);
    //let rom_path = "roms/2-ibm-logo.ch8";
    let rom_path = "roms/test_opcode.ch8";
    //let rom_path = "roms/bc_test.ch8";
    load_rom(&mut state, rom_path);
    let mut game_window = GameWindow::new();
    interpreter::game_loop(&mut state, &mut game_window);
}

fn load_font(state: &mut State, font: [[u8; 5]; 16]) {
    let mut index = FONT_ADDRESS as usize;
    for character in font.iter() {
        let end = index + character.len();
        state.ram[index..end].copy_from_slice(character);
        index = end;
    }
}

fn load_rom(state: &mut State, path: &str) {
    let mut f = fs::File::open(path).expect("File not found");
    f.read(&mut state.ram[0x200..]).expect("Error loading ROM into RAM");
    state.program_counter = 0x200u16;
}