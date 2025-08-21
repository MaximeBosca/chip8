extern crate sdl3;

use std::fs;
use std::io::Read;
use crate::runner::{ExitStatus, Runner};
use crate::state::State;

mod screen;
mod stack;
mod instruction;
mod game_window;
mod state;
mod interpreter;
mod screen_config;
mod runner;
mod keypad;

fn main() {
    loop {
        //let rom_path = "roms/2-ibm-logo.ch8";
        //let rom_path = "roms/test_opcode.ch8";
        //let rom_path = "roms/bc_test.ch8";
        let rom_path = "roms/Pong.ch8";
        //let rom_path = "roms/Pong (1 player).ch8";
        let mut runner = Runner::init(rom_path);
        let status = runner.run();
        match status {
            ExitStatus::Quit => break,
            ExitStatus::Reset => continue,
            ExitStatus::Error(message) => println!("Error running the emulator : {}", message),
        }
    }
}

fn load_rom(state: &mut State, path: &str) {
    let mut f = fs::File::open(path).expect("File not found");
    f.read(&mut state.ram[0x200..]).expect("Error loading ROM into RAM");
    state.program_counter = 0x200u16;
}