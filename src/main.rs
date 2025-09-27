extern crate sdl3;

use crate::cli::Cli;
use crate::runner::Runner;
use clap::Parser;

mod audio_player;
mod cli;
mod config;
mod game_window;
mod instruction;
mod interpreter;
mod keypad;
mod runner;
mod screen;
mod screen_config;
mod stack;
mod state;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let mut runner = Runner::init(cli.to_config());
    runner.run();
    Ok(())
}
