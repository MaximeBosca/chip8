extern crate sdl3;

use crate::runner::{ExitStatus, Runner};
use crate::state::State;
use std::fs;
use std::io::Read;

mod audio_player;
mod game_window;
mod instruction;
mod interpreter;
mod keypad;
mod runner;
mod screen;
mod screen_config;
mod stack;
mod state;

fn main() {
    loop {
        // Note : Should change this and use some kind of file picker, but I'm not really feeling like it :(
        // change rom_path here to select the rom you want to use
        let rom_path = "roms/Pong.ch8";
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
    f.read(&mut state.ram[0x200..])
        .expect("Error loading ROM into RAM");
    state.program_counter = 0x200u16;
}
