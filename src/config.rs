use crate::interpreter::InterpreterVariant;
use crate::screen_config::{Colors, Dimensions, ScreenConfig};
use std::path::PathBuf;

pub struct Config {
    pub screen_config: ScreenConfig,
    pub rom_path: PathBuf,
    pub instructions_per_frame: u8,
    pub interpreter_variant: InterpreterVariant,
}

impl Config {
    pub fn new(
        rom_path: PathBuf,
        interpreter_variant: InterpreterVariant,
        window_dimensions: Dimensions,
        colors: Colors,
        instructions_per_frame: u8,
    ) -> Self {
        Self {
            screen_config: ScreenConfig::new(window_dimensions, colors),
            rom_path,
            instructions_per_frame,
            interpreter_variant,
        }
    }
}
