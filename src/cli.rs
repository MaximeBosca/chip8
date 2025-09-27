use crate::config::Config;
use crate::interpreter::InterpreterVariant;
use crate::screen_config::{Colors, Dimensions};
use clap::Parser;
use clap::ValueEnum;
use clap::ValueHint;
use std::path::PathBuf;

// Test
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Path for the ROM to load
    #[arg(short='f', long, value_hint = ValueHint::FilePath, value_name = "FILE")]
    rom_path: PathBuf,

    /// Color of the foreground of the screen
    #[arg(value_enum, short='1', long, default_value_t=ColorValue::Green, value_name = "COLOR")]
    on_color: ColorValue,

    /// Color of the background of the screen
    #[arg(value_enum, short='0', long, default_value_t=ColorValue::Black, value_name = "COLOR")]
    off_color: ColorValue,

    /// Color of the highlighted text (like pressed keys)
    #[arg(value_enum, short, long, default_value_t=ColorValue::Red, value_name = "COLOR")]
    alt_color: ColorValue,

    /// Variant of the CHIP-8 interpreter
    #[arg(value_enum, short, long, default_value_t=InterpreterVariant::Chip48, value_name = "VARIANT")]
    interpreter_variant: InterpreterVariant,

    /// Screen resolution
    #[arg(value_enum, short, long, default_value_t=Resolution::FullHD, value_name = "VARIANT")]
    resolution: Resolution,
}

impl Cli {
    pub fn to_config(&self) -> Config {
        let (width, height) = self.resolution.to_window_dimensions();
        let dimensions = Dimensions::new(width, height);
        let colors = Colors::new(self.on_color, self.off_color, self.alt_color);
        Config::new(
            self.rom_path.clone(),
            self.interpreter_variant,
            dimensions,
            colors,
        )
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ColorValue {
    Green,
    Red,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
    Grey,
    Gray,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum Resolution {
    /// 800x600 (4:3)
    SuperVGA,
    /// 1280x720 (16:9)
    HD,
    /// 1280x960 (4:3)
    QuadVGA,
    /// 1400x1050 (4:3)
    SuperXGA,
    /// 1920x1080 (16:9)
    FullHD,
    /// 2560x1440 (16:9)
    QuadHD,
    /// 3440×1440 (21:9)
    UltraWideQHD,
}

impl Resolution {
    pub fn to_window_dimensions(&self) -> (usize, usize) {
        let (width, height): (usize, usize) = match self {
            Resolution::SuperVGA => (800, 600),
            Resolution::HD => (1280, 720),
            Resolution::QuadVGA => (1280, 960),
            Resolution::SuperXGA => (1400, 1050),
            Resolution::FullHD => (1920, 1080),
            Resolution::QuadHD => (2560, 1440),
            Resolution::UltraWideQHD => (3440, 1440),
        };
        (width, height)
    }
}
