use crate::instruction::{Instruction, Operator};
use crate::state::State;
use std::fmt::{Arguments, format};

pub fn write_instructions(state: &State) -> String {
    (0usize..32usize)
        .map(|index| state.program_counter as usize + 2 * index)
        .filter_map(|addr| read_printable_instruction(state, addr))
        .map(|instruction| instruction.print(state))
        .reduce(|a, b| format!("{}\n{}", a, b))
        .unwrap()
}

fn read_printable_instruction(state: &State, addr: usize) -> Option<PrintableInstruction> {
    let bytes = state.read_ram_16(addr).ok()?;
    Some(PrintableInstruction::new(addr, bytes))
}

pub struct PrintableInstruction {
    address: usize,
    instruction: Instruction,
}

impl PrintableInstruction {
    pub fn print(&self, state: &State) -> String {
        format(format_args!(
            "{:#03X}: {}",
            self.address,
            print_instruction(&self.instruction)
        ))
    }

    pub fn new(address: usize, bytes: (u8, u8)) -> Self {
        let instruction = Instruction::from(bytes);
        Self {
            address,
            instruction,
        }
    }
}

/// A method to print out instructions in a clear way
///
/// This method prints out the `Instruction` as a 12 (maximum) character-long `String`
/// for representation on the debug screen. If the `Instruction` has parameters, they will be
/// formatted depending on their type.
pub fn print_instruction(instruction: &Instruction) -> String {
    match instruction {
        Instruction::System => "SYS".to_string(),
        Instruction::ClearScreen => "CLR".to_string(),
        Instruction::Jump(address) => format!("JUMP {:#03X}", address),
        Instruction::SubroutineCall(address) => format!("CALL {:#03X}", address),
        Instruction::SubroutineReturn => "RET".to_string(),
        Instruction::SkipEqualValue(x, value, skip_equal) => {
            print_conditional_skip(x, format_args!("{}", value), *skip_equal)
        }
        Instruction::SkipEqualRegister(x, y, skip_equal) => {
            print_conditional_skip(x, format_args!("V{:X}", y), *skip_equal)
        }
        Instruction::SetRegister(x, value) => format!("SET V{:X} {}", x, value),
        Instruction::AddValueRegister(x, value) => format!("ADD V{:X} {}", x, value),
        Instruction::Arithmetic(x, y, operator) => print_arithmetic(x, y, operator),
        Instruction::SetIndex(value) => format!("SETI {}", value),
        Instruction::JumpOffset(x, value) => format!("JMPO V{:X} {}", x, value),
        Instruction::Random(x, value) => format!("RND V{:X} {}", x, value),
        Instruction::Draw(x, y, count) => format!("DRW V{:X} V{:X} {}", x, y, count),
        Instruction::SkipIfKey(x, pressed) => {
            let cmd = if *pressed { "SKPKEY" } else { "SKPNKEY" };
            format!("{} V{:X}", cmd, x)
        }
        Instruction::SetDelayTimer(x) => format!("WDLY V{:X}", x),
        Instruction::ReadDelayTimer(x) => format!("RDLY V{:X}", x),
        Instruction::SetSoundTimer(x) => format!("WSND V{:X}", x),
        Instruction::AddToIndex(x) => format!("ADDI V{:X}", x),
        Instruction::GetKey(x) => format!("KEY V{:X}", x),
        Instruction::FontCharacter(x) => format!("FONT V{:X}", x),
        Instruction::DecimalConversion(x) => format!("CONV V{:X}", x),
        Instruction::LoadRegisters(x) => format!("LOAD V{:X}", x),
        Instruction::StoreRegisters(x) => format!("STORE V{:X}", x),
        Instruction::Unknown(_, _) => "UNKNOWN".to_string(),
    }
}

fn print_arithmetic(x: &usize, y: &usize, operator: &Operator) -> String {
    let cmd = match operator {
        Operator::Set => "SET",
        Operator::BinaryOr => "OR",
        Operator::BinaryAnd => "AND",
        Operator::BinaryXor => "XOR",
        Operator::Add => "ADD",
        Operator::Subtract => "SUB",
        Operator::SubtractInverse => "SUBI",
        Operator::ShiftL => "SHL",
        Operator::ShiftR => "SHR",
        Operator::Unknown(_) => "UKN",
    };
    format!("{} V{:X} V{:X}", cmd, x, y)
}

fn print_conditional_skip(x: &usize, v2: Arguments, skip_equal: bool) -> String {
    let cmd = if skip_equal { "SKEQ" } else { "SKNE" };
    format!("{} V{:X} {}", cmd, x, v2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_instruction() {
        assert_eq!(
            print_instruction(&Instruction::SkipEqualValue(15, 250, true)),
            "SKEQ VF 250"
        );
        assert_eq!(
            print_instruction(&Instruction::SkipEqualValue(8, 120, false)),
            "SKNE V8 120"
        );
        assert_eq!(
            print_instruction(&Instruction::SkipEqualRegister(4, 7, true)),
            "SKEQ V4 V7"
        );
        assert_eq!(
            print_instruction(&Instruction::SkipEqualRegister(10, 12, false)),
            "SKNE VA VC"
        );
    }
}
