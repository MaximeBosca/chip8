extern crate sdl3;

use sdl3::pixels::Color;

use crate::game_window::{HEADER_FONT_SIZE, TEXT_FONT_SIZE};
use crate::interpreter::InterpreterVariant;
use crate::runner::{INTERPRETER_VARIANT, Runner};
use crate::screen_config::{
    ALT_COLOR, OFF_COLOR, ON_COLOR, SCREEN_HEIGHT, SCREEN_SCALE, SCREEN_WIDTH, ScreenColors,
    ScreenDimensions,
};

use std::path::{Path, PathBuf};

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

macro_rules! print_help {
    () => {{
        let message = r#"
        Usage Commands:
            --alt_color, -a     Set the color of the alt state (red, green, ..)
            --header_size, -e   Set the header font size
            --height_screen, -y Set the height of the screen in pixels
            --help, -h          Show this message.
            --ivariant, -i      Specify the interpreter variant (cosmicvip, chip48)
            --off_color, -f     Set the color of the off state (red, green, ..)
            --on_color, -o      Set the color of the on state (red, green, ..)
            --rom_path, -r      Custom path to rom file (unix paths)
            --scale_screen, -s  Set the window scale in pixels (val > zero)
            --text_size, -t     Set the text font size
            --width_screen, -x  Set width of the screen in pixels
        "#;
        println!("{message}");
    }};
}

pub(crate) trait Parse {
    type Item;

    fn parse(args: impl Iterator<Item = String>) -> Result<Self::Item, String>;
}

/// Commandline Arguments
#[derive(Debug)]
struct Cli {
    /// Select a custom rom
    pub rom_path: Option<PathBuf>,
    /// Custom screen width
    pub width_screen: Option<usize>,
    /// Custom screen height
    pub height_screen: Option<usize>,
    /// Custom Screen Scale
    pub scale_screen: Option<usize>,
    /// Custom color for on state
    pub on_color: Option<String>,
    /// Custom color for alt state
    pub off_color: Option<String>,
    /// Custom color for off state
    pub alt_color: Option<String>,
    /// Custom InterpreterVariant for the runner
    pub ivariant: Option<InterpreterVariant>,
    /// Custom header font size
    pub header_size: Option<f32>,
    /// Custom text font size
    pub text_size: Option<f32>,
}

impl Parse for Cli {
    type Item = Cli;

    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self::Item, String> {
        //defaults
        let mut rom_path: Option<PathBuf> = None;
        let mut width_screen: Option<usize> = None;
        let mut height_screen: Option<usize> = None;
        let mut scale_screen: Option<usize> = None;
        let mut on_color: Option<String> = None;
        let mut off_color: Option<String> = None;
        let mut alt_color: Option<String> = None;
        let mut ivariant: Option<InterpreterVariant> = None;
        let mut header_size: Option<f32> = None;
        let mut text_size: Option<f32> = None;

        // Someone fix the short argument names :)
        while let Some(nxt_arg) = args.next() {
            match nxt_arg.to_lowercase().as_str() {
                "--help" | "-h" => {
                    print_help!();
                    std::process::exit(0);
                }

                "--rom_path" | "-r" => {
                    if let Some(p) = args.next() {
                        rom_path = Some(Path::new(&p).to_path_buf());
                    }
                }

                "--width_screen" | "-x" => {
                    if let Some(p) = args.next() {
                        let p = p
                            .parse::<usize>()
                            .map_err(|_err| "Invalid screen width value".to_string())?;
                        width_screen = Some(p);
                    }
                }

                "--height_screen" | "-y" => {
                    if let Some(p) = args.next() {
                        let p = p
                            .parse::<usize>()
                            .map_err(|_err| "Invalid screen height value".to_string())?;
                        height_screen = Some(p);
                    }
                }

                "--scale_screen" | "-s" => {
                    if let Some(p) = args.next() {
                        let p = p
                            .parse::<usize>()
                            .map_err(|_err| "Invalid screen scale value".to_string())?;
                        scale_screen = Some(p);
                    }
                }

                "--on_color" | "-o" => {
                    on_color = args.next();
                }

                "--off_color" | "-f" => {
                    off_color = args.next();
                }

                "--alt_color" | "-a" => {
                    alt_color = args.next();
                }

                "--ivariant" | "-i" => {
                    ivariant = args.next().and_then(|s| match s.to_lowercase().as_str() {
                        "cosmacvip" => Some(InterpreterVariant::CosmacVip),
                        "chip48" => Some(InterpreterVariant::Chip48),
                        _ => None,
                    })
                }

                "--header_size" | "-e" => {
                    if let Some(p) = args.next() {
                        let p = p
                            .parse::<f32>()
                            .map_err(|_err| format!("Invalid header font size value: {p}"))?;
                        header_size = Some(p);
                    }
                }

                "--text_size" | "-t" => {
                    if let Some(p) = args.next() {
                        let p = p
                            .parse::<f32>()
                            .map_err(|_err| format!("Invalid text font size value: {p}"))?;
                        text_size = Some(p);
                    }
                }
                _ => {}
            }
        }

        Ok(Cli {
            rom_path,
            height_screen,
            width_screen,
            scale_screen,
            on_color,
            off_color,
            alt_color,
            ivariant,
            header_size,
            text_size,
        })
    }
}

/// Holds the game window font sizes
#[derive(Debug)]
pub struct FontSize {
    pub text: f32,
    pub header: f32,
}

impl FontSize {
    fn new(header: f32, text: f32) -> Self {
        Self { header, text }
    }
}

#[derive(Debug)]
pub struct Config {
    pub rom_path: PathBuf,
    pub dimensions: ScreenDimensions,
    pub screen_scale: usize,
    pub colors: ScreenColors,
    pub ivariant: InterpreterVariant,
    pub font_size: FontSize,
}

fn get_color_code(name: String) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "green" => Some(Color::GREEN),
        "red" => Some(Color::RED),
        "black" => Some(Color::BLACK),
        "gray" => Some(Color::GRAY),
        "white" => Some(Color::WHITE),
        "grey" => Some(Color::GREY),
        "blue" => Some(Color::BLUE),
        "magenta" => Some(Color::MAGENTA),
        "yellow" => Some(Color::YELLOW),
        "cyan" => Some(Color::CYAN),
        _ => None,
    }
}

fn load_config() -> Result<Config, String> {
    let args = Cli::parse(std::env::args().skip(1))?;
    let rom_path = args
        .rom_path
        .unwrap_or(Path::new("roms/2-ibm-logo.ch8").to_path_buf());

    let screen_height = args.height_screen.unwrap_or(SCREEN_HEIGHT);
    let screen_width = args.width_screen.unwrap_or(SCREEN_WIDTH);
    let screen_scale = args.scale_screen.unwrap_or(SCREEN_SCALE);
    let on_color = args.on_color.and_then(get_color_code).unwrap_or(ON_COLOR);
    let off_color = args.off_color.and_then(get_color_code).unwrap_or(OFF_COLOR);
    let alt_color = args.alt_color.and_then(get_color_code).unwrap_or(ALT_COLOR);
    let colors = ScreenColors::new(on_color, off_color, alt_color);

    let ivariant = args.ivariant.unwrap_or(INTERPRETER_VARIANT);

    let header_size = args.header_size.unwrap_or(HEADER_FONT_SIZE);
    let text_size = args.text_size.unwrap_or(TEXT_FONT_SIZE);

    Ok(Config {
        rom_path,
        dimensions: ScreenDimensions::new(screen_width, screen_height),
        screen_scale,
        colors,
        ivariant,
        font_size: FontSize::new(header_size, text_size),
    })
}

fn main() -> Result<(), String> {
    let mut runner = Runner::init(load_config()?);

    runner.run();
    Ok(())
}
