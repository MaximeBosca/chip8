use crate::screen::Screen;
use crate::screen_config::ScreenConfig;
use crate::stack::Stack;

const REGISTERS_SIZE: usize = 16;
const RAM_SIZE: usize = 4096;
pub struct State {
    pub ram: [u8; RAM_SIZE],
    pub stack: Stack,
    pub program_counter: u16,
    pub index: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    registers: [u8; REGISTERS_SIZE],
    pub screen: Screen,
}
impl State {
    pub fn new(screen_config: &ScreenConfig) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            stack: Stack::new(),
            program_counter: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; REGISTERS_SIZE],
            screen: Screen::new(screen_config),
        }
    }
    pub fn register(self: &Self, index :usize) -> u8 {
        self.registers[index]
    }

    pub fn set_register(self: &mut Self, index: usize, value: u8) {
        self.registers[index] = value;
    }

    pub fn vf(self: &Self) -> u8 {
        self.registers[REGISTERS_SIZE - 1]
    }

    pub fn set_vf(self: &mut Self, value: u8) {
        self.registers[REGISTERS_SIZE - 1] = value;
    }
}