use crate::keypad::Keypad;
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
    pub keypad: Keypad,
}

impl State {
    pub(crate) fn decrease_timers(&mut self) {
        // Fallback to zero rather than overflowing
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }
}

#[allow(dead_code)]
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
            keypad: Keypad::new(),
        }
    }
    pub fn register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    pub fn set_register(&mut self, index: usize, value: u8) {
        self.registers[index] = value;
    }

    pub fn vf(&self) -> u8 {
        self.registers[REGISTERS_SIZE - 1]
    }

    pub fn set_vf(&mut self, value: u8) {
        self.registers[REGISTERS_SIZE - 1] = value;
    }

    pub fn read_ram(&self, addr: usize) -> Result<u8, &str> {
        if addr >= self.ram.len() {
            return Err("Invalid address Pointer");
        }
        Ok(self.ram[addr])
    }

    pub fn read_ram_16(&self, addr: usize) -> Result<(u8, u8), &str> {
        Ok((self.read_ram(addr)?, self.read_ram(addr + 1)?))
    }

    pub fn register_numbers(&self) -> usize {
        self.registers.len()
    }
}
