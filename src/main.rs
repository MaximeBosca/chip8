extern crate sdl3;

use std::fs;
use std::io::Read;
use std::time::Duration;
use crate::instruction::Instruction;
use crate::screen::Screen;
use crate::stack::Stack;

mod screen;
mod stack;
mod instruction;

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

const FONT_ADDRESS: usize = 0x050;
fn main() {
    let mut state = State::new();
    load_font(&mut state, FONT, FONT_ADDRESS);
    let rom_path = "roms/2-ibm-logo.ch8";
    load_rom(&mut state, rom_path);
    game_loop(&mut state);
}

fn load_font(state: &mut State, font: [[u8; 5]; 16], font_address: usize) {
    let mut index = font_address;
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

fn game_loop(state: &mut State) {
    'running: loop {
        let (byte1, byte2) = fetch(state);
        let instruction = decode(byte1, byte2);
        execute(instruction, state);
        ::std::thread::sleep(Duration::new(0, 100_000_000));
    }
}

fn fetch(state: &mut State) -> (u8, u8) {
    let pc = state.program_counter as usize;
    if pc >= state.ram.len() {
        panic!("Program counter overflowed ram")
    }
    state.program_counter += 2;
    (state.ram[pc], state.ram[pc + 1])
}

fn decode(byte1: u8, byte2: u8) -> Instruction {
    Instruction::from((byte1, byte2))
}

fn execute(instruction: Instruction, state: &mut State) {
    match instruction {
        Instruction::ClearScreen => state.screen.clear(),
        Instruction::Jump(address) => state.program_counter = address,
        Instruction::SetRegister(index, value) => state.registers[index] = value,
        Instruction::AddValueRegister(index, value) => state.registers[index] = u8::wrapping_add(state.registers[index], value),
        Instruction::SetIndex(value) => state.index = value,
        Instruction::Draw(register_x, register_y, sprite_height) => draw(state, register_x, register_y, sprite_height),
        Instruction::Unknown(opcode, value) => panic!("Unkown instruction {:#X} with value {}", opcode, value)
    }
}

fn draw(state: &mut State, register_x: usize, register_y: usize, sprite_height: u8) {
    let x = state.registers[register_x] as usize % state.screen.width();
    let y = state.registers[register_y] as usize % state.screen.height();
    let begin = state.index as usize;
    let end = begin + sprite_height as usize;
    state.registers[15] = state.screen.draw_sprite(x, y, &state.ram[begin..end]) as u8;
}
