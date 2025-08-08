use rand::Rng;
use crate::instruction::{Instruction, Operator};
use crate::state::State;

pub enum InterpreterVariant {
    CosmacVip,
    Chip48,
}
pub struct Interpreter {
    variant: InterpreterVariant,
    font_address: u16,
}
impl Interpreter {
    pub fn new(variant: InterpreterVariant, font_address: u16) -> Self {
        Self {
            variant,
            font_address,
        }
    }
    pub fn game_step(&self, state: &mut State) {
        let (byte1, byte2) = fetch(state);
        let instruction = decode(byte1, byte2);
        self.execute(instruction, state);
    }
    fn execute(&self, instruction: Instruction, state: &mut State) {
        match instruction {
            Instruction::ClearScreen => state.screen.clear(),
            Instruction::Jump(address) => state.program_counter = address,
            Instruction::SubroutineCall(address) => {
                state.stack.push(state.program_counter);
                state.program_counter = address
            },
            Instruction::SubroutineReturn => state.program_counter = state.stack.pop(),
            Instruction::SkipEqualValue(rx, value, skip_condition) =>
                skip_compare(state,
                             state.register(rx),
                             value,
                             skip_condition), // 0x3 0x4
            Instruction::SkipEqualRegister(rx, ry, skip_condition) =>
                skip_compare(state,
                             state.register(rx),
                             state.register(ry),
                             skip_condition), // 0x5 0x9
            Instruction::SetRegister(rx, value) => state.set_register(rx, value), // 0x6
            Instruction::AddValueRegister(rx, value) =>
                state.set_register(rx, value.wrapping_add(state.register(rx))), // 0x7
            Instruction::Arithmetic(rx, ry, arithmetic_operator) =>
                self.arithmetic_operation(state, rx, ry, arithmetic_operator),
            Instruction::SetIndex(value) => state.index = value,
            Instruction::JumpOffset(register_x, offset) => self.jump_offset(state, register_x, offset),
            Instruction::Random(rx, value) => state.set_register(rx,{
                let random: u8 = rand::rng().random();
                random & value
            }),
            Instruction::Draw(rx, ry, sprite_height) =>
                draw(state, rx, ry, sprite_height),
            Instruction::SkipIfKey(rx, if_pressed) => panic!("deal with inputs !"),
            Instruction::ReadDelayTimer(rx) => state.set_register(rx, state.delay_timer),
            Instruction::SetDelayTimer(rx) => state.delay_timer = state.register(rx),
            Instruction::SetSoundTimer(rx) => state.sound_timer = state.register(rx),
            Instruction::AddToIndex(rx) => {
                let under = state.index <= 0xFFF;
                state.index = state.index.wrapping_add(state.register(rx) as u16);
                if under && state.index > 0xFFF {
                    state.set_vf(1);
                }
            }
            Instruction::GetKey(rx) => panic!("deal with inputs !"),
            Instruction::FontCharacter(rx) =>
                state.index = self.font_address + (state.register(rx) & 0x0F) as u16 * 5,
            Instruction::DecimalConversion(rx) => decimal_conversion(state, rx),
            Instruction::StoreRegisters(rx) => self.memory_copy(state, rx, true),
            Instruction::LoadRegisters(rx) => self.memory_copy(state, rx, false),
            Instruction::System => (), // pass
            Instruction::Unknown(opcode, value) => panic!("Unkown instruction {:#X} with value {}", opcode, value),
        }
    }
    fn memory_copy(&self, state: &mut State, rx: usize, store: bool) {
        let mut working_index = state.index;
        for i in 0..rx+1 {
            if store {
                state.ram[working_index as usize] = state.register(i);
            } else {
                state.set_register(i,state.ram[working_index as usize]);
            }
            state.index = match self.variant {
                InterpreterVariant::CosmacVip => working_index,
                InterpreterVariant::Chip48 => state.index,
            };
            working_index+=1;
        }
    }

    fn jump_offset(&self, state: &mut State, rx: usize, offset: u16) {
        let address = match self.variant {
            InterpreterVariant::CosmacVip => state.register(0) as u16 + offset,
            InterpreterVariant::Chip48 => state.register(rx) as u16 + offset,
        };
        state.program_counter = address;
    }
    fn arithmetic_operation(&self, state: &mut State, rx: usize, ry: usize, operator: Operator) {
        let x = state.register(rx);
        let y = state.register(ry);
        match operator {
            Operator::Set => state.set_register(rx, y),
            Operator::BinaryOr => state.set_register(rx,x | y),
            Operator::BinaryAnd => state.set_register(rx, x & y),
            Operator::BinaryXor => state.set_register(rx, x ^ y),
            _ => {
                let (res, overflow) = match operator {
                    Operator::Add => {
                        let (res, overflow) = x.overflowing_add(y);
                        (res, overflow as u8)
                    },
                    Operator::Subtract => subtract(x, y),
                    Operator::SubtractInverse => subtract(y, x),
                    Operator::ShiftR => shift(&self.variant, x, y, false),
                    Operator::ShiftL => shift(&self.variant, x, y, true),
                    Operator::Unknown(operator) => panic!("Unknown operator {:#X}", operator),
                    _ => unimplemented!("Unimplemented operator {:?}", operator),
                };
                state.set_register(rx, res);
                state.set_vf(overflow);
            }
        }
    }
}

fn fetch(state: &mut State) -> (u8, u8) {
    let pc = state.program_counter as usize;
    state.program_counter += 2;
    state.read_ram_16(pc).unwrap()
}

fn decode(byte1: u8, byte2: u8) -> Instruction {
    Instruction::from((byte1, byte2))
}

fn decimal_conversion(state: &mut State, rx: usize) {
    let base_address = state.index as usize;
    let x = state.register(rx);
    state.ram[base_address] = x / 100;
    state.ram[base_address + 1] = (x % 100) / 10;
    state.ram[base_address + 2] = (x % 100) % 10;
}

fn shift(variant: &InterpreterVariant, x: u8, y: u8, left: bool) -> (u8, u8) {
    let value = match variant {
        InterpreterVariant::CosmacVip => y,
        InterpreterVariant::Chip48 => x,
    };
    if left {
        let overflow = (value & 0x80 != 0) as u8;
        (value << 1, overflow)
    } else {
        let overflow = (value & 0x01 != 0) as u8;
        (value >> 1, overflow)
    }
}

fn subtract(x: u8, y: u8) -> (u8, u8) {
    if x >= y {
        return (x - y, 1)
    }
    (y - x, 0)
}

fn draw(state: &mut State, rx: usize, ry: usize, sprite_height: u8) {
    let x = state.register(rx) as usize % state.screen.width;
    let y = state.register(ry) as usize % state.screen.height;
    let begin = state.index as usize;
    let end = begin + sprite_height as usize;
    let overflow = state.screen.draw_sprite(x, y, &state.ram[begin..end]) as u8;
    state.set_vf(overflow);
}

fn skip_compare(state: &mut State, value1: u8, value2: u8, skip_condition: bool) {
    if (value1 == value2) == skip_condition {
        state.program_counter += 2;
    }
}